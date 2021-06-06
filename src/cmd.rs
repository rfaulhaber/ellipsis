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
    config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        Runner { config }
    }

    pub fn execute_cmd(&self, cmd: SubCommand) -> Result<()> {
        match cmd {
            SubCommand::Install(c) => self.execute_install_cmd(c),
            SubCommand::Link(c) => self.execute_link_cmd(c),
            SubCommand::Exec(c) => self.execute_exec_cmd(c),
        }
    }

    fn execute_install_cmd(&self, cmd: InstallCmd) -> Result<()> {
        let hostname = cmd.hostname.clone();
        for task in self.resolve_all_tasks_for_host(hostname.clone())? {
            self.execute_task(task)?;
        }

        let host_config = self.get_config_host(hostname)?;

        for link in host_config.links.clone() {
            self.make_link(link)?;
        }

        Ok(())
    }

    fn execute_link_cmd(&self, cmd: LinkCmd) -> Result<()> {
        let host_config = self.get_config_host(cmd.hostname.clone())?;
        if cmd.args.is_empty() {
            for link in host_config.links.clone() {
                self.make_link(link)?;
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
                self.make_link(link)?
            }
        }

        Ok(())
    }

    fn execute_exec_cmd(&self, cmd: ExecCmd) -> Result<()> {
        let host_config = self.get_config_host(cmd.hostname)?;
        let task = self.find_host_or_global_config_task(host_config, cmd.task_name.clone());

        match task {
            Some(t) => self.execute_task(t),
            None => Err(ExecError::NoTaskFound(cmd.task_name).into()),
        }
    }

    fn get_config_host(&self, hostname: String) -> Result<&ConfigHost> {
        match self.config.hosts.get(&hostname) {
            Some(c) => Ok(c),
            None => Err(ExecError::NoConfigFound(hostname.clone()).into()),
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

        match task.name {
            Some(name) => {
                println!("running task: {}", name);
                println!("=> {}", exec.clone());
            }
            None => println!("running: {}", exec.clone()),
        };

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

    fn resolve_all_tasks_for_host(&self, hostname: String) -> Result<Vec<LiteralTaskDefinition>> {
        let host_config = self.get_config_host(hostname)?;

        for task in host_config.tasks.clone() {
            match task {
                TaskDefinition::Ref(name) => match self.config.tasks.get(&name) {
                    Some(_) => (),
                    None => return Err(ExecError::NoTaskFound(name).into()),
                },
                _ => (),
            }
        }

        Ok(host_config
            .tasks
            .iter()
            .map(|task| match task {
                TaskDefinition::Ref(name) => self.config.tasks.get(name).unwrap().clone(),
                TaskDefinition::Literal(lit) => lit.clone(),
            })
            .collect())
    }

    #[cfg(target_family = "unix")]
    fn make_link(&self, link: LinkDefinition) -> Result<()> {
        use std::fs::{copy, hard_link};
        use std::os::unix::fs::symlink;

        let LinkDefinition { from, to, kind, .. } = link;

        let resolved_from = self.config.resolve_variable_string(from)?;
        let resolved_to = self.config.resolve_variable_string(to)?;

        println!("making link {} -> {}", resolved_from, resolved_to);

        match kind {
            Some(LinkKind::Hard) => hard_link(resolved_from, resolved_to)?,
            Some(LinkKind::Soft) | None => symlink(resolved_from, resolved_to)?,
            Some(LinkKind::Copy) => copy(resolved_from, resolved_to).and(Ok(()))?,
        };

        Ok(())
    }

    #[cfg(target_family = "windows")]
    fn make_link(&self, link: LinkDefinition) -> Result<()> {
        use std::fs::metadata;
        use std::fs::{copy, hard_link};
        use std::os::windows::fs::{symlink_dir, symlink_file};

        let LinkDefinition { from, to, kind, .. } = link;

        let resolved_from = self.config.resolve_variable_string(from)?;
        let resolved_to = self.config.resolve_variable_string(to)?;

        println!("making link from {} to {}", resolved_from, resolved_to);

        match kind {
            Some(LinkKind::Hard) => hard_link(resolved_from, resolved_to)?,
            Some(LinkKind::Soft) | None => match metadata(resolved_from) {
                Some(data) => {
                    if data.is_dir() {
                        symlink_dir(resolved_from, resolved_to)?
                    } else {
                        symlink_file(resolved_from, resolved_to)?
                    }
                }
                None => return Err(ExecError::MetadataFailure(resolved_from).inresolved_to()),
            },
            Some(LinkKind::Copy) => copy(resolved_from, resolved_to).and(Ok(()))?,
        };

        Ok(())
    }
}
