use core::ffi;

/// ROM function table lookup function.
///
/// See Pg 135 of RP2040 datasheet.
type RomTableLookupFn = extern "C" fn(table: *const u16, code: u32) -> *const ffi::c_void;

/// Convert lookup characters to code for a [RomTableLookupFn].
fn rom_table_code(c1: u8, c2: u8) -> u32 {
	((c2 << 8) | c1) as u32
}