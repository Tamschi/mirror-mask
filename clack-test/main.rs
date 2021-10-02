use std::{env::args, io::stdin, process::Command};

use structopt::StructOpt;

#[derive(StructOpt)]
/// A program for testing `clack`.
///
/// Press `Ctrl-c` to abort this program at some point.
struct Options {
	/// Read a line from standard input before doing anything else.
	/// Then exit unless `-r` is also specified.
	#[structopt(short, long)]
	interactive: bool,

	#[structopt(short, long)]
	recursive: bool,
}

fn main() {
	let options = Options::from_args();

	if options.interactive {
		let read = stdin().read_line(&mut String::new()).unwrap();
		println!("Read {} codepoints.", read);
	}

	if options.interactive && !options.recursive {
		return;
	}

	print!("Launching child process…");

	let mut command = Command::new(args().next().unwrap());
	command.arg("-i");
	if options.recursive {
		command.arg("-r");
	}
	let mut child = command.spawn().unwrap();

	println!(" Spawned with id {}", child.id());

	match child.wait() {
		Ok(exit_status) => println!("Child process exited with status {}.", exit_status),
		Err(error) => println!("Child exited unsuccessfully: {}", error),
	}
}
