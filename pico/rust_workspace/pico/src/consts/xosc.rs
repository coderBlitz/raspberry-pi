/* XOSC.CTRL register constants */
pub const XOSC_CTRL_ENABLE_SHIFT: usize = 12;
pub const XOSC_CTRL_ENABLE_BITS: u32 = 0xFFF << XOSC_CTRL_ENABLE_SHIFT;
pub const XOSC_CTRL_ENABLE_DISABLE: u32 = 0xD1E << XOSC_CTRL_ENABLE_SHIFT;
pub const XOSC_CTRL_ENABLE_ENABLE: u32 = 0xFAB << XOSC_CTRL_ENABLE_SHIFT;

pub const XOSC_CTRL_FREQ_RANGE_SHIFT: usize = 0;
pub const XOSC_CTRL_FREQ_RANGE_BITS: u32 = 0xFFF;
pub const XOSC_CTRL_FREQ_RANGE_1_15MHZ: u32 = 0xAA0;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_1: u32 = 0xAA1;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_2: u32 = 0xAA2;
pub const XOSC_CTRL_FREQ_RANGE_RESERVED_3: u32 = 0xAA3;

/* XOSC.STATUS register constants */
pub const XOSC_STATUS_STABLE_BIT: u32 = 1 << 31;
pub const XOSC_STATUS_BADWRITE_BIT: u32 = 1 << 24;
pub const XOSC_STATUS_ENABLED_BIT: u32 = 1 << 12;

pub const XOSC_STATUS_FREQ_RANGE_SHIFT: usize = 0;
pub const XOSC_STATUS_FREQ_RANGE_BITS: u32 = 0x3;
pub const XOSC_STATUS_FREQ_RANGE_1_15MHZ: u32 = 0x0 << XOSC_STATUS_FREQ_RANGE_SHIFT;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_1: u32 = 0x1 << XOSC_STATUS_FREQ_RANGE_SHIFT;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_2: u32 = 0x2 << XOSC_STATUS_FREQ_RANGE_SHIFT;
pub const XOSC_STATUS_FREQ_RANGE_RESERVED_3: u32 = 0x3 << XOSC_STATUS_FREQ_RANGE_SHIFT;

/* XOSC.DORMANT register constants */
pub const XOSC_DORMANT_DORMANT: u32 = 0x636F6D61;
pub const XOSC_DORMANT_WAKE: u32 = 0x77616B65;

/* XOSC.STARTUP register constants */
pub const XOSC_STARTUP_X4_BIT: u32 = 1 << 20;
pub const XOSC_STARTUP_DELAY_SHIFT: usize = 0;
/// Delay in multiples of 256. Resets to `0x00C4`.
pub const XOSC_STARTUP_DELAY_BITS: u32 = 0x3FFF << XOSC_STARTUP_DELAY_SHIFT;

/* XOSC.COUNT register constants */
pub const XOSC_COUNT_SHIFT: usize = 0;
pub const XOSC_COUNT_BITS: u32 = 0xF << XOSC_COUNT_SHIFT;