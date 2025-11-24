// src/astronomy.rs
//! 天文计算模块
//! 基于寿星天文历算法实现

use crate::consts::{J2000, JULIAN_CENTURY_DAYS};
use core::f64::consts::{PI, TAU as PI2};
use libm::{asin, atan2, cos, sin, sqrt};

mod coefficients;
mod delta_t;
mod math_utils;
mod nutation;
mod planetary_orbits;
mod precession;

// 保留原有公共导出
pub use coefficients::*;
pub use delta_t::*;
pub use math_utils::*;
pub use nutation::*;
pub use planetary_orbits::*;
pub use precession::*;

/// 天文计算主模块
pub struct Astronomy;

impl Astronomy {
    /// 计算儒略世纪数 (相对于J2000.0)
    pub fn julian_century(jd: f64) -> f64 {
        (jd - J2000) / JULIAN_CENTURY_DAYS
    }
}
