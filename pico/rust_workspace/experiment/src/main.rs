#![no_std]
#![no_main]

use pico::*;
use core::arch::asm;
use core::time::Duration;

fn main() {
	resets::enable_io_bank0();

	// Enable and switch REF and SYS to the XOSC.
	clocks::enable_xosc();
	clocks::ref_to_xosc();

	// Enable and configure PLL, then switch SYS to PLL.
	clocks::configure_pll();
	clocks::sys_to_pll();

	// Start the tick for timing purposes.
	watchdog::start_tick();

	// Try things
	stuff();

	flash_led();
}

fn stuff() {
	// Try to get ROM function and see if it's null or not.
	let code = rom::rom_table_code(b'T', b'3');
	let ctz: extern "C" fn(u32) -> u32 = rom::func_table_lookup(code);

	if ctz(0) != 32 {
		//panic!();
		loop {}
	}

	// Test the unsigned division function
	if (ctz(0) / 6) != 5 {
		loop {}
	}
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

	let sleep_time = Duration::from_secs(1);

	// End state is infinite loop
	loop {
		// Turn LED OFF
		set_led(true);

		// Delay
		timer::sleep(sleep_time);

		// Turn LED ON
		set_led(false);

		// Delay
		timer::sleep(sleep_time);
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