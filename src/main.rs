use clap::Clap;
use ellipsis::{config::Config, fs::read_config_file, opts::Opts};
use std::path::Path;

fn main() {
    let opts = Opts::parse();

    let config = read_config_file(opts.config_path.clone());

    println!("opts {:?}", config);
}
