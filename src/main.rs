use glob::glob;
use std::env;
use std::fs;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}


fn main() {
	let args = Cli::from_args();

	for entry in glob(&args.source).unwrap() {
		match entry {
			Ok(path) => println!("{:?}", path.display()),
			Err(e) => println!("{:?}", e),
		}
	}
}
