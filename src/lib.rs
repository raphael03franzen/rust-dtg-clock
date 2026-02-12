use chrono::prelude::*;
///Print DTG timestamp, using Local Time
pub fn print_local() {
	print("Local Time:\t", Local::now())
}
///Print DTG timestamp, using UTC-Time
pub fn print_utc() {
	print("UTC Time:\t", Utc::now());
}
fn print<Tz: chrono::TimeZone>(label: &str, dt: DateTime<Tz>)
where
	<Tz as TimeZone>::Offset: std::fmt::Display,
{
	let tz = dt.format("%Z").to_string();
	let tz = parse_tz(&tz);
	println!(
		"{}\t{}{}{}",
		label,
		dt.format("%d%H%M"),
		tz,
		dt.format("%b%y")
	);
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
