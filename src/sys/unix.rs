use std::{
	convert::TryInto,
	fmt::{self, Debug, Formatter},
	mem,
};

use crate::Intent;
use nix::{
	libc::c_int,
	sys::signal::{kill, sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal},
	unistd::Pid,
};

pub fn setup(intent: Intent) -> Result<Handle, ()> {
	let signal = intent_to_signal(intent);
	let previous_action = unsafe {
		sigaction(
			signal,
			&SigAction::new(
				SigHandler::Handler(sig_handler),
				SaFlags::SA_NODEFER,
				SigSet::empty(),
			),
		)
	}
	.map_err(|_| ())?;
	Ok(Handle {
		saved_action: previous_action,
	})
}

fn intent_to_signal(intent: Intent) -> Signal {
	match intent {
		Intent::InterruptFromKeyboard => Signal::SIGINT,
	}
}

pub struct Handle {
	saved_action: SigAction,
}

impl Debug for Handle {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("Handle (unix)").finish_non_exhaustive()
	}
}

impl Handle {
	pub fn free(self, intent: Intent) {
		match unsafe { sigaction(intent_to_signal(intent), &self.saved_action) } {
			Ok(_) => mem::forget(self),
			Err(errno) => panic!("{}", errno),
		}
	}
}

extern "C" fn sig_handler(signal: c_int) {
	crate::sig_handler(match signal {
		s if s == Signal::SIGHUP as i32 => unimplemented!(),
		s if s == Signal::SIGINT as i32 => Intent::InterruptFromKeyboard,
		s if s == Signal::SIGQUIT as i32 => unimplemented!(),
		s if s == Signal::SIGILL as i32 => unimplemented!(),
		s if s == Signal::SIGTRAP as i32 => unimplemented!(),
		s if s == Signal::SIGABRT as i32 => unimplemented!(),
		s if s == Signal::SIGBUS as i32 => unimplemented!(),
		s if s == Signal::SIGFPE as i32 => unimplemented!(),
		s if s == Signal::SIGKILL as i32 => unreachable!(),
		s if s == Signal::SIGUSR1 as i32 => unimplemented!(),
		s if s == Signal::SIGSEGV as i32 => unimplemented!(),
		s if s == Signal::SIGUSR2 as i32 => unimplemented!(),
		s if s == Signal::SIGPIPE as i32 => unimplemented!(),
		s if s == Signal::SIGALRM as i32 => unimplemented!(),
		s if s == Signal::SIGTERM as i32 => unimplemented!(),
		s if s == Signal::SIGSTKFLT as i32 => unimplemented!(),
		s if s == Signal::SIGCHLD as i32 => unimplemented!(),
		s if s == Signal::SIGCONT as i32 => unimplemented!(),
		s if s == Signal::SIGSTOP as i32 => unreachable!(),
		s if s == Signal::SIGTSTP as i32 => unimplemented!(),
		s if s == Signal::SIGTTIN as i32 => unimplemented!(),
		s if s == Signal::SIGTTOU as i32 => unimplemented!(),
		s if s == Signal::SIGURG as i32 => unimplemented!(),
		s if s == Signal::SIGXCPU as i32 => unimplemented!(),
		s if s == Signal::SIGXFSZ as i32 => unimplemented!(),
		s if s == Signal::SIGVTALRM as i32 => unimplemented!(),
		s if s == Signal::SIGPROF as i32 => unimplemented!(),
		s if s == Signal::SIGWINCH as i32 => unimplemented!(),
		s if s == Signal::SIGIO as i32 => unimplemented!(),
		s if s == Signal::SIGPWR as i32 => unimplemented!(),
		s if s == Signal::SIGSYS as i32 => unimplemented!(),
		_ => unreachable!(),
	})
}

pub fn send_intent_to_process(pid: u32, intent: Intent) {
	kill(
		Pid::from_raw(pid.try_into().unwrap()),
		intent_to_signal(intent),
	)
	.ok();
}
