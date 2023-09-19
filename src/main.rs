use std::process::ExitCode;
use version_compare::{Version, Cmp};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Version 1
    v1: String,

    /// Version 2
    v2: String,
}

fn main() -> ExitCode {
  let cli = Cli::parse();

  let ver1 = match Version::from(&cli.v1) {
    Some(v) => v,
    _ => { eprintln!("{} is not a valid semver", cli.v1); std::process::exit(1) }
  };

  let ver2 = match Version::from(&cli.v2) {
    Some(v) => v,
    _ => { eprintln!("{} is not a valid semver", cli.v2); std::process::exit(1) }
  };

  match ver1.compare(ver2) {
    Cmp::Lt => println!("<"),
    Cmp::Eq => println!("="),
    Cmp::Gt => println!(">"),
    _ => unreachable!(),
  }

  ExitCode::SUCCESS
}
