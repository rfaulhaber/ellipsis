use crate::config::LinkDefinition;
use crate::opts::{ExecCmd, InstallCmd, LinkCmd, SubCommand};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Could not get hostname")]
    NoHostname,
}

pub struct Runner {
    hostname: String,
}

impl Runner {
    fn new() -> Result<Self> {
        let name = match hostname::get()?.into_string() {
            Ok(name) => name,
            Err(_) => return Err(ExecError::NoHostname.into()),
        };

        Ok(Runner { hostname: name })
    }
}

pub fn execute_cmd(cmd: SubCommand) -> Result<()> {
    match cmd {
        SubCommand::Install(c) => execute_install_cmd(c),
        SubCommand::Link(c) => execute_link_cmd(c),
        SubCommand::Exec(c) => execute_exec_cmd(c),
    }
}

fn execute_install_cmd(cmd: InstallCmd) -> Result<()> {
    todo!();
}

fn execute_link_cmd(cmd: LinkCmd) -> Result<()> {
    todo!();
}

fn execute_exec_cmd(cmd: ExecCmd) -> Result<()> {
    todo!();
}

#[cfg(target_family = "unix")]
fn make_link(link: LinkDefinition) -> Result<()> {
    use std::os::unix::fs::symlink;

    let LinkDefinition { from, to } = link;
    symlink(from, to)?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn make_link(link: LinkDefinition) -> Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};

    todo!();
}
