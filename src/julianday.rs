//! 儒略日（Julian Day）计算模块

use crate::consts::J2000;
use crate::types::{JulianDay, SolarDate};

use alloc::string::String;
use libm::floor;

impl JulianDay {
    /// 获取儒略日数值
    pub fn value(&self) -> f64 {
        self.0
    }

    /// 儒略日转换为时间字符串
    pub fn to_datetime_string(&self) -> String {
        let datetime: SolarDate = JulianDay(self.value()).into();
        datetime.to_string()
    }

    /// 儒略日转换为从J2000起的天数
    pub fn to_j2000_days(jd: f64) -> i32 {
        floor(jd - J2000) as i32
    }

    /// 从J2000起的天数转换为儒略日
    pub fn from_j2000_days(days: i32) -> f64 {
        J2000 + days as f64
    }
}

/// 从公历日期和时间计算儒略日
impl From<SolarDate> for JulianDay {
    fn from(solar: SolarDate) -> Self {
        // 计算带时分秒的天数
        let day_with_time = solar.day as f64
            + (solar.hour as f64 + (solar.minute as f64 + solar.second / 60.0) / 60.0) / 24.0;

        // 判断是否为格里高利历日 1582*372+10*31+15 = 588829
        let is_gregorian =
            solar.year * 372 + solar.month as i32 * 31 + floor(day_with_time) as i32 >= 588829;

        // 调整年份和月份（1月和2月视为上一年的13月和14月）
        let (adjusted_year, adjusted_month) = if solar.month <= 2 {
            (solar.year - 1, solar.month as i32 + 12)
        } else {
            (solar.year, solar.month as i32)
        };

        // 计算百年闰修正值
        let century_correction = if is_gregorian {
            let century = adjusted_year / 100;
            2.0 - century as f64 + (century / 4) as f64
        } else {
            0.0
        };

        // 计算儒略日值
        let jd_value = floor(365.25 * (adjusted_year + 4716) as f64)
            + floor(30.6001 * (adjusted_month + 1) as f64)
            + day_with_time
            + century_correction
            - 1524.5;

        JulianDay(jd_value)
    }
}

/// 将儒略日转换为公历日期和时间
impl From<JulianDay> for SolarDate {
    fn from(jd: JulianDay) -> Self {
        // 调整儒略日值（12小时偏移）
        let jd_adjusted = jd.0 + 0.5;

        // 分离整数部分（日）和小数部分（时:分:秒）
        let day_number = jd_adjusted as i32;
        let fractional_day = jd_adjusted - day_number as f64;

        // 根据儒略日是否小于特定值（2299161）来确定计算方式
        // 2299161是格里高利历改革的关键日期
        let adjusted_day_number = if day_number < 2299161 {
            day_number
        } else {
            // 格里高利历修正计算
            let alpha = ((day_number as f64 - 1867216.25) / 36524.25) as i32;
            day_number + 1 + alpha - alpha / 4
        };

        // 计算中间变量
        let b = adjusted_day_number + 1524;
        let c = ((b as f64 - 122.1) / 365.25) as i32;
        let d = (365.25 * c as f64) as i32;
        let e = ((b - d) as f64 / 30.6001) as i32;

        // 计算日期、月份和年份
        let day = b - d - (30.6001 * e as f64) as i32;
        let month = if e < 14 { e - 1 } else { e - 13 };
        let year = if month > 2 { c - 4716 } else { c - 4715 };

        // 计算时分秒
        let total_seconds = fractional_day * 86400.0; // 一天86400秒
        let hours = (total_seconds / 3600.0) as u8;
        let minutes = ((total_seconds % 3600.0) / 60.0) as u8;
        let seconds = total_seconds % 60.0;

        // 返回构造的SolarDate实例
        SolarDate {
            year,
            month: month as u8,
            day: day as u8,
            hour: hours,
            minute: minutes,
            second: seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_conversion() {
        let solar = SolarDate {
            year: 2024,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,
            second: 0.0,
        };

        let jd: JulianDay = solar.into();
        let solar2: SolarDate = jd.into();

        assert_eq!(solar.year, solar2.year);
        assert_eq!(solar.month, solar2.month);
        assert_eq!(solar.day, solar2.day);
        assert_eq!(solar.hour, solar2.hour);
        assert_eq!(solar.minute, solar2.minute);
        assert!(solar.second - solar2.second < 1e-10);
    }
}
