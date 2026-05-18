use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

use crate::structs::CV;

pub mod structs;

const DEFAULT_FNAME: &str = "cv2res.toml";
const DEFAULT_FPATH: &str = ".";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_FNAME)]
    fpath: String,

    #[arg(short, long, default_value = DEFAULT_FPATH)]
    dir: String,
}

fn parse_cfg_file(path: &str) -> Result<CV> {
    let toml_str = fs::read_to_string(path)?;
    let cv = toml::from_str::<CV>(&toml_str)?;
    Ok(cv)
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Running on {} in {}.", args.fpath, args.dir);

    if args.dir != DEFAULT_FPATH {
        std::env::set_current_dir(args.dir)?;
    }

    let _cv_info =
        parse_cfg_file(&args.fpath).with_context(|| "Failed to read config file.".to_string());

    Ok(())
}
