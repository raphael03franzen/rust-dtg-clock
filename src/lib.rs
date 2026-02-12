use chrono::prelude::*;
///Print DTG timestamp, using Local Time
pub fn print_local(color: bool) {
	print("Local Time:\t", Local::now(), color)
}
///Print DTG timestamp, using UTC-Time
pub fn print_utc(color: bool) {
	print("UTC Time:\t", Utc::now(), color);
}
fn print<Tz: chrono::TimeZone>(label: &str, dt: DateTime<Tz>, color: bool)
where
	<Tz as TimeZone>::Offset: std::fmt::Display,
{
	let tz = dt.format("%Z").to_string();
	let tz = parse_tz(&tz);
        
        let mut label:String = label.to_string();
        let mut tz:String = tz.to_string();
	let mut day:String = dt.format("%d").to_string();
	let mut hour:String = dt.format("%H").to_string();
	let mut minute:String = dt.format("%M").to_string();
	let mut month:String = dt.format("%b").to_string();
	let mut year:String = dt.format("%y").to_string();

	if color {
                use minimal_ansi_color::AnsiColor;
		label = format!("{}{label}{}", AnsiColor::GREEN, AnsiColor::RESET);
		tz = format!("{}{tz}{}", AnsiColor::GREEN, AnsiColor::RESET);
		day = format!("{}{day}{}", AnsiColor::RED, AnsiColor::RESET);
		hour = format!("{}{hour}{}", AnsiColor::CYAN, AnsiColor::RESET);
		minute = format!("{}{minute}{}", AnsiColor::CYAN, AnsiColor::RESET);
		month = format!("{}{month}{}", AnsiColor::RED, AnsiColor::RESET);
		year = format!("{}{year}{}", AnsiColor::RED, AnsiColor::RESET);
        }

	println!("{label}\t{day}{hour}{minute}{tz}{month}{year}",);
}
fn parse_tz(tz: &str) -> &str {
	match tz {
		"-12:00" => "Y",
		"-11:00" => "X",
		"-10:00" => "W",
		"-09:00" => "V",
		"-08:00" => "U",
		"-07:00" => "T",
		"-06:00" => "S",
		"-05:00" => "R",
		"-04:00" => "Q",
		"-03:00" => "P",
		"-02:00" => "O",
		"-01:00" => "N",
		"+00:00" | "UTC" => "Z",
		"+01:00" => "A",
		"+02:00" => "B",
		"+03:00" => "C",
		"+04:00" => "D",
		"+05:00" => "E",
		"+06:00" => "F",
		"+07:00" => "G",
		"+08:00" => "H",
		"+09:00" => "I",
		"+10:00" => "K",
		"+11:00" => "L",
		"+12:00" => "M",
		&_ => "_",
	}
}
