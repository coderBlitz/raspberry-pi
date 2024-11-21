/* PLL_SYS.CS */
pub const PLL_SYS_CS_LOCK_BIT: u32 = 1 << 31;
pub const PLL_SYS_CS_BYPASS_BIT: u32 = 1 << 8;
pub const PLL_SYS_CS_REFDIV_SHIFT: usize = 0;
pub const PLL_SYS_CS_REFDIV_BITS: u32 = 0x3F << PLL_SYS_CS_REFDIV_SHIFT;

/* PLL_SYS.PWR */
pub const PLL_SYS_PWR_VCOPD_BIT: u32 = 1 << 5;
pub const PLL_SYS_PWR_POSTDIVPD_BIT: u32 = 1 << 3;
pub const PLL_SYS_PWR_DSMPD_BIT: u32 = 1 << 2;
pub const PLL_SYS_PWR_PD_BIT: u32 = 1 << 0;

/* PLL_SYS.FBDIV_INT */
pub const PLL_SYS_FBDIV_INT_SHIFT: usize = 0;
pub const PLL_SYS_FBDIV_INT_BITS: u32 = 0xFFF << PLL_SYS_FBDIV_INT_SHIFT

/* PLL_SYS.PRIM */;
pub const PLL_SYS_PRIM_POSTDIV1_SHIFT: usize = 16;
pub const PLL_SYS_PRIM_POSTDIV1_BITS: u32 = 0x7 << PLL_SYS_PRIM_POSTDIV1_SHIFT;
pub const PLL_SYS_PRIM_POSTDIV2_SHIFT: usize = 12;
pub const PLL_SYS_PRIM_POSTDIV2_BITS: u32 = 0x7 << PLL_SYS_PRIM_POSTDIV2_SHIFT;