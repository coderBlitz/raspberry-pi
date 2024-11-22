use crate::consts;

/// ROM function table lookup function.
///
/// See Pg 135 of RP2040 datasheet.
type RomTableLookupFn<T> = unsafe extern "C" fn(table: *const u16, code: u32) -> T;

/// Convert lookup characters to code for a [RomTableLookupFn].
#[inline(always)]
pub fn rom_table_code(c1: u8, c2: u8) -> u32 {
	(c2 as u32) << 8 | c1 as u32
}

#[inline(always)]
fn get_lookup_fn<T>() -> RomTableLookupFn<T> {
	unsafe {
		let addr = *(consts::ROM_TABLE_LOOKUP_FN as *const u16) as u32;
		core::mem::transmute(addr)
	}
}

pub fn func_table_lookup<T>(code: u32) -> T {
	let func_table: *const u16 = unsafe { *(consts::ROM_FUNC_TABLE as *const u16) as *const u16};
	let lookup = get_lookup_fn::<T>();

	unsafe {
		lookup(func_table, code)
	}
}

// TODO: Try again before more debugging. Fixed get_lookup_fn and fun_table_lookup.
pub fn flash_enter_cmd_xip() {
	let code = rom_table_code(b'C', b'X');
	let enter_xip: extern "C" fn() = func_table_lookup(code);
	let code = rom_table_code(b'I', b'F');
	let connect_flash: extern "C" fn() = func_table_lookup(code);

	connect_flash();
	enter_xip();
}