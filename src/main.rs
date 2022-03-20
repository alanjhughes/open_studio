use std::env;
use std::process::Command;
use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(name = "studio")]
struct Args {
    #[clap(help = "Open Android Studio in this directory", short, long, default_value = ".")]
    dir: String,
}

fn main() {
    let args: Args = Args::parse();

    let mut command = Command::new("open");
    command.arg("-a").arg("Android Studio.app");

    let current = env::current_dir().expect("Failed to open directory");

    if args.dir == "." {
        command.arg(current);
    } else {
        command.arg(args.dir);
    }

    command.spawn().expect("Failed to run program");
}
