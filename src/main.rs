use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
	///Print local time
	#[arg(short, long)]
	local: bool,
	///Print UTC time
	#[arg(short, long)]
	utc: bool,
}

fn main() {
	let args = Args::parse();
	if args.utc {
		dtg::print_utc();
	}
	if args.local {
		dtg::print_local();
	}
	if !args.local && !args.utc {
		dtg::print_local();
		dtg::print_utc();
	}
}
