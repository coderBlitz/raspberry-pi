use crate::consts::all::{self, CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC, XOSC_STATUS_ENABLED_BIT, XOSC_STATUS_STABLE_BIT};
use crate::registers::Register;

static XOSC_CTRL: Register = unsafe { Register::new(all::XOSC_CTRL) };
static XOSC_STATUS: Register = unsafe { Register::new(all::XOSC_STATUS) };
static XOSC_STARTUP: Register = unsafe { Register::new(all::XOSC_STARTUP) };

static PLL_SYS_CS: Register = unsafe { Register::new(all::PLL_SYS_CS) };
static PLL_SYS_PWR: Register = unsafe { Register::new(all::PLL_SYS_PWR) };
static PLL_SYS_FBDIV_INT: Register = unsafe { Register::new(all::PLL_SYS_FBDIV_INT) };
static PLL_SYS_PRIM: Register = unsafe { Register::new(all::PLL_SYS_PRIM) };

static CLK_REF_CTRL: Register = unsafe { Register::new(all::CLOCKS_CLK_REF_CTRL) };
static CLK_REF_SELECTED: Register = unsafe { Register::new(all::CLOCKS_CLK_REF_SELECTED) };

static CLK_SYS_CTRL: Register = unsafe { Register::new(all::CLOCKS_CLK_SYS_CTRL) };
static CLK_SYS_SELECTED: Register = unsafe { Register::new(all::CLOCKS_CLK_SYS_SELECTED) };

pub fn enable_xosc() {
	// Minimum cycles is 47, so go with 50 to be slightly safe.
	XOSC_STARTUP.set(50);

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

pub fn configure_pll() {
	// FBDIV 133
	// POSTDIV1 6 x POSTDIV2 2 = 12 POSTDIV

	// Power on PLL system
	crate::resets::enable_pll_sys();

	// Enable all PLL PD registers
	PLL_SYS_PWR.atomic_bitclear(0xFF);

	// Set FBDIV
	PLL_SYS_FBDIV_INT.set(133);

	// Set POSTDIVs
	PLL_SYS_PRIM.set(6 << all::PLL_SYS_PRIM_POSTDIV1_SHIFT | 2 << all::PLL_SYS_PRIM_POSTDIV2_SHIFT);
}

pub fn sys_to_pll() {
	// Set SYS AUX to PLL
	CLK_SYS_CTRL.atomic_bitclear(all::CLOCKS_CLK_SYS_CTRL_AUXSRC_BITS);
	CLK_SYS_CTRL.atomic_bitset(all::CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_PLL_SYS);

	// Set the glitchless to the AUX
	CLK_SYS_CTRL.atomic_bitset(all::CLOCKS_CLK_SYS_CTRL_SRC_BIT);

	// Wait for switch
	while CLK_SYS_SELECTED.get() == 0 {}
}
