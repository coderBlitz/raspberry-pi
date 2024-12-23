#![allow(dead_code)]

pub mod clocks;
pub mod gpio;
pub mod pll;
pub mod proc;
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
	pub use super::proc::*;
	pub use super::psm::*;
	pub use super::resets::*;
	pub use super::watchdog::*;
	pub use super::xosc::*;
}

/// Macro to reduce syntax needed to declare a group of register address
///  constants.
///
/// Token-muncher which increments an offset value by 0x4 for every ident
///  given.
macro_rules! consts_block {
	{
		$base:ident = $addr:literal
	} => {
		pub const $base: u32 = $addr;
	};
	{
		$base:ident = $addr:literal
		$($rem:tt)*
	} => {
		consts_block! {
			$base = $addr
		}
		consts_block! {
			$base
			0x0 =>
			$($rem)*
		}
	};
	{
		$base:ident
		$offset:expr => $next:ident
		$($rem:tt)*
	} => {
		pub const $next: u32 = $base + $offset;
		consts_block! {
			$base
			$offset + 0x4 => $($rem)*
		}
	};
	// If desired to skip, use `#[skip]` to skip the next increment or
	//  `#[skip = N]` to skip the next N increments.
	{
		$base:ident
		$offset:expr => #[skip $(= $times:literal)?]
		$($rem:tt)*
	} => {
		consts_block! {
			$base
			$offset + 0x4 $(* $times)? => $($rem)*
		}
	};
	{
		$base:ident
		$offset:expr =>
	} => {}; // Leftover
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
consts_block! {
	RESETS_BASE = 0x4000_C000
	RESETS_RESET
	RESETS_WDSEL
	RESETS_RESET_DONE
}

/* Power-on state machine (PSM) addresses */
consts_block! {
	PSM_BASE = 0x4001_0000
	PSM_FRCE_ON
	PSM_FRCE_OFF
	PSM_WDSEL
	PSM_DONE
}

/* IO bank addresses (GPIO, etc.) */
consts_block! {
	IO_BANK0_BASE = 0x4001_4000
	GPIO0_STATUS
	GPIO0_CTRL
	GPIO1_STATUS
	GPIO1_CTRL
	GPIO2_STATUS
	GPIO2_CTRL
	GPIO3_STATUS
	GPIO3_CTRL
	GPIO4_STATUS
	GPIO4_CTRL
	GPIO5_STATUS
	GPIO5_CTRL
	GPIO6_STATUS
	GPIO6_CTRL
	GPIO7_STATUS
	GPIO7_CTRL
	GPIO8_STATUS
	GPIO8_CTRL
	GPIO9_STATUS
	GPIO9_CTRL
	GPIO10_STATUS
	GPIO10_CTRL
	GPIO11_STATUS
	GPIO11_CTRL
	GPIO12_STATUS
	GPIO12_CTRL
	GPIO13_STATUS
	GPIO13_CTRL
	GPIO14_STATUS
	GPIO14_CTRL
	GPIO15_STATUS
	GPIO15_CTRL
	GPIO16_STATUS
	GPIO16_CTRL
	GPIO17_STATUS
	GPIO17_CTRL
	GPIO18_STATUS
	GPIO18_CTRL
	GPIO19_STATUS
	GPIO19_CTRL
	GPIO20_STATUS
	GPIO20_CTRL
	GPIO21_STATUS
	GPIO21_CTRL
	GPIO22_STATUS
	GPIO22_CTRL
	GPIO23_STATUS
	GPIO23_CTRL
	GPIO24_STATUS
	GPIO24_CTRL
	GPIO25_STATUS
	GPIO25_CTRL
	GPIO26_STATUS
	GPIO26_CTRL
	GPIO27_STATUS
	GPIO27_CTRL
	GPIO28_STATUS
	GPIO28_CTRL
	GPIO29_STATUS
	GPIO29_CTRL
}

/* IO QSPI addresses */
consts_block! {
	IO_QSPI_BASE = 0x4001_8000
	GPIO_QSPI_SCLK_STATUS
	GPIO_QSPI_SCLK_CTRL
	GPIO_QSPI_SS_STATUS
	GPIO_QSPI_SS_CTRL
	GPIO_QSPI_SD0_STATUS
	GPIO_QSPI_SD0_CTRL
	GPIO_QSPI_SD1_STATUS
	GPIO_QSPI_SD1_CTRL
	GPIO_QSPI_SD2_STATUS
	GPIO_QSPI_SD2_CTRL
	GPIO_QSPI_SD3_STATUS
	GPIO_QSPI_SD3_CTRL
	INTR
	PROC0_INTE
	PROC0_INTF
	PROC0_INTS
	PROC1_INTE
	PROC1_INTF
	PROC1_INTS
	DORMANT_WAKE_INTE
	DORMANT_WAKE_INTF
	DORMANT_WAKE_INTS
}


/* Pads IO bank addresses */
consts_block! {
	PADS_BANK0_BASE = 0x4001_C000
	PADS_BANK0_VOLTAGE_SELECT
	PADS_BANK0_GPIO0
	PADS_BANK0_GPIO1
	PADS_BANK0_GPIO2
	PADS_BANK0_GPIO3
	PADS_BANK0_GPIO4
	PADS_BANK0_GPIO5
	PADS_BANK0_GPIO6
	PADS_BANK0_GPIO7
	PADS_BANK0_GPIO8
	PADS_BANK0_GPIO9
	PADS_BANK0_GPIO10
	PADS_BANK0_GPIO11
	PADS_BANK0_GPIO12
	PADS_BANK0_GPIO13
	PADS_BANK0_GPIO14
	PADS_BANK0_GPIO15
	PADS_BANK0_GPIO16
	PADS_BANK0_GPIO17
	PADS_BANK0_GPIO18
	PADS_BANK0_GPIO19
	PADS_BANK0_GPIO20
	PADS_BANK0_GPIO21
	PADS_BANK0_GPIO22
	PADS_BANK0_GPIO23
	PADS_BANK0_GPIO24
	PADS_BANK0_GPIO25
	PADS_BANK0_GPIO26
	PADS_BANK0_GPIO27
	PADS_BANK0_GPIO28
	PADS_BANK0_GPIO29
	PADS_BANK0_SWCLK
	PADS_BANK0_SWD
}

/* Pads QSPI bank addresses */
consts_block! {
	PADS_QSPI_BASE = 0x4001_C000
	PADS_QSPI_VOLTAGE_SELECT
	PADS_QSPI_SCLK
	PADS_QSPI_SD0
	PADS_QSPI_SD1
	PADS_QSPI_SD2
	PADS_QSPI_SD3
	PADS_QSPI_SS
}

/* Crystal oscillator (XOSC) addresses */
consts_block! {
	XOSC_BASE = 0x4002_4000
	XOSC_CTRL
	XOSC_STATUS
	XOSC_DORMANT
	XOSC_STARTUP
}
pub const XOSC_COUNT: u32 = XOSC_BASE + 0x1C;

/* Phase-locked loop (PLL) sys addreses */
consts_block! {
	PLL_SYS_BASE = 0x4002_8000
	PLL_SYS_CS
	PLL_SYS_PWR
	PLL_SYS_FBDIV_INT
	PLL_SYS_PRIM
}

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
consts_block! {
	TIMER_BASE = 0x4005_4000
	TIMER_TIMEHW
	TIMER_TIMELW
	TIMER_TIMEHR
	TIMER_TIMELR
	TIMER_ALARM0
	TIMER_ALARM1
	TIMER_ALARM2
	TIMER_ALARM3
	TIMER_ARMED
	TIMER_TIMERAWH
	TIMER_TIMERAWL
	TIMER_DBGPAUSE
	TIMER_PAUSE
	TIMER_INTR
	TIMER_INTE
	TIMER_INTF
	TIMER_INTS
}

/* Watchdog addresses */
consts_block! {
	WATCHDOG_BASE = 0x4005_8000
	WATCHDOG_CTRL
	WATCHDOG_LOAD
	WATCHDOG_REASON
	WATCHDOG_SCRATCH0
	WATCHDOG_SCRATCH1
	WATCHDOG_SCRATCH2
	WATCHDOG_SCRATCH3
	WATCHDOG_SCRATCH4
	WATCHDOG_SCRATCH5
	WATCHDOG_SCRATCH6
	WATCHDOG_SCRATCH7
	WATCHDOG_TICK
}

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
consts_block! {
	SIO_BASE = 0xD000_0000
	SIO_CPUID
	SIO_GPIO_IN
	SIO_GPIO_HI_IN
	#[skip]
	SIO_GPIO_OUT
	SIO_GPIO_OUT_SET
	SIO_GPIO_OUT_CLR
	SIO_GPIO_OUT_XOR
	SIO_GPIO_OE
	SIO_GPIO_OE_SET
	SIO_GPIO_OE_CLR
	SIO_GPIO_OE_XOR
	SIO_GPIO_HI_OUT
	SIO_GPIO_HI_OUT_SET
	SIO_GPIO_HI_OUT_CLR
	SIO_GPIO_HI_OUT_XOR
	SIO_GPIO_HI_OE
	SIO_GPIO_HI_OE_SET
	SIO_GPIO_HI_OE_CLR
	SIO_GPIO_HI_OE_XOR
	SIO_FIFO_ST
	SIO_FIFO_WR
	SIO_FIFO_RD
	SIO_SPINLOCK_ST
	SIO_DIV_UDIVIDEND
	SIO_DIV_UDIVISOR
	SIO_DIV_SDIVIDEND
	SIO_DIV_SDIVISOR
	SIO_DIV_QUOTIENT
	SIO_DIV_REMAINDER
	SIO_DIV_CSR
	SIO_INTERP0_ACCUM0
	SIO_INTERP0_ACCUM1
	SIO_INTERP0_BASE0
	SIO_INTERP0_BASE1
	SIO_INTERP0_BASE2
	SIO_INTERP0_POP_LANE0
	SIO_INTERP0_POP_LANE1
	SIO_INTERP0_POP_FULL
	SIO_INTERP0_PEEK_LANE0
	SIO_INTERP0_PEEK_LANE1
	SIO_INTERP0_PEEK_FULL
	SIO_INTERP0_CTRL_LANE0
	SIO_INTERP0_CTRL_LANE1
	SIO_INTERP0_ACCUM0_ADD
	SIO_INTERP0_ACCUM1_ADD
	SIO_INTERP0_BASE_1AND0
	SIO_INTERP1_ACCUM0
	SIO_INTERP1_ACCUM1
	SIO_INTERP1_BASE0
	SIO_INTERP1_BASE1
	SIO_INTERP1_BASE2
	SIO_INTERP1_POP_LANE0
	SIO_INTERP1_POP_LANE1
	SIO_INTERP1_POP_FULL
	SIO_INTERP1_PEEK_LANE0
	SIO_INTERP1_PEEK_LANE1
	SIO_INTERP1_PEEK_FULL
	SIO_INTERP1_CTRL_LANE0
	SIO_INTERP1_CTRL_LANE1
	SIO_INTERP1_ACCUM0_ADD
	SIO_INTERP1_ACCUM1_ADD
	SIO_INTERP1_BASE_1AND0
	SIO_SPINLOCK0
	SIO_SPINLOCK1
	SIO_SPINLOCK2
	SIO_SPINLOCK3
	SIO_SPINLOCK4
	SIO_SPINLOCK5
	SIO_SPINLOCK6
	SIO_SPINLOCK7
	SIO_SPINLOCK8
	SIO_SPINLOCK9
	SIO_SPINLOCK10
	SIO_SPINLOCK11
	SIO_SPINLOCK12
	SIO_SPINLOCK13
	SIO_SPINLOCK14
	SIO_SPINLOCK15
	SIO_SPINLOCK16
	SIO_SPINLOCK17
	SIO_SPINLOCK18
	SIO_SPINLOCK19
	SIO_SPINLOCK20
	SIO_SPINLOCK21
	SIO_SPINLOCK22
	SIO_SPINLOCK23
	SIO_SPINLOCK24
	SIO_SPINLOCK25
	SIO_SPINLOCK26
	SIO_SPINLOCK27
	SIO_SPINLOCK28
	SIO_SPINLOCK29
	SIO_SPINLOCK30
	SIO_SPINLOCK31
}

/* Cortex-M0+ internal peripherals
*/
pub const PPB_BASE: u32 = 0xE000_0000;
pub const PPB_SYST_CSR: u32 = PPB_BASE + 0xE010;
pub const PPB_SYST_RVR: u32 = PPB_BASE + 0xE014;
pub const PPB_SYST_CVR: u32 = PPB_BASE + 0xE018;
pub const PPB_SYST_CALIB: u32 = PPB_BASE + 0xE01C;
pub const PPB_NVIC_ISER: u32 = PPB_BASE + 0xE100;
pub const PPB_NVIC_ICER: u32 = PPB_BASE + 0xE180;
pub const PPB_NVIC_ISPR: u32 = PPB_BASE + 0xE200;
pub const PPB_NVIC_ICPR: u32 = PPB_BASE + 0xE280;
pub const PPB_NVIC_IPR0: u32 = PPB_BASE + 0xE400;
pub const PPB_NVIC_IPR1: u32 = PPB_BASE + 0xE404;
pub const PPB_NVIC_IPR2: u32 = PPB_BASE + 0xE408;
pub const PPB_NVIC_IPR3: u32 = PPB_BASE + 0xE40C;
pub const PPB_NVIC_IPR4: u32 = PPB_BASE + 0xE410;
pub const PPB_NVIC_IPR5: u32 = PPB_BASE + 0xE414;
pub const PPB_NVIC_IPR6: u32 = PPB_BASE + 0xE418;
pub const PPB_NVIC_IPR7: u32 = PPB_BASE + 0xE41C;
pub const PPB_CPUID: u32 = PPB_BASE + 0xED00;
pub const PPB_ICSR: u32 = PPB_BASE + 0xED04;
pub const PPB_VTOR: u32 = PPB_BASE + 0xED08;
pub const PPB_AIRCR: u32 = PPB_BASE + 0xED0C;
pub const PPB_SCR: u32 = PPB_BASE + 0xED10;
pub const PPB_CCR: u32 = PPB_BASE + 0xED14;
pub const PPB_SHPR2: u32 = PPB_BASE + 0xED1C;
pub const PPB_SHPR3: u32 = PPB_BASE + 0xED20;
pub const PPB_SHCSR: u32 = PPB_BASE + 0xED24;
pub const PPB_MPU_TYPE: u32 = PPB_BASE + 0xED90;
pub const PPB_MPU_CTRL: u32 = PPB_BASE + 0xED94;
pub const PPB_MPU_RNR: u32 = PPB_BASE + 0xED98;
pub const PPB_MPU_RBAR: u32 = PPB_BASE + 0xED9C;
pub const PPB_MPU_RASR: u32 = PPB_BASE + 0xEDA0;