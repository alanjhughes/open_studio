extern crate core;

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug)]
struct DirError(String);

#[derive(Parser, Debug)]
#[clap(name = "studio")]
struct Args {
    #[clap(short = 'd', parse(from_os_str), value_hint = clap::ValueHint::DirPath, default_value = ".")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let mut command = Command::new("open");

    command.arg("-a").arg("Android Studio.app").arg("--args");

    let path = args
        .dir
        .canonicalize()
        .with_context(|| format!("Error: could not find directory `{}`", args.dir.display()))?;

    command.arg(path);
    command.spawn()?;
    Ok(())
}
