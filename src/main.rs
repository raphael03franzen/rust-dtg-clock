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

        //Enable color
        #[arg(long,short)]
        color: bool,
}

fn main() {
	let args = Args::parse();
	let mode: Mode = match &args.mode as &str {
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
		Mode::Local => dtg::print_local(args.color),
		Mode::Utc => dtg::print_utc(args.color),
		Mode::Both => {
			dtg::print_local(args.color);
			dtg::print_utc(args.color);
		}
	}
}
