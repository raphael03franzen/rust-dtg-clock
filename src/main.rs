use chrono::prelude::*;

enum Mode {
    Local,
    Utc,
    Both,
}
use std::env;
fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let mode: Mode = match argc {
        0 | 1 => Mode::Both,
        2.. => match &argv[1] {
            val if *val == "--local".to_string() => Mode::Local,
            val if *val == "--utc".to_string() => Mode::Utc,
            val if *val == "--both" => Mode::Both,
            _ => {
                eprintln!("Invallid Argument, assuming --both");
                Mode::Both
            }
        },
    };
    match mode {
        Mode::Utc => print_utc(),
        Mode::Local => print_local(),
        Mode::Both => print_local_utc(),
    }
}

fn print_local() {
    let local: DateTime<Local> = Local::now();
    let tz = local.format("%Z").to_string();
    let tz = parse_tz(&tz);
    println!("Local Time:\t{}{}{}", local.format("%d%H%M"),tz,local.format("%b%y"));
}
fn print_utc() {
    let utc: DateTime<Utc> = Utc::now();
    let tz = 'Z';
    println!("UTC Time:\t{}{}{}", utc.format("%d%H%M"),tz,utc.format("%b%y"));
}
fn print_local_utc() {
    let local: DateTime<Local> = Local::now();
    let tz = local.format("%Z").to_string();
    let tz = parse_tz(&tz);
    println!("Local Time:\t{}{}{}", local.format("%d%H%M"),tz,local.format("%b%y"));
    let utc: DateTime<Utc> = Utc::now();
    let tz = 'Z';
    println!("UTC Time:\t{}{}{}", utc.format("%d%H%M"),tz,utc.format("%b%y"));
}
fn parse_tz(tz: &str) -> char {
    match tz {
        "-12:00" => 'Y',
        "-11:00" => 'X',
        "-10:00" => 'W',
        "-09:00" => 'V',
        "-08:00" => 'U',
        "-07:00" => 'T',
        "-06:00" => 'S',
        "-05:00" => 'R',
        "-04:00" => 'Q',
        "-03:00" => 'P',
        "-02:00" => 'O',
        "-01:00" => 'N',
        "+00:00" => 'Z',
        "+01:00" => 'A',
        "+02:00" => 'B',
        "+03:00" => 'C',
        "+04:00" => 'D',
        "+05:00" => 'E',
        "+06:00" => 'F',
        "+07:00" => 'G',
        "+08:00" => 'H',
        "+09:00" => 'I',
        "+10:00" => 'K',
        "+11:00" => 'L',
        "+12:00" => 'M',
        _ => todo!(),
    }
}
