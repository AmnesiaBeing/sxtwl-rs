#![no_std]

extern crate alloc;
extern crate core;

pub mod culture;
#[cfg(feature = "eight-char")]
pub mod eightchar;
pub mod enums;
#[cfg(feature = "festival")]
pub mod festival;
#[cfg(feature = "holiday")]
pub mod holiday;
pub mod jd;
pub mod lunar;
#[cfg(feature = "rabbyung")]
pub mod rabbyung;
pub mod sixtycycle;
pub mod solar;
pub mod sxtwl;
pub mod types;

mod cache;

#[cfg(feature = "holiday")]
mod generated_holidays_data;
mod generated_leap_year_data;
#[cfg(feature = "rabbyung")]
mod generated_rab_byung;
