enum Mode {
	Local,
	Utc,
	Both,
}
use dtg::{print_local, print_local_utc, print_utc};
use std::env;
fn main() {
	let args: Vec<String> = env::args().collect();
	let mode: Mode;
	if args.len() == 1 {
		mode = Mode::Both;
	} else if args.len() == 2 {
		mode = match &args[1] {
			val if *val == "--local" => Mode::Local,
			val if *val == "--utc" => Mode::Utc,
			val if *val == "--both" => Mode::Both,
			_ => {
				eprintln!("Invalid Argument.");
				usage(&args[0]);
				return;
			}
		};
	} else {
		usage(&args[0]);
		return;
	}

	match mode {
		Mode::Utc => print_utc(),
		Mode::Local => print_local(),
		Mode::Both => print_local_utc(),
	}
}
fn usage(name: &str) {
	eprintln!("Usage: {} [--local | --utc | --both]", name);
}
