//! 儒略日计算模块
//! 提供公历与儒略日的相互转换
use crate::{types::SolarDate, types::JulianDay};

impl JulianDay {
    /// 获取儒略日数值
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl From<SolarDate> for JulianDay {
    /// 从公历日期和时间计算儒略日
    /// 注意：此算法适用于公历1582年10月15日及以后（格里高利历）
    fn from(solar: SolarDate) -> Self {
        let y = solar.year as f64;
        let m = solar.month as f64;
        let d = solar.day as f64
            + (solar.hour as f64 + (solar.minute as f64 + solar.second / 60.0) / 60.0) / 24.0;

        let a = (solar.month as i32 / 12) as f64;
        let b = y + 4800.0 - a;
        let c = m + 12.0 * a - 3.0;

        let n = d + (153.0 * c + 2.0) / 5.0 + 365.0 * b + b / 4.0 - b / 100.0 + b / 400.0 - 32045.5;

        JulianDay(n)
    }
}

impl From<JulianDay> for SolarDate {
    /// 将儒略日转换为公历日期和时间
    fn from(jd: JulianDay) -> Self {
        let jd_val = jd.0 + 0.5;
        let z = jd_val as i32;
        let f = jd_val - z as f64;

        let a = if z < 2299161 {
            z
        } else {
            let alpha = ((z as f64 - 1867216.25) / 36524.25) as i32;
            z + 1 + alpha - alpha / 4
        };

        let b = a + 1524;
        let c = ((b as f64 - 122.1) / 365.25) as i32;
        let d = (365.25 * c as f64) as i32;
        let e = ((b - d) as f64 / 30.6001) as i32;

        let day = b - d - (30.6001 * e as f64) as i32;
        let month = if e < 14 { e - 1 } else { e - 13 };
        let year = if month > 2 { c - 4716 } else { c - 4715 };

        let total_seconds = f * 86400.0;
        let hours = (total_seconds / 3600.0) as u8;
        let minutes = ((total_seconds % 3600.0) / 60.0) as u8;
        let seconds = total_seconds % 60.0;

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
