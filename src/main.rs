use std::env;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}


fn main() {
	let args = Cli::from_args();
    println!("{param}", param = args.source);
}
