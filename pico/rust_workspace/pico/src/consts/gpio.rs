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