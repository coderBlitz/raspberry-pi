//! Processor subsystem
//!

use crate::Register;
use crate::consts::*;
use core::arch::asm;

static CPUID: Register = unsafe { Register::new(SIO_CPUID) };
static VTOR: Register = unsafe { Register::new(PPB_VTOR) };

/// Wait-for-interrupt (WFI).
pub fn wfi() { unsafe {
	asm!("wfi", options(nomem, nostack));
}}

/// Wait-for-event (WFE).
pub fn wfe() { unsafe {
	asm!("wfe", options(nomem, nostack));
}}

/// Send event (SEV).
pub fn sev() { unsafe {
	asm!("sev", options(nomem, nostack));
}}

pub fn cpu_id() -> u32 {
	CPUID.get()
}

pub fn replace_nvic_table() {
	VTOR.set(&raw const NVIC_STUFF as u32);
}

// TODO: NVIC stuff
#[repr(align(256))]
struct NvicTable([u32; 48]);

// Exception numbers (pg 192 ARMv6-M ARM):
// 1 - Reset
// 2 - NMI
// 3 - HardFault
// 4-10 - Reserved
// 11 - SVCall
// 12-13 - Reserved
// 14 - PendSV
// 15 - SysTick, optional
// 16 - External interrupt 0
// ...
// 16 + N - External interrupt N

// Exception stack pointer plus exception addresses. High bit must be set on
//  all entries otherwise first instruction causes HardFault.
// TODO: Utilize handler pointers from ROM for reset, NMI, and hard fault.
static mut NVIC_STUFF: NvicTable = NvicTable (
	[
		0x2004_0000, // SP_main (just use top of SRAM for now)
		0x8000_0000, // Reset
		0x8000_0000, // NMI
		0x8000_0000, // HardFault
		0x8000_0000, // -- Start Reserved --
		0x8000_0000,
		0x8000_0000,
		0x8000_0000,
		0x8000_0000,
		0x8000_0000,
		0x8000_0000, // -- End Reserved --
		0x8000_0000, // SVCall
		0x8000_0000, // Reserved
		0x8000_0000, // Reserved
		0x8000_0000, // PendSV
		0x8000_0000, // SysTick
		0x8000_0000, // TIMER_IRQ_0 (external 0)
		0x8000_0000, // TIMER_IRQ_1 (external 1)
		0x8000_0000, // TIMER_IRQ_2
		0x8000_0000, // TIMER_IRQ_3
		0x8000_0000, // PWM_IRQ_WRAP
		0x8000_0000, // USBCTRL_IRQ
		0x8000_0000, // XIP_IRQ
		0x8000_0000, // PIO0_IRQ_0
		0x8000_0000, // PIO0_IRQ_1
		0x8000_0000, // PIO1_IRQ_0
		0x8000_0000, // PIO1_IRQ_1
		0x8000_0000, // DMA_IRQ_0
		0x8000_0000, // DMA_IRQ_1
		0x8000_0000, // IO_IRQ_BANK0
		0x8000_0000, // IO_IRQ_QSPI
		0x8000_0000, // SIO_IRQ_PROC0
		0x8000_0000, // SIO_IRQ_PROC1
		0x8000_0000, // CLOCKS_IRQ
		0x8000_0000, // SPI0_IRQ
		0x8000_0000, // SPI1_IRQ
		0x8000_0000, // UART0_IRQ
		0x8000_0000, // UART1_IRQ
		0x8000_0000, // ADC_IRQ_FIFO
		0x8000_0000, // I2C0_IRQ
		0x8000_0000, // I2C1_IRQ
		0x8000_0000, // RTC_IRQ
		0x8000_0000, // (external 26)
		0x8000_0000, // (external 27)
		0x8000_0000, // (external 28)
		0x8000_0000, // (external 29)
		0x8000_0000, // (external 30)
		0x8000_0000, // (external 31)
	]
);