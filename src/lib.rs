//! 农历/公历转换与传统历法计算库（no_std支持）
//! 支持天干地支、节气、生肖等传统历法元素的计算

#![no_std]
#![deny(missing_docs)]

extern crate alloc;
extern crate libm;

// 内部模块导出
pub mod consts;
// pub mod error;
// pub mod ganzhi;
pub mod jieqi;
pub mod julian;
pub mod lunar;
pub mod types;
// pub mod utils;

// 重导出核心类型与错误，简化外部使用
// pub use error::CalendarError;
pub use types::*;
