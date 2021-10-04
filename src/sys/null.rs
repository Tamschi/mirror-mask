#![allow(clippy::unnecessary_wraps, clippy::unused_self, unused_variables)]

use crate::Intent;
use std::mem;

pub fn setup(intent: Intent) -> Result<Handle, ()> {
	Ok(Handle)
}

#[derive(Debug)]
pub struct Handle;

impl Handle {
	pub fn free(self, intent: Intent) {
		mem::forget(self)
	}
}

pub fn send_intent_to_process(pid: u32, intent: Intent) {}
