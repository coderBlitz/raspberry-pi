use crate::consts::all::{self, CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC, XOSC_STATUS_ENABLED_BIT, XOSC_STATUS_STABLE_BIT};
use crate::registers::Register;

static XOSC_CTRL: Register = unsafe { Register::new(all::XOSC_CTRL) };
static XOSC_STATUS: Register = unsafe { Register::new(all::XOSC_STATUS) };
static XOSC_STARTUP: Register = unsafe { Register::new(all::XOSC_STARTUP) };

static CLK_REF_CTRL: Register = unsafe { Register::new(all::CLOCKS_CLK_REF_CTRL) };
static CLK_REF_SELECTED: Register = unsafe { Register::new(all::CLOCKS_CLK_REF_SELECTED) };

static CLK_SYS_CTRL: Register = unsafe { Register::new(all::CLOCKS_CLK_SYS_CTRL) };
static CLK_SYS_SELECTED: Register = unsafe { Register::new(all::CLOCKS_CLK_SYS_SELECTED) };

pub fn enable_xosc() {
	// Set startup delay to 10ms to be safe (~470 cycles @ 12MHz)
	XOSC_STARTUP.set(470);

	// Enable XOSC
	XOSC_CTRL.set(all::XOSC_CTRL_ENABLE_ENABLE);

	// Wait for enable
	while XOSC_STATUS.get() & XOSC_STATUS_ENABLED_BIT == 0 {}

	// Wait for stable
	while XOSC_STATUS.get() & XOSC_STATUS_STABLE_BIT == 0 {}
}

pub fn ref_to_xosc() {
	// Switch ref
	CLK_REF_CTRL.set(CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC);

	// Wait for the switch
	while CLK_REF_SELECTED.get() == 0 {}
}