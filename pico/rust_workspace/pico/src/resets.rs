use crate::consts::all::*;
use crate::registers::Register;
use paste::paste;

static RESETS: Register = unsafe { Register::new(RESETS_BASE) };
static DONE: Register = unsafe { Register::new(RESETS_RESET_DONE) };

macro_rules! impl_resets {
	{
		$($name:ident = $bit:expr)*
	} => { $( paste! {
		#[inline(always)]
		pub fn [< enable_ $name >] () {
			RESETS.atomic_bitclear($bit);
			while DONE.read() & $bit == 0x0 {}
		}
		#[inline(always)]
		pub fn [< disable_ $name >] () {
			RESETS.atomic_bitset($bit);
			while DONE.read() & $bit == 0x1 {}
		}
	})*}
}

impl_resets! {
	usbctrl = RESETS_USBCTRL_BIT
	uart1 = RESETS_UART1_BIT
	uart0 = RESETS_UART0_BIT
	timer = RESETS_TIMER_BIT
	tbman = RESETS_TBMAN_BIT
	sysinfo = RESETS_SYSINFO_BIT
	syscfg = RESETS_SYSCFG_BIT
	spi1 = RESETS_SPI1_BIT
	spi0 = RESETS_SPI0_BIT
	rtc = RESETS_RTC_BIT
	pwm = RESETS_PWM_BIT
	pll_usb = RESETS_PLL_USB_BIT
	pll_sys = RESETS_PLL_SYS_BIT
	pio1 = RESETS_PIO1_BIT
	poi0 = RESETS_PIO0_BIT
	pads_qspi = RESETS_PADS_QSPI_BIT
	pads_bank0 = RESETS_PADS_BANK0_BIT
	jtag = RESETS_JTAG_BIT
	io_qspi = RESETS_IO_QSPI_BIT
	io_bank0 = RESETS_IO_BANK0_BIT
	i2c1 = RESETS_I2C1_BIT
	i2c0 = RESETS_I2C0_BIT
	dma = RESETS_DMA_BIT
	busctrl = RESETS_BUSCTRL_BIT
	adc = RESETS_ADC_BIT
}