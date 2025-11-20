//! 公历处理模块
//! 提供公历结构体创建功能（含合法性校验）及打印功能
use crate::types::SolarDate;
use core::fmt;

impl SolarDate {
    /// 创建一个新的公历日期，并进行基本校验
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> Option<Self> {
        // 基本范围检查
        if month < 1
            || month > 12
            || day < 1
            || day > 31
            || hour > 23
            || minute > 59
            || second < 0.0
            || second >= 60.0
        {
            return None;
        }

        Some(SolarDate {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

impl fmt::Display for SolarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:05.2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solordate_creation() {
        let solar = SolarDate::new(2024, 1, 1, 12, 0, 0.0).unwrap();
        assert_eq!(solar.year, 2024);
        assert_eq!(solar.month, 1);
        assert_eq!(solar.day, 1);
    }
}
