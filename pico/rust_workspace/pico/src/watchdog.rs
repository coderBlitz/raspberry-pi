//! Watchdog subsystem

use crate::consts::all::*;
use crate::Register;

static TICK: Register = unsafe { Register::new(WATCHDOG_TICK) };

pub fn start_tick() {
	TICK.set(WATCHDOG_TICK_ENABLE_BIT | 12); // Assume XOSC for now.

	while (TICK.get() & WATCHDOG_TICK_RUNNING_BIT) == 0 {}
}