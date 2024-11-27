//! Raspberry pi pico library
//!
//! This custom library contains the various pico constants, functions, and any
//!  utility wrappers that I write.

// TODO: Macro to implement _strat() and panic handler

#![no_std]

pub mod clocks;
pub mod consts;
pub mod gpio;
pub mod resets;
pub mod registers;
pub mod rom;
pub mod watchdog;

pub use registers::Register;
