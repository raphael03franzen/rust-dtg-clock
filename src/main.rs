use clap::Parser;
#[derive(Debug)]
enum Mode {
	Local,
	Utc,
	Both,
	Invalid,
}
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
	///Mode select [local,utc,both]
	#[arg(long,short,default_value_t = String::from("both"))]
	mode: String,
}

fn main() {
	let args = Args::parse();
	let mode: &str = &args.mode;
	let mode: Mode = match mode {
		"local" => Mode::Local,
		"utc" => Mode::Utc,
		"both" => Mode::Both,
		_ => Mode::Invalid,
	};
	match mode {
		Mode::Invalid => {
			eprintln!("Invalid Mode!");
			return;
		}
		Mode::Local => dtg::print_local(),
		Mode::Utc => dtg::print_utc(),
		Mode::Both => {
			dtg::print_local();
			dtg::print_utc();
		}
	}
}
