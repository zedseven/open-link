use std::{env::args, io};

use webbrowser::open;

#[derive(Debug)]
enum Error {
	User(String),
	Io(io::Error),
}

fn main() -> Result<(), Error> {
	let link: String = args().skip(1).take(1).collect();
	if link.len() == 0 {
		return Err(Error::User(String::from(
			"Please supply the link to open as a command line argument.",
		)));
	}
	open(link.as_str()).map(|_| ()).map_err(|e| Error::Io(e))
}
