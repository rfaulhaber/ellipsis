use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Ryan Faulhaber <ryf@sent.as>")]
pub struct Opts {
    /// Path to config file. If not specified, it will look for a file in the
    /// current directory called "ellipsis".
    #[clap(long)]
    pub config: Option<String>,

    /// Overrides hostname check, manually specifying the hostname to run.
    #[clap(long)]
    pub hostname: Option<String>,

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
    /// Hard links files rather than soft link. Exclusive with -c
    #[clap(short, long)]
    pub hard: Option<bool>,

    /// Copies files rather than links. Exclusive with -h
    #[clap(short, long)]
    pub copy: Option<bool>,

    /// Links to run. Makes all links if not specified.
    pub args: Vec<String>,
}

/// Runs all tasks for a host.
#[derive(Clap, Debug)]
pub struct InstallCmd {}

/// Executes a particular task for a host.
#[derive(Clap, Debug)]
pub struct ExecCmd {
    /// Task to execute.
    pub arg: String,
}
