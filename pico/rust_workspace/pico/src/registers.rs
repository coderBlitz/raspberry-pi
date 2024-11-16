
const ATOMIC_XOR: usize = 0x1000;
const ATOMIC_BITMASK_SET: usize = 0x2000;
const ATOMIC_BITMASK_CLEAR: usize = 0x3000;

/// Pico peripheral register.
///
// Uses int so it can be used in static context.
pub struct Register(u32);
impl Register {
	/// Construct a register from the given address.
	///
	/// # Safety
	/// The user must ensure the address is a valid register, as defined in the
	///  RP2040 datasheet.
	pub const unsafe fn new(reg: u32) -> Self {
		Register(reg)
	}

	#[inline(always)]
	pub fn write(&mut self, value: u32) {
		unsafe {
			self.as_mut().write_volatile(value);
		}
	}
	#[inline(always)]
	pub fn read(&self) -> u32 {
		unsafe {
			self.as_mut().read_volatile()
		}
	}

	/// Atomically XOR the register with the given value.
	#[inline(always)]
	pub fn atomic_xor(&self, value: u32) {
		unsafe {
			self.as_mut().byte_add(ATOMIC_XOR).write_volatile(value)
		}
	}

	/// Atomically set bits in register given by `value`.
	#[inline(always)]
	pub fn atomic_bitset(&self, value: u32) {
		unsafe {
			self.as_mut().byte_add(ATOMIC_BITMASK_SET).write_volatile(value)
		}
	}

	/// Atomically clear bits in register given by `value`.
	#[inline(always)]
	pub fn atomic_bitclear(&self, value: u32) {
		unsafe {
			self.as_mut().byte_add(ATOMIC_BITMASK_CLEAR).write_volatile(value)
		}
	}

	#[inline(always)]
	const fn as_mut(&self) -> *mut u32 {
		self.0 as _
	}
}
