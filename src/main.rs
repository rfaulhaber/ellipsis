use anyhow::Result;
use clap::Clap;
use ellipsis::{cmd::Runner, config::Config, fs::read_config_file, opts::Opts};
use std::path::Path;

fn main() -> Result<()> {
    let opts = Opts::parse();

    let config = read_config_file(opts.config.clone())?;

    // println!("opts {:?}", config);

    Runner::new(config)?.execute_cmd(opts.subcommand)?;

    Ok(())
}
