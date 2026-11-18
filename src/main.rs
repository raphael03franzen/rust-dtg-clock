use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
	//Enable color
	#[arg(long, short)]
	color: bool,
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
		dtg::print_utc(args.color);
	}
	if args.local {
		dtg::print_local(args.color);
	}
	if !args.local && !args.utc {
		dtg::print_local(args.color);
		dtg::print_utc(args.color);
	}
}
