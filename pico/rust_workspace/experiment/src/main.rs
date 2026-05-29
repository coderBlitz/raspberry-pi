#![no_std]
#![no_main]

use core::time::Duration;
use pico::gpio::*;
use pico::*;

extern "C" {
    static _start_ramload: u8;
    static _end_ramload: u8;
    static _end_text: u8;
}

// NOTES:
// This fails if XIP is enabled. The copy line (27 as of writing) causes the
// fault.
fn copy_ram_fns() {
    unsafe {
    	//let mut count = 0;
        let mut src = &raw const _end_text;
        let mut dst = (&raw const _start_ramload) as *mut u8;
        let dst_end = &raw const _end_ramload;
        while dst < dst_end as _ {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
            //count += 1;
        }

        //count as u32
    }
}

fn check_size() -> u32 {
	unsafe {
	(&raw const _end_ramload).offset_from(&raw const _start_ramload) as u32
	}
}

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
    //stuff();
    test_modulus();

    // Testing copy RAM
    //copy_ram_fns();
    //let size = check_size();
    //flash_n(size, Some(Duration::from_millis(250)));
    //flash_n(1, Some(Duration::from_secs(2)));

    //led_on_from_ram();

    //flash_led();
    // Heartbeat style flashing.
    loop {
    	flash_n(2, Some(Duration::from_millis(100)));
    	timer::sleep(Duration::from_millis(500));
    }
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

fn test_modulus() {
	const N: u64 = 20;
	for i in 0..N {
		//let val = core::hint::black_box(i) % core::hint::black_box(N);
		let val = core::hint::black_box(i) % N;
		if val != i {
			loop {}
		}
	}
}

#[inline(always)]
fn set_led(on: bool) {
    let pin25_ctrl = Gpio::from(GpioPin::Gpio25);
    pin25_ctrl.oe_override(GpioOverride::HighEnable);
    pin25_ctrl.select_fn(GpioFn::Fn5);

    if on {
        pin25_ctrl.out_override(GpioOverride::HighEnable);
    } else {
        pin25_ctrl.out_override(GpioOverride::LowDisable);
    }
}

fn flash_n(n: u32, time: Option<Duration>) {
	let sleep_time = time.unwrap_or(Duration::from_millis(500));
	for _ in 0..n {
		set_led(true);
		timer::sleep(sleep_time);
		set_led(false);
		timer::sleep(sleep_time);
	}
}

#[unsafe(no_mangle)]
#[link_section = ".ram"]
fn led_on_from_ram() {
	loop {
		set_led(true);
	}
}

//#[link_section = ".ram"] // Cannot inline when trying this.
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
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
