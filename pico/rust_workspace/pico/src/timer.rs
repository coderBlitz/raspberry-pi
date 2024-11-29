//! Timer system.

use crate::consts::*;
use crate::Register;
use core::time::Duration;

static TIMERAWH: Register = unsafe { Register::new(TIMER_TIMERAWH) };
static TIMERAWL: Register = unsafe { Register::new(TIMER_TIMERAWL) };

pub struct Instant(u64);
impl Instant {
	pub fn now() -> Self {
		Instant(get_time_us())
	}

	pub fn elapsed(&self) -> Duration {
		Duration::from_millis(self.0)
	}
}

/// Get the current time.
///
/// Taken from the programmer model (pg 539) where unlatched values used to
///  ensure time is correct. Since high bits unlikely to change during the
///  execution of this function, should require a loop once every ~4300s
///  (~71min).
fn get_time_us() -> u64 {
	let mut time_hi = TIMERAWH.get();
	let mut time_lo;
	loop {
		time_lo = TIMERAWL.get();
		let now_hi = TIMERAWH.get();
		if now_hi == time_hi {
			break;
		} else {
			time_hi = now_hi;
		}
	}

	(time_hi as u64) << 32 | time_lo as u64
}

/// Sleep for the given duration
// TODO: Utilize WFI to sleep until interrupt. SysTick, Alarm, etc. can
//        generate the waking interrupt.
pub fn sleep(dur: Duration) {
	// TODO:
	// 1. Arm interrupt however
	// 2. Use WFI to sleep
	// 3. Upon waking, check elapsed duration and WFI again if short.
	// 4. Else done.
}