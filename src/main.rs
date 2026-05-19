use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use tera::{self, Tera};

use crate::structs::CV;

pub mod structs;

const DEFAULT_IN_FNAME: &str = "2cv.toml";
const DEFAULT_OUT_FNAME: &str = "output.typ";
const DEFAULT_DIR: &str = ".";
const DEFAULT_TMPLT_NAME: &str = "cv.typ";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_IN_FNAME)]
    input: String,

    #[arg(short, long, default_value = DEFAULT_OUT_FNAME)]
    output: String,

    #[arg(short, long, default_value = DEFAULT_DIR)]
    dir: String,

    #[arg(short, long, default_value_t = false)]
    print: bool,

    #[arg(short, long, default_value_t = false)]
    anon: bool,
}

fn parse_cfg_file(path: &str) -> Result<CV> {
    let toml_str = fs::read_to_string(path)?;
    let cv = toml::from_str::<CV>(&toml_str)?;
    Ok(cv)
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.dir != DEFAULT_DIR {
        std::env::set_current_dir(args.dir)?;
    }

    let mut cv =
        parse_cfg_file(&args.input).with_context(|| "Failed to read config file.".to_string())?;

    if args.anon {
        cv.anonymise();
    }

    let tmplt = include_str!("../template/cv.typ");
    let mut tera = Tera::default();
    tera.add_raw_template(DEFAULT_TMPLT_NAME, tmplt)?;
    let output = tera.render(DEFAULT_TMPLT_NAME, &tera::Context::from_serialize(&cv)?)?;

    if args.print {
        println!("{output}");
    }

    std::fs::write(args.output, output)
        .with_context(|| "Failed to dump output to file.".to_string())?;

    // TODO: run a typst formatter and `typst compile fname.typ` too

    Ok(())
}
