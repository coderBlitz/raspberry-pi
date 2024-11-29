//! Raspberry pi pico library
//!
//! This custom library contains the various pico constants, functions, and any
//!  utility wrappers that I write.

// TODO: Macro to implement _strat() and panic handler

#![no_std]

use core::arch::asm;

pub use registers::Register;

pub mod clocks;
pub mod consts;
pub mod gpio;
pub mod resets;
pub mod registers;
pub mod rom;
pub mod timer;
pub mod watchdog;


static UDIV_DIVIDEND: Register = unsafe { Register::new(consts::SIO_DIV_UDIVIDEND) };
static UDIV_DIVISOR: Register = unsafe { Register::new(consts::SIO_DIV_UDIVISOR) };
static DIV_QUOTIENT: Register = unsafe { Register::new(consts::SIO_DIV_QUOTIENT) };
static DIV_REMAINDER: Register = unsafe { Register::new(consts::SIO_DIV_REMAINDER) };

// Use the SIO for integer division. 64 bit combines quotient and remainder.
// Quotient should be high 32 bits, remainder low 32 bits.
// See https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#integer-32-32-32-division-functions
#[unsafe(no_mangle)]
extern "C" fn __aeabi_uidiv(num: u32, den: u32) -> u64 {
	// Write the values
	UDIV_DIVIDEND.set(num);
	UDIV_DIVISOR.set(den);

	// Delay 8 cycles
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();

	// Get results
	(DIV_QUOTIENT.get() as u64) << 32 | DIV_REMAINDER.get() as u64
}

#[inline(always)]
pub fn nop() {
	unsafe {
		asm!("NOP", options(nomem, nostack));
	}
}