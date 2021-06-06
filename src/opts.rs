use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Ryan Faulhaber <ryf@sent.as>")]
pub struct Opts {
    /// Path to config file. If not specified, it will look for a file in the
    /// current directory called "ellipsis".
    #[clap(long)]
    pub config: Option<String>,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    Install(InstallCmd),
    Link(LinkCmd),
    Exec(ExecCmd),
}

/// Makes some or all links for a host.
#[derive(Clap, Debug)]
pub struct LinkCmd {
    /// Host to execute.
    pub hostname: String,

    /// Links to run. Makes all links if not specified.
    pub args: Vec<String>,
}

/// Runs all tasks for a host.
#[derive(Clap, Debug)]
pub struct InstallCmd {
    /// Host to execute.
    pub hostname: String,
}

/// Executes a particular task for a host.
#[derive(Clap, Debug)]
pub struct ExecCmd {
    /// Host to execute.
    pub hostname: String,

    /// Task to execute.
    pub task_name: String,
}
