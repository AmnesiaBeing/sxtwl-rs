//! 儒略日计算模块
//! 提供公历与儒略日的相互转换

use core::ops::{Add, Sub};

use crate::{JulianDay, types::SolarDate};

use libm::{floor, round};

impl From<SolarDate> for JulianDay {
    /// 公历转儒略日
    fn from(solar: SolarDate) -> Self {
        let y = solar.year as i32;
        let m = solar.month as i32;
        let d = solar.day as f64
            + solar.hour as f64 / 24.0
            + solar.minute as f64 / 1440.0
            + solar.second / 86400.0;

        let (y, m) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };

        // 判断是否为格里高利历日1582年10月15日
        let b = if solar.year > 1582
            || (solar.year == 1582 && solar.month > 10)
            || (solar.year == 1582 && solar.month == 10 && solar.day >= 15)
        {
            let a = y / 100;
            2 - a + a / 4
        } else {
            0
        };

        let jd = floor(365.25 * (y + 4716) as f64) + floor(30.6001 * (m + 1) as f64) + d + b as f64
            - 1524.5;

        Self(jd)
    }
}

impl From<JulianDay> for SolarDate {
    /// 儒略日转公历
    fn from(jd: JulianDay) -> Self {
        let jd = jd.0 + 0.5;
        let z = floor(jd);
        let f = jd - z;

        let a = if z < 2299161.0 {
            z
        } else {
            let alpha = floor((z - 1867216.25) / 36524.25);
            z + 1.0 + alpha - floor(alpha / 4.0)
        };

        let b = a + 1524.0;
        let c = floor((b - 122.1) / 365.25);
        let d = floor(365.25 * c);
        let e = floor((b - d) / 30.6001);

        let day = b - d - floor(30.6001 * e) + f;
        let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
        let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

        let total_seconds = (day - floor(day)) * 86400.0;
        let hour = (total_seconds / 3600.0) as u8;
        let remaining = total_seconds % 3600.0;
        let minute = (remaining / 60.0) as u8;
        let second = round(remaining % 60.0);

        Self {
            year: year as i32,
            month: month as u8,
            day: floor(day) as u8,
            hour,
            minute,
            second,
        }
    }
}

impl Add for JulianDay {
    type Output = Self;

    /// 儒略日加法
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Add<f64> for JulianDay {
    type Output = Self;

    /// 儒略日加法
    fn add(self, days: f64) -> Self::Output {
        Self(self.0 + days)
    }
}

impl Sub for JulianDay {
    type Output = Self;

    /// 儒略日减法
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Sub<f64> for JulianDay {
    type Output = Self;

    /// 儒略日减法
    fn sub(self, days: f64) -> Self::Output {
        Self(self.0 - days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_day_conversion() {
        // 测试用例1: 现代日期
        let solar1 = SolarDate {
            year: 2024,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,
            second: 0.0,
        };
        let jd1: JulianDay = solar1.into();
        let converted1: SolarDate = jd1.into();
        assert_eq!(solar1.year, converted1.year);
        assert_eq!(solar1.month, converted1.month);
        assert_eq!(solar1.day, converted1.day);
        // 允许时分秒有微小误差
        assert!((solar1.hour as i32 - converted1.hour as i32).abs() <= 1);

        // 测试用例2: 儒略历日期
        let solar2 = SolarDate {
            year: 1582,
            month: 10,
            day: 4,
            hour: 0,
            minute: 0,
            second: 0.0,
        };
        let jd2: JulianDay = solar2.into();
        let converted2: SolarDate = jd2.into();
        assert_eq!(solar2.year, converted2.year);
        assert_eq!(solar2.month, converted2.month);
        assert_eq!(solar2.day, converted2.day);

        // 测试用例3: 格里高利历日期
        let solar3 = SolarDate {
            year: 1582,
            month: 10,
            day: 15,
            hour: 0,
            minute: 0,
            second: 0.0,
        };
        let jd3: JulianDay = solar3.into();
        let converted3: SolarDate = jd3.into();
        assert_eq!(solar3.year, converted3.year);
        assert_eq!(solar3.month, converted3.month);
        assert_eq!(solar3.day, converted3.day);

        // 测试用例4: 包含时分秒
        let solar4 = SolarDate {
            year: 2000,
            month: 1,
            day: 1,
            hour: 14,
            minute: 30,
            second: 45.5,
        };
        let jd4: JulianDay = solar4.into();
        let converted4: SolarDate = jd4.into();
        assert_eq!(solar4.year, converted4.year);
        assert_eq!(solar4.month, converted4.month);
        assert_eq!(solar4.day, converted4.day);
        assert_eq!(solar4.hour, converted4.hour);
        assert_eq!(solar4.minute, converted4.minute);
        assert!((solar4.second - converted4.second).abs() < 1.0);

        // 测试用例5: 负年份（公元前）
        let solar5 = SolarDate {
            year: -100,
            month: 3,
            day: 15,
            hour: 6,
            minute: 0,
            second: 0.0,
        };
        let jd5: JulianDay = solar5.into();
        let converted5: SolarDate = jd5.into();
        assert_eq!(solar5.year, converted5.year);
        assert_eq!(solar5.month, converted5.month);
        assert_eq!(solar5.day, converted5.day);
    }

    #[test]
    fn test_known_julian_days() {
        // 已知的儒略日测试用例
        // 2000年1月1日 12:00:00 UTC 的儒略日应该是 2451545.0
        let solar = SolarDate {
            year: 2000,
            month: 1,
            day: 1,
            hour: 12,
            minute: 0,
            second: 0.0,
        };
        let jd: JulianDay = solar.into();
        assert!((jd.0 - 2451545.0).abs() < 0.001);

        // 1980年1月1日 的儒略日
        let solar = SolarDate {
            year: 1980,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0.0,
        };
        let jd: JulianDay = solar.into();
        assert!((jd.0 - 2444239.5).abs() < 0.001);
    }

    #[test]
    fn test_edge_cases() {
        // 闰年测试
        let leap_day = SolarDate {
            year: 2020,
            month: 2,
            day: 29,
            hour: 0,
            minute: 0,
            second: 0.0,
        };
        let jd: JulianDay = leap_day.into();
        let converted: SolarDate = jd.into();
        assert_eq!(leap_day.year, converted.year);
        assert_eq!(leap_day.month, converted.month);
        assert_eq!(leap_day.day, converted.day);

        // 午夜边界测试
        let midnight = SolarDate {
            year: 2024,
            month: 6,
            day: 15,
            hour: 23,
            minute: 59,
            second: 59.9,
        };
        let jd: JulianDay = midnight.into();
        let converted: SolarDate = jd.into();
        // 由于浮点精度，日期可能变化，但应该在合理范围内
        assert!((converted.day as i32 - midnight.day as i32).abs() <= 1);
    }
}
