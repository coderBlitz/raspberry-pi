use crate::consts::{IO_BANK0_BASE, IO_QSPI_BASE};
use crate::consts::gpio::*;
use crate::registers::Register;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum GpioOverride {
	Normal = GPIO_CTRL_NORMAL,
	Invert = GPIO_CTRL_INVERT,
	LowDisable = GPIO_CTRL_LOWDIS,
	HighEnable = GPIO_CTRL_HIGHEN,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Pin {
	Gpio(GpioPin),
	Qspi(QspiPin),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GpioPin {
	Gpio0 = 0,
	Gpio1,
	Gpio2,
	Gpio3,
	Gpio4,
	Gpio5,
	Gpio6,
	Gpio7,
	Gpio8,
	Gpio9,
	Gpio10,
	Gpio11,
	Gpio12,
	Gpio13,
	Gpio14,
	Gpio15,
	Gpio16,
	Gpio17,
	Gpio18,
	Gpio19,
	Gpio20,
	Gpio21,
	Gpio22,
	Gpio23,
	Gpio24,
	Gpio25,
	Gpio26,
	Gpio27,
	Gpio28,
	Gpio29,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum QspiPin {
	Sclk,
	Ss,
	Sd0,
	Sd1,
	Sd2,
	Sd3,
}

/// Represents a GPIO pin.
// Stored value is the address of the STATUS register, CTRL is +4. Pad control
//  is PADS_BANK0_BASE + id*4.
// Each GPIO pin gets 8 bytes (4 for STATUS, 4 for CTRL), so multiply
//  ID by 8 to get the address. Bit shifting here to avoid multiply.
pub struct Gpio(Register);
impl Gpio {
	pub const fn new(pin: Pin) -> Self {
		unsafe {
			match pin {
				Pin::Gpio(p) => Self::gpio_id(p as u32),
				Pin::Qspi(p) => Self::qspi_id(p as u32),
			}
		}
	}
	/// Get instance of a GPIO pin, where `id <= 29`.
	pub const unsafe fn gpio_id(id: u32) -> Self {
		unsafe {
			Gpio(Register::new(IO_BANK0_BASE + id << 3))
		}
	}
	/// Get instance of a QSPI pin, where `id <= 5`.
	pub const unsafe fn qspi_id(id: u32) -> Self {
		unsafe {
			Gpio(Register::new(IO_QSPI_BASE + id << 3))
		}
	}

	/* Function selects */
	pub fn use_f0(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F0);
	}
	pub fn use_f1(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F1);
	}
	pub fn use_f2(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F2);
	}
	pub fn use_f3(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F3);
	}
	pub fn use_f4(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F4);
	}
	pub fn use_f5(&self) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_CTRL_F5);
	}

	/* Overrides */
	/// Set interrupt override.
	pub fn irq_override(&self, v: GpioOverride) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_IRQOVER_BITS);
		self.0.atomic_bitset((v as u32) << IO_BANK0_GPIO_CTRL_IRQOVER_SHIFT);
	}
	/// Set input override.
	pub fn in_override(&self, v: GpioOverride) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_INOVER_BITS);
		self.0.atomic_bitset((v as u32) << IO_BANK0_GPIO_CTRL_INOVER_SHIFT);
	}
	/// Set output enable override.
	pub fn oe_override(&self, v: GpioOverride) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_OEOVER_BITS);
		self.0.atomic_bitset((v as u32) << IO_BANK0_GPIO_CTRL_OEOVER_SHIFT);
	}
	/// Set output override.
	pub fn out_override(&self, v: GpioOverride) {
		self.0.atomic_bitclear(IO_BANK0_GPIO_CTRL_OUTOVER_BITS);
		self.0.atomic_bitset((v as u32) << IO_BANK0_GPIO_CTRL_OUTOVER_SHIFT);
	}

	/* Status bits */
	/// Interrupt to processors after override.
	pub fn irq_to_proc(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_IRQTOPROC_BIT != 0
	}
	/// Interrupt from pad before override.
	pub fn irq_from_pad(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_IRQFROMPAD_BIT != 0
	}
	/// Input to peripheral after override.
	pub fn in_to_peri(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_INTOPERI_BIT != 0
	}
	/// Input from pad before override.
	pub fn in_from_pad(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_INFROMPAD_BIT != 0
	}
	/// Output enable to pad after override.
	pub fn oe_to_pad(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_OETOPAD_BIT != 0
	}
	/// Output enable from peripheral before override.
	pub fn oe_from_peri(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_OEFROMPERI_BIT != 0
	}
	/// Output to pad after override.
	pub fn out_to_pad(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_OUTTOPAD_BIT != 0
	}
	/// Output from peripheral before override.
	pub fn out_from_peri(&self) -> bool {
		self.0.get() & IO_BANK0_GPIO_STATUS_OUTFROMPERI_BIT != 0
	}
}