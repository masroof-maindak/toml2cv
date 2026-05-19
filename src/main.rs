use anyhow::{Context, Result, bail};
use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;
use tera::{self, Tera};

use crate::structs::CV;

pub mod structs;

const DEFAULT_IN_FNAME: &str = "2cv.toml";
const DEFAULT_OUT_FNAME: &str = "output.typ";
const DEFAULT_DIR: &str = ".";
const DEFAULT_TMPLT_NAME: &str = "cv.typ";
const FMT_BIN: &str = "typstyle";
const TYPST_BIN: &str = "typst";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_IN_FNAME, help = "input filename")]
    input: String,

    #[arg(short, long, default_value = DEFAULT_OUT_FNAME, help = "output filename")]
    output: String,

    #[arg(short, long, default_value = DEFAULT_DIR, help = "directory to look for input file in")]
    dir: String,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "whether to anonymise some personal info"
    )]
    anon: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "run formatter after generating output"
    )]
    format: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "compile w/ typst after generating output"
    )]
    compile: bool,
}

fn check_if_in_path(bin: &str) -> Result<bool> {
    match std::env::var("PATH") {
        Ok(paths) => Ok(paths
            .split(":")
            .map(|p| format!("{}/{}", p, bin))
            .any(|p| Path::new(&p).exists())),
        Err(e) => bail!(format!("Failed to find binary `{bin}` in PATH `{e}`.")),
    }
}

fn run_bin(bin: &str, args: &[&str]) -> Result<()> {
    let exists = check_if_in_path(bin).with_context(|| "Failed to search PATH".to_string())?;

    if exists {
        Command::new(bin)
            .args(args)
            .spawn()
            .with_context(|| format!("{bin} failed to start."))?;

        // TODO: check exit code / stderr output
    } else {
        bail!("Failed to find `{bin}` in PATH; skipping.")
    }

    Ok(())
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

    std::fs::write(&args.output, output)
        .with_context(|| "Failed to dump output to file.".to_string())?;

    // CHECK: better way to handle results?

    if args.format
        && let Err(e) = run_bin(FMT_BIN, &[&args.output, "--wrap-text", "--inplace"])
    {
        eprintln!("Format failed: {e}")
    }

    if args.compile
        && let Err(e) = run_bin(TYPST_BIN, &["compile", &args.output])
    {
        eprintln!("Typst compile failed: {e}")
    }

    Ok(())
}
