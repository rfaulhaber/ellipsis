use anyhow::Result;
use clap::Clap;
use ellipsis::{cmd::Runner, fs::read_config_file, opts::Opts};

fn main() -> Result<()> {
    let opts = Opts::parse();
    let config = read_config_file(opts.config.clone())?;

    println!("hosts: {:?}", config.hosts);

    Runner::new(config).execute_cmd(opts.subcommand)?;

    Ok(())
}
