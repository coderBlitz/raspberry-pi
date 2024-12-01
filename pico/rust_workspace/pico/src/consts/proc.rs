/* M0PLUS.SYST_CSR */
pub const PPB_SYST_CSR_COUNTFLAG_BIT: u32 = 1 << 16;
pub const PPB_SYST_CSR_CLKSOURCE_BIT: u32 = 1 << 2;
pub const PPB_SYST_CSR_TICKINT_BIT: u32 = 1 << 1;
pub const PPB_SYST_CSR_ENABLE_BIT: u32 = 1 << 0;

/* M0PLUS.SYST_RVR */
pub const PPB_SYST_RVR_RELOAD_BITS: u32 = 0x00FF_FFFF;

/* M0PLUS.SYST_CVR */
pub const PPB_SYST_CVR_CURRENT_BITS: u32 = 0x00FF_FFFF;

/* M0PLUS.SYST_CALIB */
pub const PPB_SYST_CALIB_NOREF_BIT: u32 = 1 << 31;
pub const PPB_SYST_CALIB_SKEW_BIT: u32 = 1 << 30;
pub const PPB_SYST_CALIB_TENMS_BITS: u32 = 0x00FF_FFFF;

/* Interrupt bits */
pub const TIMER_IRQ_0: u32 = 1 << 0;
pub const TIMER_IRQ_1: u32 = 1 << 1;
pub const TIMER_IRQ_2: u32 = 1 << 2;
pub const TIMER_IRQ_3: u32 = 1 << 3;
pub const PWM_IRQ_WRAP: u32 = 1 << 4;
pub const USBCTRL_IRQ: u32 = 1 << 5;
pub const XIP_IRQ: u32 = 1 << 6;
pub const PIO0_IRQ_0: u32 = 1 << 7;
pub const PIO0_IRQ_1: u32 = 1 << 8;
pub const PIO1_IRQ_0: u32 = 1 << 9;
pub const PIO1_IRQ_1: u32 = 1 << 10;
pub const DMA_IRQ_0: u32 = 1 << 11;
pub const DMA_IRQ_1: u32 = 1 << 12;
pub const IO_IRQ_BANK0: u32 = 1 << 13;
pub const IO_IRQ_QSPI: u32 = 1 << 14;
pub const SIO_IRQ_PROC0: u32 = 1 << 15;
pub const SIO_IRQ_PROC1: u32 = 1 << 16;
pub const CLOCKS_IRQ: u32 = 1 << 17;
pub const SPI0_IRQ: u32 = 1 << 18;
pub const SPI1_IRQ: u32 = 1 << 19;
pub const UART0_IRQ: u32 = 1 << 20;
pub const UART1_IRQ: u32 = 1 << 21;
pub const ADC_IRQ_FIFO: u32 = 1 << 22;
pub const I2C0_IRQ: u32 = 1 << 23;
pub const I2C1_IRQ: u32 = 1 << 24;
pub const RTC_IRQ: u32 = 1 << 25;
pub const IRQ_26: u32 = 1 << 26;
pub const IRQ_27: u32 = 1 << 27;
pub const IRQ_28: u32 = 1 << 28;
pub const IRQ_29: u32 = 1 << 29;
pub const IRQ_30: u32 = 1 << 30;
pub const IRQ_31: u32 = 1 << 31;