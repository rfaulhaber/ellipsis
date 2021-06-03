use crate::config::{
    Config, ConfigHost, LinkDefinition, LinkKind, LiteralTaskDefinition, TaskDefinition,
};
use crate::opts::{ExecCmd, InstallCmd, LinkCmd, SubCommand};
use anyhow::Result;
use std::io::{self, Write};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Could not get hostname")]
    NoHostname,
    #[error("No config found for hostname {0}")]
    NoConfigFound(String),
    #[error("Could not find task named {0}")]
    NoTaskFound(String),
    #[error("Task {0} failed with {1}")]
    RunFailure(String, String),
    #[error("Could not retrieve metadata for file {0}")]
    MetadataFailure(String),
}

pub struct Runner {
    hostname: String,
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Result<Self> {
        let name = match hostname::get()?.into_string() {
            Ok(name) => name,
            Err(_) => return Err(ExecError::NoHostname.into()),
        };

        Ok(Runner {
            hostname: name,
            config,
        })
    }

    pub fn execute_cmd(&self, cmd: SubCommand) -> Result<()> {
        match cmd {
            SubCommand::Install(c) => self.execute_install_cmd(c),
            SubCommand::Link(c) => self.execute_link_cmd(c),
            SubCommand::Exec(c) => self.execute_exec_cmd(c),
        }
    }

    fn execute_install_cmd(&self, cmd: InstallCmd) -> Result<()> {
        todo!();
    }

    fn execute_link_cmd(&self, cmd: LinkCmd) -> Result<()> {
        let host_config = self.get_config_host()?;
        if cmd.args.is_empty() {
            for link in host_config.links.clone() {
                make_link(link)?;
            }
        } else {
            let links: Vec<LinkDefinition> = host_config
                .links
                .iter()
                .filter_map(|link| match link.name.clone() {
                    Some(name) if cmd.args.contains(&name) => Some(link.clone()),
                    Some(_) | None => None,
                })
                .collect();

            for link in links {
                make_link(link)?
            }
        }

        Ok(())
    }

    fn execute_exec_cmd(&self, cmd: ExecCmd) -> Result<()> {
        let host_config = self.get_config_host()?;
        let task = self.find_host_or_global_config_task(host_config, cmd.arg.clone());

        match task {
            Some(t) => self.execute_task(t),
            None => Err(ExecError::NoTaskFound(cmd.arg).into()),
        }
    }

    fn get_config_host(&self) -> Result<&ConfigHost> {
        match self.config.hosts.get(&self.hostname) {
            Some(c) => Ok(c),
            None => Err(ExecError::NoConfigFound(self.hostname.clone()).into()),
        }
    }

    fn find_host_config_task(
        &self,
        config_host: &ConfigHost,
        task_name: String,
    ) -> Option<LiteralTaskDefinition> {
        config_host
            .tasks
            .iter()
            .find(|task| match task {
                TaskDefinition::Ref(_) => false,
                TaskDefinition::Literal(task_def) => match &task_def.name {
                    Some(name) => *name == task_name,
                    None => false,
                },
            })
            .and_then(|task| match task {
                TaskDefinition::Ref(_) => unreachable!(),
                TaskDefinition::Literal(lit) => Some(lit.clone()),
            })
    }

    fn find_host_or_global_config_task(
        &self,
        config_host: &ConfigHost,
        task_name: String,
    ) -> Option<LiteralTaskDefinition> {
        self.find_host_config_task(config_host, task_name.clone())
            .or_else(|| match self.config.tasks.get(&task_name) {
                Some(t) => Some(t.clone()),
                None => None,
            })
    }

    fn execute_task(&self, task: LiteralTaskDefinition) -> Result<()> {
        let exec = self.config.resolve_variable_string(task.exec)?;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", exec.as_str()]).output()?
        } else {
            Command::new("sh").arg("-c").arg(exec.as_str()).output()?
        };

        if !output.status.success() {
            return Err(ExecError::RunFailure(exec, output.status.to_string()).into());
        }

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Ok(())
    }
}

#[cfg(target_family = "unix")]
fn make_link(link: LinkDefinition) -> Result<()> {
    use std::fs::{copy, hard_link};
    use std::os::unix::fs::symlink;

    let LinkDefinition { from, to, kind, .. } = link;

    match kind {
        Some(LinkKind::Hard) => hard_link(from, to)?,
        Some(LinkKind::Soft) | None => symlink(from, to)?,
        Some(LinkKind::Copy) => copy(from, to).and(Ok(()))?,
    };

    Ok(())
}

#[cfg(target_family = "windows")]
fn make_link(link: LinkDefinition) -> Result<()> {
    use std::fs::metadata;
    use std::fs::{copy, hard_link};
    use std::os::windows::fs::{symlink_dir, symlink_file};

    let LinkDefinition { from, to, kind, .. } = link;

    match kind {
        Some(LinkKind::Hard) => hard_link(from, to)?,
        Some(LinkKind::Soft) | None => match metadata(from) {
            Some(data) => {
                if data.is_dir() {
                    symlink_dir(from, to)?
                } else {
                    symlink_file(from, to)?
                }
            }
            None => return Err(ExecError::MetadataFailure(from).into()),
        },
        Some(LinkKind::Copy) => copy(from, to).and(Ok(()))?,
    };

    Ok(())
}
