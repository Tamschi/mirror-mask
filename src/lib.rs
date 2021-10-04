#![doc(html_root_url = "https://docs.rs/mirror-mask/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(dead_code)]

use std::{
	convert::TryInto,
	fmt::Debug,
	marker::PhantomData,
	mem::ManuallyDrop,
	process::Child,
	sync::atomic::{AtomicU32, Ordering},
}; //TODO

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[cfg(all(not(unix), feature = "required"))]
compile_error!("`mirror-mask` currently supports only the following platforms: `unix` (via `nix`)");

/// Maps to signals on Unix, messages on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Intent {
	/// The Ctrl-c signal.
	///
	/// Unix: `SIGINT`.
	InterruptFromKeyboard,
}

mod sys {
	#[cfg(unix)]
	mod unix;
	#[cfg(unix)]
	pub use unix::{send_intent_to_process, setup, Handle};

	#[cfg(not(unix))]
	mod null;
	#[cfg(not(unix))]
	pub use null::{send_intent_to_process, setup, Handle};

	impl Drop for Handle {
		fn drop(&mut self) {
			panic!()
		}
	}
}
use sys::{send_intent_to_process, setup, Handle};

static INTERRUPT_FROM_KEYBOARD_TARGET: AtomicU32 = AtomicU32::new(0);

fn sig_handler(intent: Intent) {
	let target = target_of_intent(intent).load(Ordering::Relaxed);
	send_intent_to_process(target, intent)
}

fn target_of_intent(intent: Intent) -> &'static AtomicU32 {
	match intent {
		Intent::InterruptFromKeyboard => &INTERRUPT_FROM_KEYBOARD_TARGET,
	}
}

impl Intent {
	/// # Panics
	///
	/// TODO (also mention drop panic)
	#[must_use = "The `RelayGuard` must be held onto for the signal redirection to remain active."]
	pub fn relay_to_child<'a>(&'a self, child: &Child) -> RelayGuard<'a> {
		RelayGuard {
			_a: PhantomData,
			_unsend: PhantomData,
			intent: *self,
			shadowed_target: target_of_intent(*self).swap(child.id(), Ordering::Relaxed), // <-- 1.
			shadowing_target: child.id(),
			handle: ManuallyDrop::new(setup(*self).unwrap()), // <-- 2.
		}
	}

	/// # Panics
	///
	/// TODO (also mention drop panic)
	#[must_use]
	pub fn relay_n_to_child<'a, const N: usize>(
		intents: &'a [Self; N],
		child: &Child,
	) -> [RelayGuard<'a>; N] {
		let mut result = vec![];
		for intent in intents {
			result.push(intent.relay_to_child(child))
		}
		result.reverse(); // Dropped front to back later.
		let result: Box<[RelayGuard; N]> = result.into_boxed_slice().try_into().unwrap();
		*result
	}
}

pub struct RelayGuard<'a> {
	_a: PhantomData<&'a ()>,
	_unsend: PhantomData<*mut ()>,
	intent: Intent,
	shadowed_target: u32,
	shadowing_target: u32,
	handle: ManuallyDrop<Handle>,
}

impl<'a> Debug for RelayGuard<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RelayGuard")
			.field("intent", &self.intent)
			.field("shadowed_target", &self.shadowed_target)
			.field("shadowing_target", &self.shadowing_target)
			.field("handle", &self.handle)
			.finish()
	}
}

impl<'a> Drop for RelayGuard<'a> {
	fn drop(&mut self) {
		unsafe { ManuallyDrop::take(&mut self.handle).free(self.intent) }; // <-- 1.
		let removed = target_of_intent(self.intent).swap(self.shadowed_target, Ordering::Relaxed);
		// <-- 2.

		assert_eq!(removed, self.shadowing_target, "Removed target did not match shadowing target. There were likely some threading issues. Note that `mirror-mask` performs no synchronisation beyong _very_ basic memory safety, but still requires correct context stacking!")
	}
}
