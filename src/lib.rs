//! sxtwl-rust - 农历计算库
//! 提供公历/农历转换、节气、干支等功能

#![no_std]
// #![warn(missing_docs)]
#[deny(clippy::approx_constant)]

extern crate alloc;

pub mod astronomy;
pub mod consts;
pub mod date;
pub mod gz;
pub mod julianday;
pub mod lunar_phase_calculator;
pub mod types;

// 自动生成的气朔修正表
mod compressed_qishuo_correction_data;

// 气朔修正参数表
mod qishuo_fit_parameter;

// 从各模块中重新导出公共API
pub use crate::gz::GanZhi;
pub use crate::types::{JieQiInfo, JulianDay, SolarDate};
