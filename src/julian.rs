//! 儒略日计算模块
//! 提供公历与儒略日的相互转换

use crate::{consts::J2000, error::CalendarError, types::SolarDate, utils::validate_solar_date};

/// 公历转儒略日
///
/// # 参数
/// - `solar`: 公历日期（含时分秒）
///
/// # 返回值
/// 儒略日（JD）
pub fn solar_to_julian(solar: &SolarDate) -> Result<f64, CalendarError> {
    validate_solar_date(solar)?;

    let y = solar.year as i32;
    let m = solar.month as i32;
    let d = solar.day as f64
        + solar.hour as f64 / 24.0
        + solar.minute as f64 / 1440.0
        + solar.second / 86400.0;

    let (y, m) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };

    let b = if solar.year > 1582
        || (solar.year == 1582 && m > 10)
        || (solar.year == 1582 && m == 10 && d >= 15.0)
    {
        // 格里高利历修正
        let a = y / 100;
        2 - a + a / 4
    } else {
        0
    };

    let c = if y < 0 { 0.75 } else { 0.0 };

    let jd =
        (365.25 * (y + 4716) as f64).floor() + (30.6001 * (m + 1) as f64).floor() + d + b - 1524.5;

    Ok(jd)
}

/// 儒略日转公历
///
/// # 参数
/// - `jd`: 儒略日
///
/// # 返回值
/// 公历日期结构体
pub fn julian_to_solar(jd: f64) -> SolarDate {
    let mut jd = jd + 0.5;
    let z = jd.floor() as i32;
    let f = jd - z as f64;

    let a = if z < 2299161 {
        z
    } else {
        let alpha = ((z - 1867216.25) / 36524.25).floor() as i32;
        z + 1 + alpha - (alpha / 4)
    };

    let b = a + 1524;
    let c = ((b - 122.1) / 365.25).floor() as i32;
    let d = (365.25 * c as f64).floor() as i32;
    let e = ((b - d) / 30.6001).floor() as i32;

    let day = (b - d - (30.6001 * e as f64).floor() as i32) as u8 + f;
    let month = if e < 14 { e - 1 } else { e - 13 } as u8;
    let year = if month > 2 { c - 4716 } else { c - 4715 } as i32;

    // 拆分时分秒
    let day_frac = day.fract();
    let hour = (day_frac * 24.0).floor() as u8;
    let min_frac = (day_frac * 24.0 - hour as f64) * 60.0;
    let minute = min_frac.floor() as u8;
    let second = (min_frac - minute as f64) * 60.0;

    SolarDate {
        year,
        month,
        day: day.floor() as u8,
        hour,
        minute,
        second,
    }
}

/// 计算相对于J2000的儒略日偏移
pub fn jd_to_j2000_offset(jd: f64) -> f64 {
    jd - J2000
}

/// 从J2000偏移计算儒略日
pub fn j2000_offset_to_jd(offset: f64) -> f64 {
    offset + J2000
}
