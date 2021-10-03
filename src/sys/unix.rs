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
				SaFlags::SA_NODEFER | SaFlags::SA_RESTART,
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
		f.debug_struct("Handle (unix)")
			.field("(private)", &"..")
			.finish()
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
		s if s == Signal::SIGINT as i32 => Intent::InterruptFromKeyboard,
		_ => unimplemented!(),
	})
}

pub fn send_intent_to_process(pid: u32, intent: Intent) {
	kill(
		Pid::from_raw(pid.try_into().unwrap()),
		intent_to_signal(intent),
	)
	.ok();
}
