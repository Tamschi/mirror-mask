#![doc(html_root_url = "https://docs.rs/clack/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(dead_code)] //TODO

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[cfg(not(unix))]
compile_error!("`clack` currently supports only the following platforms: `unix` (via `nix`)");

/// Maps to signals on Unix, messages on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Intent {
	/// The Ctrl-c signal.
	///
	/// Unix: `SIGINT`.
	InterruptFromKeyboard,
}

#[cfg(unix)]
mod unix;

fn sig_handler(intent: Intent) {}
