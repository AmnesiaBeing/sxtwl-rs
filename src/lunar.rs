//! 农历计算模块
//! 提供公历与农历的相互转换及农历信息查询

use crate::{
    consts::LUNAR_MONTH_NAMES,
    error::CalendarError,
    jieqi::{get_jieqi_by_year, jieqi_from_index},
    julian::{from_julian, jd_to_j2000_offset, to_julian},
    types::{LunarDate, SolarDate},
    utils::{validate_lunar_date, validate_solar_date},
};

/// 公历转农历
///
/// # 参数
/// - `solar`: 公历日期
///
/// # 返回值
/// 农历日期结构体
pub fn solar_to_lunar(solar: &SolarDate) -> Result<LunarDate, CalendarError> {
    validate_solar_date(solar)?;
    let jd = to_julian(solar)?;
    let j2000_offset = jd_to_j2000_offset(jd);

    // 获取前后两年的节气用于判断农历月份
    let year = solar.year;
    let jieqis_prev = get_jieqi_by_year(year - 1)?;
    let jieqis_curr = get_jieqi_by_year(year)?;
    let jieqis_next = get_jieqi_by_year(year + 1)?;
    let mut all_jieqis = [jieqis_prev, jieqis_curr, jieqis_next].concat();

    // 找到当前日期所在的农历月份区间
    let mut month_idx = 0;
    let mut is_leap = false;
    while month_idx < all_jieqis.len() - 1 {
        let start_jd = all_jieqis[month_idx].jd;
        let end_jd = all_jieqis[month_idx + 1].jd;
        if jd >= start_jd && jd < end_jd {
            // 判断是否为闰月（中气间隔超过30天）
            is_leap = (end_jd - start_jd) > 30.0;
            break;
        }
        month_idx += 1;
    }

    // 计算农历年（以立春为界）
    let lichun_jd = jieqis_curr[0].jd; // 当年立春
    let lunar_year = if jd < lichun_jd { year - 1 } else { year };

    // 计算农历月（1-12，结合闰月）
    let lunar_month = (month_idx % 12) + 1;

    // 计算农历日（当月天数内的偏移）
    let month_start_jd = all_jieqis[month_idx].jd;
    let lunar_day = (jd - month_start_jd).floor() as u8 + 1;

    Ok(LunarDate {
        year: lunar_year,
        month: lunar_month as u8,
        day: lunar_day,
        is_leap,
    })
}

/// 农历转公历
///
/// # 参数
/// - `lunar`: 农历日期
///
/// # 返回值
/// 公历日期结构体
pub fn lunar_to_solar(lunar: &LunarDate) -> Result<SolarDate, CalendarError> {
    validate_lunar_date(lunar)?;

    // 查找农历月对应的节气区间
    let mut target_jd = 0.0;
    let mut year = lunar.year;
    // 最多检查前后2年避免遗漏
    for y_offset in -1..=1 {
        let jieqis = get_jieqi_by_year(year + y_offset)?;
        for (i, jieqi) in jieqis.iter().enumerate() {
            let jq = jieqi_from_index(jieqi.jq_index)?;
            // 农历月份对应节气索引（简化逻辑）
            if i % 2 == 0 && (i / 2 + 1) == lunar.month as usize {
                // 计算当月天数（下一个节气与当前节气的差值）
                let next_jieqi = jieqis.get(i + 1).ok_or_else(|| {
                    CalendarError::JieQiCalculationError("无法找到下一个节气".into())
                })?;
                let month_days = (next_jieqi.jd - jieqi.jd).floor() as u8;

                // 检查是否为闰月且天数匹配
                if lunar.is_leap == ((next_jieqi.jd - jieqi.jd) > 30.0) && lunar.day <= month_days {
                    target_jd = jieqi.jd + (lunar.day - 1) as f64;
                    return Ok(from_julian(target_jd));
                }
            }
        }
    }

    Err(CalendarError::InvalidDate("无法转换农历到公历".into()))
}

/// 获取农历月份名称（含闰月标识）
pub fn get_lunar_month_name(month: u8, is_leap: bool) -> Result<String, CalendarError> {
    if month < 1 || month > 12 {
        return Err(CalendarError::InvalidLunarMonth);
    }
    let base_name = LUNAR_MONTH_NAMES[(month - 1) as usize];
    Ok(if is_leap {
        format!("闰{}", base_name)
    } else {
        base_name.to_string()
    })
}

/// 获取农历某月的天数
pub fn get_lunar_month_days(
    lunar_year: i32,
    month: u8,
    is_leap: bool,
) -> Result<u8, CalendarError> {
    let jieqis = get_jieqi_by_year(lunar_year)?;
    for (i, jieqi) in jieqis.iter().enumerate() {
        if i % 2 == 0 && (i / 2 + 1) == month as usize {
            let next_jieqi = jieqis
                .get(i + 1)
                .ok_or_else(|| CalendarError::JieQiCalculationError("无法找到下一个节气".into()))?;
            let days = (next_jieqi.jd - jieqi.jd).floor() as u8;
            // 闰月天数通常为29或30
            if is_leap == ((next_jieqi.jd - jieqi.jd) > 30.0) {
                return Ok(days);
            }
        }
    }
    Err(CalendarError::InvalidLunarMonth)
}

/// 获取某年的闰月（0表示无闰月）
pub fn get_leap_month(lunar_year: i32) -> Result<u8, CalendarError> {
    let jieqis = get_jieqi_by_year(lunar_year)?;
    // 检查中气间隔超过30天的月份（视为闰月）
    for i in 0..jieqis.len() - 1 {
        if i % 2 == 0 {
            // 中气索引
            let interval = jieqis[i + 1].jd - jieqis[i].jd;
            if interval > 30.0 {
                return Ok((i / 2 + 1) as u8);
            }
        }
    }
    Ok(0)
}
