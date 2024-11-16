#![no_std]
#![no_main]

use pico;
use core::arch::asm;


fn main() {
	pico::resets::enable_iobank();
	flash_led();
}

#[inline(always)]
fn set_led(on: bool) {
	let pin25_ctrl = pico::consts::GPIO25_CTRL as *mut u32;
	const LED_PIN_STATE_ON: u32 = 0x0000_3305;
	const LED_PIN_STATE_OFF: u32 = 0x0000_3205;

	if on {
		unsafe { pin25_ctrl.write_volatile(LED_PIN_STATE_ON); }
	} else {
		unsafe { pin25_ctrl.write_volatile(LED_PIN_STATE_OFF); }
	}
}

#[inline(always)]
fn flash_led() -> ! {
	/* Attempt to turn on LED.
		1. Un-reset APB/GPIO interface
		2. Set LED pin (GPIO25) to SIO mode
		3. Set LED pin high (turn on)
	*/

	set_led(false);

	// End state is infinite loop
	let mut i: usize;
	loop {
		// Turn LED OFF
		set_led(true);

		// Delay
		i = 0;
		while i < 0x50000 {
			nop();
			i += 1;
		}

		// Turn LED ON
		set_led(false);

		// Delay
		i = 0;
		while i < 0x50000 {
			nop();
			i += 1;
		}
	}
}

#[inline(always)]
fn nop() {
	unsafe {
		asm!("NOP", options(nomem, nostack));
	}
}
#[no_mangle]
#[link_section = ".strat"]
pub extern "C" fn _strat() -> ! {
	_ = main();

	loop {}
}
#[panic_handler]
fn panic( _info: &core::panic::PanicInfo) -> ! {
	loop {}
}