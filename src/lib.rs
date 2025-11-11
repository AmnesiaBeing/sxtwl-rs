//! 农历/公历转换与传统历法计算库（no_std支持）
//! 支持天干地支、节气、生肖等传统历法元素的计算

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

// 外部依赖
use libm::floor;
use thiserror_no_std::Error;

// 内部模块导出
pub mod consts;
pub mod error;
pub mod ganzhi;
pub mod jieqi;
pub mod julian;
pub mod lunar;
pub mod types;
pub mod utils;

// 重导出核心类型与错误，简化外部使用
pub use error::CalendarError;
pub use types::*;

/// 从公历日期转换为农历日期
///
/// # 参数
/// - `solar`: 公历日期（年、月、日等）
///
/// # 返回值
/// 成功时返回农历日期，失败时返回`CalendarError`
pub fn solar_to_lunar(solar: SolarDate) -> Result<LunarDate, CalendarError> {
    // 1. 校验公历日期合法性
    utils::validate_solar_date(&solar)?;
    // 2. 转换为儒略日
    let jd = julian::solar_to_julian(solar)?;
    // 3. 儒略日转农历
    lunar::julian_to_lunar(jd)
}

/// 从农历日期转换为公历日期
///
/// # 参数
/// - `lunar`: 农历日期（含是否闰月）
///
/// # 返回值
/// 成功时返回公历日期，失败时返回`CalendarError`
pub fn lunar_to_solar(lunar: LunarDate) -> Result<SolarDate, CalendarError> {
    // 1. 校验农历日期合法性
    utils::validate_lunar_date(&lunar)?;
    // 2. 农历转儒略日
    let jd = lunar::lunar_to_julian(lunar)?;
    // 3. 儒略日转公历
    julian::julian_to_solar(jd)
}

/// 获取指定公历日期所在年份的节气列表
///
/// # 参数
/// - `year`: 公历年
///
/// # 返回值
/// 包含24个节气的列表，每个元素为（节气枚举，精确时间）
pub fn get_jieqi_by_year(year: i32) -> Result<alloc::vec::Vec<(JieQi, Time)>, CalendarError> {
    jieqi::calculate_year_jieqi(year)
}

/// 计算指定公历日期的干支（年、月、日、时）
///
/// # 参数
/// - `solar`: 公历日期（含时分秒，用于计算时辰干支）
///
/// # 返回值
/// 元组形式返回（年干支、月干支、日干支、时干支）
pub fn get_ganzhi(solar: SolarDate) -> Result<(GanZhi, GanZhi, GanZhi, GanZhi), CalendarError> {
    // 1. 转换为儒略日
    let jd = julian::solar_to_julian(solar)?;
    // 2. 计算各维度干支
    let year_gz = ganzhi::year_ganzhi(jd)?;
    let month_gz = ganzhi::month_ganzhi(jd)?;
    let day_gz = ganzhi::day_ganzhi(jd)?;
    let hour_gz = ganzhi::hour_ganzhi(day_gz.tg, solar.hour, solar.minute, solar.second)?;
    Ok((year_gz, month_gz, day_gz, hour_gz))
}
