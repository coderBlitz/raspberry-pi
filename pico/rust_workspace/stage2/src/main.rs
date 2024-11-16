//! Pico compilation and flashing test
//!
//! Simple file to test compilation of rust code for flashing the pi pico.

#![no_std]
#![no_main]

#![allow(dead_code)]

use core::arch::asm;
use core::panic::PanicInfo;
use pico::{
	self,
	consts::all::*,
	gpio::Gpio,
	resets,
};

/// The entry function on boot (as defined in picomap.ld)
///
/// Will call all other functions and do all processing. Not named _start or
///  _main to avoid accidental success when linking/compiling.
//
#[no_mangle]
#[link_section = ".strat"]
pub extern "C" fn _strat() -> ! {
	//enable_xip();
	//pico::rom::flash_enter_cmd_xip();

	jump_to_entry()
}

#[inline(always)]
fn jump_to_entry() -> ! {
	unsafe {
		// Set stack address
		const SADDR: u32 = SRAM5_BASE; // Stack grows down, so this places it in SRAM4.
		asm!(
			"mov sp, {saddr}",
			saddr = in(reg) SADDR,
			options(nomem)
		);

		// Jump to main.
		//const MAIN_ADDR: u32 = 0x1000_0100; // Flash load (need to get XIP working first)
		const MAIN_ADDR: u32 = 0x2000_0100; // RAM load
		asm!(
			"mov pc, {addr}",
			addr = in(reg) MAIN_ADDR,
			options(noreturn)
		)
	}
}

/// Yet another attempt to get XIP working. Refer to SPI_notes.md for details.
///
/// The process this time:
/// 1. Check PSM_BASE.DONE register for all needed peripheral's registers.
/// 2. Un-reset RESETS_BASE for any needed peripherals (SPI, IO, etc.). (TODO: check if this is always before config)
/// 3. Check RESETS_DONE register for ready status.
/// 4. Write SSI control register (INST_L = 8 bits, ADDR_L = 24 bits, XIP_CMD = 0x03). May need to disable SSI before config.
/// 4b. After XIP enabled (should verify with testing first), enable quad read with (INST_L = 0, ADDR_L = 32 bits, XIP_CMD = 0xa0)
/// 5. XIP
// TODO: Try inserting NOP delays or find status registers to check.
#[inline(always)]
fn enable_xip() { unsafe {
	// Make register vars
	let xip_ssienr = XIP_SSI_SSIENR as *mut u32;
	let xip_ctrl = XIP_CTRL_BASE as *mut u32;
	let xip_ctrlr0 = XIP_SSI_CTRLR0 as *mut u32;
	let xip_ctrlr1 = XIP_SSI_CTRLR1 as *mut u32;
	let xip_spi_ctrlr0 = XIP_SSI_SPI_CTRLR0 as *mut u32;
	let xip_baudr = XIP_SSI_BAUDR as *mut u32;

	// Set QSPI pins to XIP (GPIO_QSPI_SCLK_CTRL ... GPIO_QSPI_SD3_CTRL) simply by clearing to all 0.
	resets::enable_io_qspi();
	Gpio::qspi_id(0).use_f0(); // SCLK
	Gpio::qspi_id(1).use_f0(); // SS
	Gpio::qspi_id(2).use_f0(); // SD0
	Gpio::qspi_id(3).use_f0(); // SD1
	Gpio::qspi_id(4).use_f0(); // SD2
	Gpio::qspi_id(5).use_f0(); // SD3

	// Quad-mode (0x2), data frame size 32 bits (0x1F)
	const CTRLR0: u32 = 0 | 0x2 << 21 | 0x1F << 16;
	//
	// See Pg. 571 and 608
	// XIP_CMD = 0xa0, WAIT_CYCLES = 31/32 (0x1F) from DFS_32, INST_L = 0, ADDR_L = 32 bits ()
	const SPI_CTRLR0: u32 = 0 | 0xa0 << 24 | 0x1F << 11 | 0x8 << 2;

	// Disable SSI
	xip_ssienr.write_volatile(0);

	// Enable cache
	xip_ctrl.write_volatile(0x1);

	// Set baud rate (clock divider)
	xip_baudr.write_volatile(4);

	// Enable XIP with value described above.
	xip_ctrlr0.write_volatile(CTRLR0);
	xip_spi_ctrlr0.write_volatile(SPI_CTRLR0);

	// Set NDF -> 0
	xip_ctrlr1.write_volatile(0);

	// Enable SSI
	xip_ssienr.write_volatile(1);

	// Enable cache (again)
	xip_ctrl.write_volatile(0x1);

	// Test read
	let _a = (0x1000_0200 as *const u32).read_volatile();
}}

#[inline(always)]
fn nop() {
	unsafe {
		asm!("NOP", options(nomem, nostack));
	}
}

#[panic_handler]
fn panic( _info: &PanicInfo) -> ! {
	loop {}
}
