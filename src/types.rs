//! 基础类型定义

use core::fmt::Display;


/// 儒略日（天文计算基础，高精度浮点数）
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct JulianDay(pub f64);

/// 时间结构
#[derive(Debug, Clone, Copy)]
pub struct SolarDate {
    /// 年份
    pub year: i32,
    /// 月份（1-12）
    pub month: u8,
    /// 日期（1-31）
    pub day: u8,
    /// 小时（0-23）
    pub hour: u8,
    /// 分钟（0-59）
    pub minute: u8,
    /// 秒（0-59.999...）
    pub second: f64,
}

impl SolarDate {
    /// 创建新的时间实例
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

/// 转换为字符串表示
impl Display for SolarDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:.0}:{:.0}:{:.0}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/// 节气信息
#[derive(Debug, Clone, Copy)]
pub struct JieQiInfo {
    /// 节气的儒略日
    pub julian_day: f64,
    /// 节气索引
    pub jq_index: u8,
}

/// 农历日期结构
#[derive(Debug, Clone, Copy)]
pub struct LunarDate {
    /// 年份（以1984年为基准的农历年）
    pub year: i32,
    /// 月份（1-12）
    pub month: u8,
    /// 日期（1-30）
    pub day: u8,
    /// 是否为闰月
    pub is_leap_month: bool,
}

impl LunarDate {
    /// 创建新的农历日期实例
    pub fn new(year: i32, month: u8, day: u8, is_leap_month: bool) -> Self {
        Self {
            year,
            month,
            day,
            is_leap_month,
        }
    }
}

/// 转换为字符串表示
impl Display for LunarDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_leap_month {
            write!(f, "{}-闰{:02}-{:02}", self.year, self.month, self.day)
        } else {
            write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }
}
