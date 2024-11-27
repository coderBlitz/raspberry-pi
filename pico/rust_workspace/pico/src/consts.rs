#![allow(dead_code)]

// TODO: Macro to auto-increment constants from base register address.

pub mod clocks;
pub mod gpio;
pub mod pll;
pub mod psm;
pub mod resets;
pub mod watchdog;
pub mod xosc;

// Export all constants in one mod for ease of reference.
pub mod all {
	pub use super::*;
	pub use super::clocks::*;
	pub use super::gpio::*;
	pub use super::pll::*;
	pub use super::psm::*;
	pub use super::resets::*;
	pub use super::watchdog::*;
	pub use super::xosc::*;
}

/* ROM addresses
*/
pub const ROM_BASE: u32 = 0x0000_0000;
pub const ROM_INIT_SP: u32 = ROM_BASE;
pub const ROM_BOOT_RESET_FN: u32 = ROM_BASE + 0x4;
pub const ROM_BOOT_NMI_FN: u32 = ROM_BASE + 0x8;
pub const ROM_BOOT_FAULT_FN: u32 = ROM_BASE + 0xC;
pub const ROM_BOOT_MAGIC: u32 = ROM_BASE + 0x10;
pub const ROM_BOOT_VER: u32 = ROM_BASE + 0x13;
pub const ROM_FUNC_TABLE: u32 = ROM_BASE + 0x14;
pub const ROM_DATA_TABLE: u32 = ROM_BASE + 0x16;
pub const ROM_TABLE_LOOKUP_FN: u32 = ROM_BASE + 0x18;

/* XIP
*/
pub const XIP_BASE: u32 = 0x1000_0000;
pub const XIP_NOALLOC_BASE: u32 = 0x1100_0000;
pub const XIP_NOCACHE_BASE: u32 = 0x1200_0000;
pub const XIP_NOCACHE_NOALLOC_BASE: u32 = 0x1300_0000;
pub const XIP_CTRL_BASE: u32 = 0x1400_0000;
pub const XIP_SRAM_BASE: u32 = 0x1500_0000;
pub const XIP_SRAM_END: u32 = 0x1500_4000;

/* XIP SSI addresses */
pub const XIP_SSI_BASE: u32 = 0x1800_0000;
pub const XIP_SSI_CTRLR0: u32 = XIP_SSI_BASE;
pub const XIP_SSI_CTRLR1: u32 = XIP_SSI_BASE + 0x04;
pub const XIP_SSI_SSIENR: u32 = XIP_SSI_BASE + 0x08;
pub const XIP_SSI_BAUDR: u32 = XIP_SSI_BASE + 0x14;
pub const XIP_SSI_SPI_CTRLR0: u32 = XIP_SSI_BASE + 0xf4;

/* SRAM
*/
pub const SRAM_BASE: u32 = 0x2000_0000;
/// SRAM0-3 striped
pub const SRAM_STRIPED_BASE: u32 = SRAM_BASE;
pub const SRAM_STRIPED_END: u32 = 0x2004_0000;
/// SRAM4 is always non-striped.
pub const SRAM4_BASE: u32 = 0x2004_0000;
/// SRAM5 is always non-striped.
pub const SRAM5_BASE: u32 = 0x2004_1000;
pub const SRAM_END: u32 = 0x2004_2000;

/// Non-striped alias of SRAM0
pub const SRAM0_BASE: u32 = 0x2100_0000;
/// Non-striped alias of SRAM1
pub const SRAM1_BASE: u32 = 0x2101_0000;
/// Non-striped alias of SRAM2
pub const SRAM2_BASE: u32 = 0x2102_0000;
/// Non-striped alias of SRAM3
pub const SRAM3_BASE: u32 = 0x2103_0000;


/* APB peripherals
*/
pub const SYSINFO_BASE: u32 = 0x4000_0000;
pub const SYSCFG_BASE: u32 = 0x4000_4000;

/* Clock addresses */
pub const CLOCKS_BASE: u32 = 0x4000_8000;
pub const CLOCKS_CLK_REF_CTRL: u32 = CLOCKS_BASE + 0x30;
pub const CLOCKS_CLK_REF_DIV: u32 = CLOCKS_BASE + 0x34;
pub const CLOCKS_CLK_REF_SELECTED: u32 = CLOCKS_BASE + 0x38;
pub const CLOCKS_CLK_SYS_CTRL: u32 = CLOCKS_BASE + 0x3C;
pub const CLOCKS_CLK_SYS_DIV: u32 = CLOCKS_BASE + 0x40;
pub const CLOCKS_CLK_SYS_SELECTED: u32 = CLOCKS_BASE + 0x44;

/* Resets addresses */
pub const RESETS_BASE: u32 = 0x4000_C000;
pub const RESETS_RESET: u32 = RESETS_BASE + 0x0;
pub const RESETS_WDSEL: u32 = RESETS_BASE + 0x4;
pub const RESETS_RESET_DONE: u32 = RESETS_BASE + 0x8;

/* Power-on state machine (PSM) addresses */
pub const PSM_BASE: u32 = 0x4001_0000;
pub const PSM_FRCE_ON: u32 = PSM_BASE + 0x0;
pub const PSM_FRCE_OFF: u32 = PSM_BASE + 0x4;
pub const PSM_WDSEL: u32 = PSM_BASE + 0x8;
pub const PSM_DONE: u32 = PSM_BASE + 0xC;

/* IO bank addresses (GPIO, etc.) */
pub const IO_BANK0_BASE: u32 = 0x4001_4000;
pub const GPIO0_STATUS: u32 = IO_BANK0_BASE + 0x000;
pub const GPIO0_CTRL: u32 = IO_BANK0_BASE + 0x004;
pub const GPIO1_STATUS: u32 = IO_BANK0_BASE + 0x008;
pub const GPIO1_CTRL: u32 = IO_BANK0_BASE + 0x00C;
pub const GPIO2_STATUS: u32 = IO_BANK0_BASE + 0x010;
pub const GPIO2_CTRL: u32 = IO_BANK0_BASE + 0x014;
pub const GPIO3_STATUS: u32 = IO_BANK0_BASE + 0x018;
pub const GPIO3_CTRL: u32 = IO_BANK0_BASE + 0x01C;
pub const GPIO4_STATUS: u32 = IO_BANK0_BASE + 0x020;
pub const GPIO4_CTRL: u32 = IO_BANK0_BASE + 0x024;
pub const GPIO5_STATUS: u32 = IO_BANK0_BASE + 0x028;
pub const GPIO5_CTRL: u32 = IO_BANK0_BASE + 0x02C;
pub const GPIO6_STATUS: u32 = IO_BANK0_BASE + 0x030;
pub const GPIO6_CTRL: u32 = IO_BANK0_BASE + 0x034;
pub const GPIO7_STATUS: u32 = IO_BANK0_BASE + 0x038;
pub const GPIO7_CTRL: u32 = IO_BANK0_BASE + 0x03C;
pub const GPIO8_STATUS: u32 = IO_BANK0_BASE + 0x040;
pub const GPIO8_CTRL: u32 = IO_BANK0_BASE + 0x044;
pub const GPIO9_STATUS: u32 = IO_BANK0_BASE + 0x048;
pub const GPIO9_CTRL: u32 = IO_BANK0_BASE + 0x04C;
pub const GPIO10_STATUS: u32 = IO_BANK0_BASE + 0x050;
pub const GPIO10_CTRL: u32 = IO_BANK0_BASE + 0x054;
pub const GPIO11_STATUS: u32 = IO_BANK0_BASE + 0x058;
pub const GPIO11_CTRL: u32 = IO_BANK0_BASE + 0x05C;
pub const GPIO12_STATUS: u32 = IO_BANK0_BASE + 0x060;
pub const GPIO12_CTRL: u32 = IO_BANK0_BASE + 0x064;
pub const GPIO13_STATUS: u32 = IO_BANK0_BASE + 0x068;
pub const GPIO13_CTRL: u32 = IO_BANK0_BASE + 0x06C;
pub const GPIO14_STATUS: u32 = IO_BANK0_BASE + 0x070;
pub const GPIO14_CTRL: u32 = IO_BANK0_BASE + 0x074;
pub const GPIO15_STATUS: u32 = IO_BANK0_BASE + 0x078;
pub const GPIO15_CTRL: u32 = IO_BANK0_BASE + 0x07C;
pub const GPIO16_STATUS: u32 = IO_BANK0_BASE + 0x080;
pub const GPIO16_CTRL: u32 = IO_BANK0_BASE + 0x084;
pub const GPIO17_STATUS: u32 = IO_BANK0_BASE + 0x088;
pub const GPIO17_CTRL: u32 = IO_BANK0_BASE + 0x08C;
pub const GPIO18_STATUS: u32 = IO_BANK0_BASE + 0x090;
pub const GPIO18_CTRL: u32 = IO_BANK0_BASE + 0x094;
pub const GPIO19_STATUS: u32 = IO_BANK0_BASE + 0x098;
pub const GPIO19_CTRL: u32 = IO_BANK0_BASE + 0x09C;
pub const GPIO20_STATUS: u32 = IO_BANK0_BASE + 0x0A0;
pub const GPIO20_CTRL: u32 = IO_BANK0_BASE + 0x0A4;
pub const GPIO21_STATUS: u32 = IO_BANK0_BASE + 0x0A8;
pub const GPIO21_CTRL: u32 = IO_BANK0_BASE + 0x0AC;
pub const GPIO22_STATUS: u32 = IO_BANK0_BASE + 0x0B0;
pub const GPIO22_CTRL: u32 = IO_BANK0_BASE + 0x0B4;
pub const GPIO23_STATUS: u32 = IO_BANK0_BASE + 0x0B8;
pub const GPIO23_CTRL: u32 = IO_BANK0_BASE + 0x0BC;
pub const GPIO24_STATUS: u32 = IO_BANK0_BASE + 0x0C0;
pub const GPIO24_CTRL: u32 = IO_BANK0_BASE + 0x0C4;
pub const GPIO25_STATUS: u32 = IO_BANK0_BASE + 0x0C8;
pub const GPIO25_CTRL: u32 = IO_BANK0_BASE + 0x0CC; // LED pin
pub const GPIO26_STATUS: u32 = IO_BANK0_BASE + 0x0D0;
pub const GPIO26_CTRL: u32 = IO_BANK0_BASE + 0x0D4;
pub const GPIO27_STATUS: u32 = IO_BANK0_BASE + 0x0D8;
pub const GPIO27_CTRL: u32 = IO_BANK0_BASE + 0x0DC;
pub const GPIO28_STATUS: u32 = IO_BANK0_BASE + 0x0E0;
pub const GPIO28_CTRL: u32 = IO_BANK0_BASE + 0x0E4;
pub const GPIO29_STATUS: u32 = IO_BANK0_BASE + 0x0E8;
pub const GPIO29_CTRL: u32 = IO_BANK0_BASE + 0x0EC;

/* IO QSPI addresses */
pub const IO_QSPI_BASE: u32 = 0x4001_8000;
pub const GPIO_QSPI_SCLK_STATUS: u32 = IO_QSPI_BASE + 0x000;
pub const GPIO_QSPI_SCLK_CTRL: u32 = IO_QSPI_BASE + 0x004;
pub const GPIO_QSPI_SS_STATUS: u32 = IO_QSPI_BASE + 0x008;
pub const GPIO_QSPI_SS_CTRL: u32 = IO_QSPI_BASE + 0x00C;
pub const GPIO_QSPI_SD0_STATUS: u32 = IO_QSPI_BASE + 0x010;
pub const GPIO_QSPI_SD0_CTRL: u32 = IO_QSPI_BASE + 0x014;
pub const GPIO_QSPI_SD1_STATUS: u32 = IO_QSPI_BASE + 0x018;
pub const GPIO_QSPI_SD1_CTRL: u32 = IO_QSPI_BASE + 0x01C;
pub const GPIO_QSPI_SD2_STATUS: u32 = IO_QSPI_BASE + 0x020;
pub const GPIO_QSPI_SD2_CTRL: u32 = IO_QSPI_BASE + 0x024;
pub const GPIO_QSPI_SD3_STATUS: u32 = IO_QSPI_BASE + 0x028;
pub const GPIO_QSPI_SD3_CTRL: u32 = IO_QSPI_BASE + 0x02C;

/* Pads IO bank addresses */
pub const PADS_BANK0_BASE: u32 = 0x4001_C000;
pub const PADS_BANK0_GPIO25: u32 = PADS_BANK0_BASE + 0x68;

/* Crystal oscillator (XOSC) addresses */
pub const XOSC_BASE: u32 = 0x4002_4000;
pub const XOSC_CTRL: u32 = XOSC_BASE;
pub const XOSC_STATUS: u32 = XOSC_BASE + 0x4;
pub const XOSC_DORMANT: u32 = XOSC_BASE + 0x8;
pub const XOSC_STARTUP: u32 = XOSC_BASE + 0xC;
pub const XOSC_COUNT: u32 = XOSC_BASE + 0x1C;

/* Phase-locked loop (PLL) sys addreses */
pub const PLL_SYS_BASE: u32 = 0x4002_8000;
pub const PLL_SYS_CS: u32 = PLL_SYS_BASE;
pub const PLL_SYS_PWR: u32 = PLL_SYS_BASE + 0x4;
pub const PLL_SYS_FBDIV_INT: u32 = PLL_SYS_BASE + 0x8;
pub const PLL_SYS_PRIM: u32 = PLL_SYS_BASE + 0xC;

/* Phase-locked loop (PLL) USB addreses */
pub const PLL_USB_BASE: u32 = 0x4002_C000;

/* Bus control addresses */
pub const BUSCTRL_BASE: u32 = 0x4003_0000;

/* UART addresses */
pub const UART0_BASE: u32 = 0x4003_4000;
pub const UART1_BASE: u32 = 0x4003_8000;

/* SPI addresses */
pub const SPI0_BASE: u32 = 0x4003_C000;
pub const SPI1_BASE: u32 = 0x4004_0000;

/* I2C addresses */
pub const I2C0_BASE: u32 = 0x4004_4000;
pub const I2C1_BASE: u32 = 0x4004_8000;

/* Analog-Digital controller addresses */
pub const ADC_BASE: u32 = 0x4004_C000;

/* Pulse-width modulation (PWM) addresses */
pub const PWM_BASE: u32 = 0x4005_0000;

/* Timer addresses */
pub const TIMER_BASE: u32 = 0x4005_4000;

/* Watchdog addresses */
pub const WATCHDOG_BASE: u32 = 0x4005_8000;
pub const WATCHDOG_CTRL: u32 = WATCHDOG_BASE;
pub const WATCHDOG_LOAD: u32 = WATCHDOG_BASE + 0x4;
pub const WATCHDOG_REASON: u32 = WATCHDOG_BASE + 0x8;
pub const WATCHDOG_SCRATCH0: u32 = WATCHDOG_BASE + 0xC;
pub const WATCHDOG_SCRATCH1: u32 = WATCHDOG_BASE + 0x10;
pub const WATCHDOG_SCRATCH2: u32 = WATCHDOG_BASE + 0x14;
pub const WATCHDOG_SCRATCH3: u32 = WATCHDOG_BASE + 0x18;
pub const WATCHDOG_SCRATCH4: u32 = WATCHDOG_BASE + 0x1C;
pub const WATCHDOG_SCRATCH5: u32 = WATCHDOG_BASE + 0x20;
pub const WATCHDOG_SCRATCH6: u32 = WATCHDOG_BASE + 0x24;
pub const WATCHDOG_SCRATCH7: u32 = WATCHDOG_BASE + 0x28;
pub const WATCHDOG_TICK: u32 = WATCHDOG_BASE + 0x2C;

/* RTC addresses */
pub const RTC_BASE: u32 = 0x4005_C000;

/* Ring oscillator (ROSC) addresses */
pub const ROSC_BASE: u32 = 0x4006_0000;

/* Voltage and chip reset addresses */
pub const VREG_AND_CHIP_RESET_BASE: u32 = 0x4006_4000;

/* Testbench manager addresses */
pub const TBMAN_BASE: u32 = 0x4006_C000;

/* AHB-Lite peripherals
*/
/* Direct memory access (DMA) addresses */
pub const DMA_BASE: u32 = 0x5000_0000;

/* USB registers */
pub const USBCTRL_BASE: u32 = 0x5010_0000;
pub const USBCTRL_DPRAM_BASE: u32 = USBCTRL_BASE;
pub const USBCTRL_REGS_BASE: u32 = 0x5011_0000;

/* Programmable IO (PIO) addresses */
pub const PIO0_BASE: u32 = 0x5020_0000;
pub const PIO1_BASE: u32 = 0x5030_0000;

/* XIP Aux addresses */
pub const XIP_AUX_BASE: u32 = 0x5040_0000;

/* IOPORT peripherals
*/
/* Single-cycle IO (SIO) addresses */
pub const SIO_BASE: u32 = 0xD000_0000;

/* Cortex-M0+ internal peripherals
*/
pub const PPB_BASE: u32 = 0xE000_0000;