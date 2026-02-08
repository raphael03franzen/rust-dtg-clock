#[derive(Debug)]
struct Mode {
	print_local: bool,
	print_utc: bool,
}
fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mode: Mode;
	if args.len() == 1 {
		mode = Mode {
			print_local: true,
			print_utc: true,
		};
	} else if args.len() == 2 {
		mode = match &args[1] {
			val if *val == "--local" => Mode {
				print_local: true,
				print_utc: false,
			},
			val if *val == "--utc" => Mode {
				print_local: false,
				print_utc: true,
			},
			val if *val == "--both" => Mode {
				print_local: true,
				print_utc: true,
			},
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
	if mode.print_local {
		dtg::print_local();
	}
	if mode.print_utc {
		dtg::print_utc();
	}
}
fn usage(name: &str) {
	eprintln!("Usage: {} [--local | --utc | --both]", name);
}
