# Experimental Rust project for Pi Pico
My simultaneous foray into embedded programming and Rust embedded programming.

# Instructions
* `cargo build`
* `make_uf2.sh path/to/bin`
* Flash the resulting UF2 to pi
* Profit

# Second stage bootloader steps
Just calls the ROM functions to connect flash and enable XIP.

# Notes/TODOs
* Eventually define intrinsic overwrites with ROM functions (for float, etc.).
   See [rp2040-hal source][1] for details/examples, and [the GCC docs][2] for
   the function names. Remember "weak symbols".
* Watchdog tick needs manual value. XOSC is 12MHz, and watchdog uses 1usec
   ticks, so cycles per tick should be 12. Needs testing, as claimed reference
   clock "is divided internally to generate a tick (nominally 1 usec) to use
   as the watchdog tick" (pg 546). SDK docs found indicate the `TICK.cycles`
   value _is the divider_, so 12 should be correct for the XOSC.
* Cortex `SYST_CALIB` register has a potential calibration value for 10ms,
   and if non-zero, can use to determine usec ticks (divide 10,000).
* RTC clock source and divisor set in `CLOCKS` registers.
* Possible approach to allow for copying functions/code to RAM is to create a
   unique linker section with linker-defined symbols that mark the start and
   end of this section, then use code to copy the address range to wherever in
   RAM. Every function that goes in section (#\[link_section\]) will be loaded
   into RAM. CAUTION: Any branching/jumping performed by the function could
   cause a crash/hang due to the destination potentially being outside this RAM
   address range.

[1]: https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal/src/float/div.rs
[2]: https://gcc.gnu.org/onlinedocs/gccint/Soft-float-library-routines.html
