extern crate core;

use std::path::PathBuf;
use std::process::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "studio")]
struct Args {
    #[clap(short = 'd', parse(from_os_str), value_hint = clap::ValueHint::DirPath, default_value = ".")]
    dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let mut command = Command::new("open");
    command.arg("-a").arg("Android Studio.app").arg("--args");

    let path = args.dir.canonicalize()?;

    command.arg(path);

    command.spawn().expect("Failed to run program");
    Ok(())
}
