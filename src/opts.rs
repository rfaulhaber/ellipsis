use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Ryan Faulhaber <ryf@sent.as>")]
pub struct Opts {
    #[clap(long)]
    pub config_path: Option<String>,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    Install(InstallCmd),
    Link(LinkCmd),
}

#[derive(Clap, Debug)]
pub struct LinkCmd {}

#[derive(Clap, Debug)]
pub struct InstallCmd {}
