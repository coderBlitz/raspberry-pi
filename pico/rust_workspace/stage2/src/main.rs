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
	registers::Register,
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
	//pico::rom::flash_enter_cmd_xip();
	enable_xip();

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
		const MAIN_ADDR: u32 = 0x1000_0100; // Flash load (need to get XIP working first)
		//const MAIN_ADDR: u32 = 0x2000_0100; // RAM load
		asm!(
			"mov pc, {addr}",
			addr = in(reg) MAIN_ADDR,
			options(noreturn)
		)
	}
}

/// Logic copied from bootrom source.
///
/// Documentation pg593 accurate except the "only supported in enhanced modes".
#[inline(always)]
fn enable_xip() { unsafe {
	// Make register vars
	let xip_ssienr = Register::new(XIP_SSI_SSIENR);
	let xip_ctrl = Register::new(XIP_CTRL_BASE);
	let xip_ctrlr0 = Register::new(XIP_SSI_CTRLR0);
	let xip_spi_ctrlr0 = Register::new(XIP_SSI_SPI_CTRLR0);

	// Quad-mode (0x2), data frame size 32 bits (0x1F), TMOD EEPROM read (0x3)
	//const CTRLR0: u32 = 0 | 0x2 << 21 | 31 << 16 | 0x3 << 8; // Quad-read
	const CTRLR0: u32 = 0 | 0x0 << 21 | 31 << 16 | 0x3 << 8; // Standard read

	// See Pg. 571 and 608
	// XIP_CMD = 0xa0, WAIT_CYCLES = 31/32 (0x1F) from DFS_32, INST_L = 0, ADDR_L = 32 bits ()
	//const SPI_CTRLR0: u32 = 0 | 0xa0 << 24 | 31 << 11 | 0 << 8 | 0x8 << 2; // Quad-read continuation
	const SPI_CTRLR0: u32 = 0 | 0x03 << 24 | 31 << 11 | 0x2 << 8 | 0x6 << 2; // Standard 03h

	// Enable cache
	xip_ctrl.set(0x1);

	// Disable SSI
	xip_ssienr.set(0);

	// Enable XIP with value described above.
	xip_ctrlr0.set(CTRLR0);
	xip_spi_ctrlr0.set(SPI_CTRLR0);

	// Enable SSI
	xip_ssienr.set(1);
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
