/* CLOCKS.CLK_REF_CTRL */
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_SHIFT: usize = 5;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_BITS: u32 = 0x3 << CLOCKS_CLK_REF_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_CLKSRC_PLL_USB: u32 = 0x0 << CLOCKS_CLK_REF_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_CLKSRC_GPIN0: u32 = 0x1 << CLOCKS_CLK_REF_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_AUXSRC_CLKSRC_GPIN1: u32 = 0x2 << CLOCKS_CLK_REF_CTRL_AUXSRC_SHIFT;

pub const CLOCKS_CLK_REF_CTRL_SRC_SHIFT: usize = 0;
pub const CLOCKS_CLK_REF_CTRL_SRC_BITS: usize = 0x3 << CLOCKS_CLK_REF_CTRL_SRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_SRC_ROSC_CLKSRC_PH: usize = 0x0 << CLOCKS_CLK_REF_CTRL_SRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_SRC_CLKSRC_CLK_REF_AUX: usize = 0x1 << CLOCKS_CLK_REF_CTRL_SRC_SHIFT;
pub const CLOCKS_CLK_REF_CTRL_SRC_XOSC_CLKSRC: usize = 0x2 << CLOCKS_CLK_REF_CTRL_SRC_SHIFT;

/* CLOCKS.CLK_SYS_CTRL */
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT: usize = 5;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_BITS: u32 = 0x7 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_PLL_SYS: u32 = 0x0 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_PLL_USB: u32 = 0x1 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_ROSC_CLKSRC: u32 = 0x2 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_XOSC_CLKSRC: u32 = 0x3 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_GPIN0: u32 = 0x4 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;
pub const CLOCKS_CLK_SYS_CTRL_AUXSRC_CLKSRC_GPIN1: u32 = 0x5 << CLOCKS_CLK_SYS_CTRL_AUXSRC_SHIFT;

pub const CLOCKS_CLK_SYS_CTRL_SRC_BIT: u32 = 1 << 0;
pub const CLOCKS_CLK_SYS_CTRL_SRC_CLK_REF: u32 = 0;
pub const CLOCKS_CLK_SYS_CTRL_SRC_CLKSRC_CLK_SYS_AUX: u32 = 1;