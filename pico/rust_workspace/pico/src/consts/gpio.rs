/* Function constants */
pub const GPIO_F0: u32 = 0x0;
pub const GPIO_F1: u32 = 0x1;
pub const GPIO_F2: u32 = 0x2;
pub const GPIO_F3: u32 = 0x3;
pub const GPIO_F4: u32 = 0x4;
pub const GPIO_F5: u32 = 0x5;
pub const GPIO_F6: u32 = 0x6;
pub const GPIO_F7: u32 = 0x7;

pub const GPIO_FUNC_SIO: u32 = GPIO_F5;

pub const GPIO_FUNC_NULL: u32 = 0x1F;

pub const GPIO_FUNCSEL_SHIFT: usize = 0;
pub const GPIO_FUNCSEL_BITS: u32 = 0x1F << GPIO_FUNCSEL_SHIFT;

/* IO_BANK0.GPIOXX_STATUS */
pub const IO_BANK0_GPIO_STATUS_IRQTOPROC_BIT: u32 = 1 << 26;
pub const IO_BANK0_GPIO_STATUS_IRQFROMPAD_BIT: u32 = 1 << 24;
pub const IO_BANK0_GPIO_STATUS_INTOPERI_BIT: u32 = 1 << 19;
pub const IO_BANK0_GPIO_STATUS_INFROMPAD_BIT: u32 = 1 << 17;
pub const IO_BANK0_GPIO_STATUS_OETOPAD_BIT: u32 = 1 << 13;
pub const IO_BANK0_GPIO_STATUS_OEFROMPERI_BIT: u32 = 1 << 12;
pub const IO_BANK0_GPIO_STATUS_OUTTOPAD_BIT: u32 = 1 << 9;
pub const IO_BANK0_GPIO_STATUS_OUTFROMPERI_BIT: u32 = 1 << 8;

/* IO_BANK0.GPIOXX_CTRL */
pub const IO_BANK0_GPIO_CTRL_IRQOVER_SHIFT: usize = 28;
pub const IO_BANK0_GPIO_CTRL_IRQOVER_BITS: u32 = 3 << IO_BANK0_GPIO_CTRL_IRQOVER_SHIFT;
pub const IO_BANK0_GPIO_CTRL_INOVER_SHIFT: usize = 16;
pub const IO_BANK0_GPIO_CTRL_INOVER_BITS: u32 = 3 << IO_BANK0_GPIO_CTRL_INOVER_SHIFT;
pub const IO_BANK0_GPIO_CTRL_OEOVER_SHIFT: usize = 12;
pub const IO_BANK0_GPIO_CTRL_OEOVER_BITS: u32 = 3 << IO_BANK0_GPIO_CTRL_OEOVER_SHIFT;
pub const IO_BANK0_GPIO_CTRL_OUTOVER_SHIFT: usize = 8;
pub const IO_BANK0_GPIO_CTRL_OUTOVER_BITS: u32 = 3 << IO_BANK0_GPIO_CTRL_OUTOVER_SHIFT;
pub const IO_BANK0_GPIO_CTRL_FUNCSEL_BITS: u32 = 0x1F;