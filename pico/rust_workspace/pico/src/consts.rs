#![allow(dead_code)]

pub mod gpio;

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
pub const CLOCKS_BASE: u32 = 0x4000_8000;

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

pub const GPIO25_STATUS: u32 = IO_BANK0_BASE + 0x0C8; // LED pin
pub const GPIO25_CTRL: u32 = IO_BANK0_BASE + 0x0CC;

/* IO QSPI addresses */
pub const IO_QSPI_BASE: u32 = 0x4001_8000;

/* Pads IO bank addresses */
pub const PADS_BANK0_BASE: u32 = 0x4001_C000;
pub const PADS_BANK0_GPIO25: u32 = PADS_BANK0_BASE + 0x68;

/* Crystal oscillator (XOSC) addresses */
pub const XOSC_BASE: u32 = 0x4002_4000;

/* Phase-locked loop (PLL) sys addreses */
pub const PLL_SYS_BASE: u32 = 0x4002_8000;

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