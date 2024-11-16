use core::ffi;
use crate::consts;

const FUNC_TABLE: *const u16 = consts::ROM_FUNC_TABLE as _;
const DATA_TABLE: *const u16 = consts::ROM_DATA_TABLE as _;

/// ROM function table lookup function.
///
/// See Pg 135 of RP2040 datasheet.
type RomTableLookupFn<T> = unsafe extern "C" fn(table: *const u16, code: u32) -> T;

/// Convert lookup characters to code for a [RomTableLookupFn].
fn rom_table_code(c1: u8, c2: u8) -> u32 {
	(c2 as u32) << 8 | c1 as u32
}

fn get_lookup_fn<T>() -> RomTableLookupFn<T> {
	unsafe {
		let lookup_fn = core::mem::transmute(consts::ROM_TABLE_LOOKUP_FN);
		lookup_fn
	}
}

fn rom_table_lookup<T>(table: *const u16, code: u32) -> T {
	let lookup = get_lookup_fn::<T>();

	unsafe {
		lookup(table, code)
	}
}

// TODO: Debug
pub fn flash_enter_cmd_xip() {
	let code = rom_table_code(b'C', b'X');
	let enter_xip: extern "C" fn() = rom_table_lookup(FUNC_TABLE, code);
	enter_xip();
}