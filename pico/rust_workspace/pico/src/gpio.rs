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

/// Represents a GPIO pin.
// Stored value is the address of the STATUS register, CTRL is +4. Pad control
//  is PADS_BANK0_BASE + id*4.
// Each GPIO pin gets 8 bytes (4 for STATUS, 4 for CTRL), so multiply
//  ID by 8 to get the address. Bit shifting here to avoid multiply.
pub struct Gpio(Register);
impl Gpio {
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
}