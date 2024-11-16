use crate::consts::{IO_BANK0_BASE, IO_QSPI_BASE};
use crate::consts::gpio::*;
use crate::registers::Register;

/// Represents a GPIO pin.
// Stored value is the address of the STATUS register, CTRL is +4. Pad control
//  is PADS_BANK0_BASE + id*4.
// Each GPIO pin gets 8 bytes (4 for STATUS, 4 for CTRL), so multiply
//  ID by 8 to get the address. Bit shifting here to avoid multiply.
struct Gpio(Register);
impl Gpio {
	/// Get instance of a GPIO pin, where `id <= 29`.
	pub const unsafe fn gpio_id(id: u32) -> Self {
		unsafe {
			Gpio(Register::new(IO_BANK0_BASE + id << 3))
		}
	}
	/// Get instance of a QSPI pin, where `id <= 6`.
	pub const unsafe fn qspi_id(id: u32) -> Self {
		unsafe {
			Gpio(Register::new(IO_QSPI_BASE + id << 3))
		}
	}

	pub fn use_F0(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F0);
	}
	pub fn use_F1(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F1);
	}
	pub fn use_F2(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F2);
	}
	pub fn use_F3(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F3);
	}
	pub fn use_F4(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F4);
	}
	pub fn use_F5(&self) {
		self.0.atomic_bitclear(GPIO_FUNCSEL_BITS);
		self.0.atomic_bitset(GPIO_F5);
	}
}