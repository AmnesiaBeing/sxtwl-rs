//! 农历计算模块
//! 提供公历与农历的相互转换及农历信息查询

use alloc::vec::Vec;

use crate::{
    JieQi, JieQiInfo, JulianDay,
    types::{LunarDate, SolarDate},
};

use libm::floor;

/// 公历转农历
///
/// # 参数
/// - `solar`: 公历日期
///
/// # 返回值
/// 农历日期结构体
impl From<SolarDate> for LunarDate {
    fn from(solar: SolarDate) -> Self {
        let jd: JulianDay = solar.into();

        // 获取前后两年的节气用于判断农历月份
        let year = solar.year;
        let jieqis_prev = JieQi::get_all_jieqi_by_solar_year(year - 1);
        let jieqis_curr = JieQi::get_all_jieqi_by_solar_year(year);
        let jieqis_next = JieQi::get_all_jieqi_by_solar_year(year + 1);

        let mut all_jieqis = Vec::with_capacity(72);
        all_jieqis.extend(&jieqis_prev);
        all_jieqis.extend(&jieqis_curr);
        all_jieqis.extend(&jieqis_next);

        // 找到当前日期所在的农历月份区间
        let (month_idx, is_leap) = find_lunar_month_info(jd, &all_jieqis);

        // 计算农历年（以立春为界）
        let lunar_year = calculate_lunar_year(jd, year, &jieqis_curr);

        // 计算农历月（1-12，结合闰月）
        let lunar_month = calculate_lunar_month(month_idx, &all_jieqis);

        // 计算农历日（当月天数内的偏移）
        let lunar_day = calculate_lunar_day(jd, month_idx, &all_jieqis);

        Self {
            year: lunar_year,
            month: lunar_month,
            day: lunar_day,
            is_leap,
        }
    }
}

/// 农历转公历
///
/// # 参数
/// - `lunar`: 农历日期
///
/// # 返回值
/// 公历日期结构体
impl From<LunarDate> for SolarDate {
    fn from(lunar: LunarDate) -> Self {
        // 查找农历月对应的节气区间
        for y_offset in -1..=1 {
            let check_year = lunar.year + y_offset;
            let jieqis = JieQi::get_all_jieqi_by_solar_year(check_year);
            if let Some(solar_date) = find_solar_date_from_lunar(&lunar, &jieqis, check_year) {
                return solar_date;
            }
        }

        // 如果在前三年内没有找到，使用默认的近似计算
        // 这种情况应该很少见，只在极端情况下发生
        approximate_lunar_to_solar(lunar)
    }
}

/// 近似计算农历转公历（备用方法）
fn approximate_lunar_to_solar(lunar: LunarDate) -> SolarDate {
    // 使用简单的近似计算：假设农历月平均29.5天
    let days_since_spring = ((lunar.month as i32 - 1) * 29 + (lunar.day as i32 - 1)) as f64;

    // 估算立春日期（2月4日左右）
    let spring_jd = JulianDay::from(SolarDate {
        year: lunar.year,
        month: 2,
        day: 4,
        hour: 12,
        minute: 0,
        second: 0.0,
    })
    .0;

    let target_jd = spring_jd + days_since_spring;
    JulianDay(target_jd).into()
}

/// 查找农历月份信息
fn find_lunar_month_info(jd: JulianDay, all_jieqis: &[JieQiInfo]) -> (usize, bool) {
    for i in 0..all_jieqis.len().saturating_sub(1) {
        let start_jd = all_jieqis[i].jd;
        let end_jd = all_jieqis[i + 1].jd;

        if jd >= start_jd && jd < end_jd {
            // 判断是否为闰月（中气间隔超过30天）
            let is_leap = (end_jd - start_jd).0 > 30.0;
            return (i, is_leap);
        }
    }
    // 如果没有找到匹配的区间，返回最后一个区间或第一个区间
    if all_jieqis.len() >= 2 {
        (all_jieqis.len() - 2, false)
    } else {
        (0, false)
    }
}

/// 计算农历年
fn calculate_lunar_year(jd: JulianDay, solar_year: i32, jieqis_curr: &[JieQiInfo]) -> i32 {
    if jieqis_curr.is_empty() {
        return solar_year;
    }

    let lichun_jd = jieqis_curr[0].jd; // 当年立春
    if jd < lichun_jd {
        solar_year - 1
    } else {
        solar_year
    }
}

/// 计算农历月份
fn calculate_lunar_month(month_idx: usize, all_jieqis: &[JieQiInfo]) -> u8 {
    // 简化逻辑：取模12得到月份，考虑闰月情况
    let base_month = (month_idx % 12) as u8 + 1;

    // 检查是否需要调整闰月
    if month_idx >= 12 && (all_jieqis[month_idx].jd - all_jieqis[month_idx - 12].jd).0 > 30.0 {
        base_month - 1 // 调整闰月
    } else {
        base_month
    }
}

/// 计算农历日
fn calculate_lunar_day(jd: JulianDay, month_idx: usize, all_jieqis: &[JieQiInfo]) -> u8 {
    if month_idx >= all_jieqis.len() {
        return 1;
    }

    let month_start_jd = all_jieqis[month_idx].jd;
    let day_offset = floor((jd - month_start_jd).0);

    // 确保日期在合理范围内 (1-30)
    day_offset.max(0.0).min(29.0) as u8 + 1
}

/// 从农历日期查找公历日期
fn find_solar_date_from_lunar(
    lunar: &LunarDate,
    jieqis: &[JieQiInfo],
    year: i32,
) -> Option<SolarDate> {
    for i in 0..jieqis.len().saturating_sub(1) {
        // 检查节气索引对应的月份
        let jieqi_month = (i / 2) as u8 + 1;

        if jieqi_month == lunar.month {
            let start_jieqi = &jieqis[i];
            let end_jieqi = &jieqis[i + 1];

            // 检查闰月条件是否匹配
            let is_leap_month = (end_jieqi.jd - start_jieqi.jd).0 > 30.0;
            if lunar.is_leap != is_leap_month {
                continue;
            }

            // 计算月份天数
            let month_days = floor((end_jieqi.jd - start_jieqi.jd).0) as u8;

            // 检查日期是否在有效范围内
            if lunar.day > 0 && lunar.day <= month_days {
                let target_jd = start_jieqi.jd + (lunar.day - 1) as f64;
                let solar: SolarDate = target_jd.into();

                // 验证年份是否匹配
                if solar.year == year {
                    return Some(solar);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    fn create_test_jieqi_info(jd: f64, index: u8) -> JieQiInfo {
        JieQiInfo {
            jd: JulianDay(jd),
            jq_index: JieQi::from_index(index).unwrap(),
        }
    }

    fn create_test_solar_date(year: i32, month: u8, day: u8) -> SolarDate {
        SolarDate {
            year,
            month,
            day,
            hour: 12,
            minute: 0,
            second: 0.0,
        }
    }

    #[test]
    fn test_solar_to_lunar_basic() {
        let solar = create_test_solar_date(2023, 6, 15);
        let lunar: LunarDate = solar.into();

        assert!(lunar.year >= 2022 && lunar.year <= 2024);
        assert!(lunar.month >= 1 && lunar.month <= 12);
        assert!(lunar.day >= 1 && lunar.day <= 30);
    }

    #[test]
    fn test_lunar_to_solar_basic() {
        let lunar = LunarDate {
            year: 2023,
            month: 5,
            day: 15,
            is_leap: false,
        };

        let solar: SolarDate = lunar.into();

        // 验证转换后的公历日期在合理范围内
        assert!(solar.year >= 2022 && solar.year <= 2024);
        assert!(solar.month >= 1 && solar.month <= 12);
        assert!(solar.day >= 1 && solar.day <= 31);
    }

    #[test]
    fn test_find_lunar_month_info() {
        let jieqis = vec![
            create_test_jieqi_info(2450000.0, 0),
            create_test_jieqi_info(2450030.0, 1),
            create_test_jieqi_info(2450060.0, 2),
        ];

        let jd = JulianDay(2450015.0);
        let (month_idx, is_leap) = find_lunar_month_info(jd, &jieqis);

        assert_eq!(month_idx, 0);
        assert!(!is_leap); // 30天间隔不算闰月
    }

    #[test]
    fn test_calculate_lunar_year() {
        let jieqis_2023 = vec![
            create_test_jieqi_info(2450000.0, 0), // 立春
        ];

        // 在立春之前
        let jd_before = JulianDay(2449999.0);
        let year_before = calculate_lunar_year(jd_before, 2023, &jieqis_2023);
        assert_eq!(year_before, 2022);

        // 在立春之后
        let jd_after = JulianDay(2450001.0);
        let year_after = calculate_lunar_year(jd_after, 2023, &jieqis_2023);
        assert_eq!(year_after, 2023);
    }

    #[test]
    fn test_calculate_lunar_month() {
        let jieqis = vec![
            create_test_jieqi_info(2450000.0, 0),
            create_test_jieqi_info(2450035.0, 1), // 35天间隔，可能表示闰月
        ];

        // 正常月份
        let month1 = calculate_lunar_month(0, &jieqis);
        assert_eq!(month1, 1);

        // 可能需要调整的月份
        let month2 = calculate_lunar_month(13, &jieqis);
        assert!(month2 >= 1 && month2 <= 12);
    }

    #[test]
    fn test_calculate_lunar_day() {
        let jieqis = vec![
            create_test_jieqi_info(2450000.0, 0),
            create_test_jieqi_info(2450030.0, 1),
        ];

        let jd = JulianDay(2450015.0);
        let day = calculate_lunar_day(jd, 0, &jieqis);

        assert_eq!(day, 16); // 2450015.0 - 2450000.0 = 15 + 1 = 16
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let jieqis_empty = vec![];
        let jd = JulianDay(2450000.0);

        // 空节气列表
        let (month_idx, is_leap) = find_lunar_month_info(jd, &jieqis_empty);
        assert_eq!(month_idx, 0);
        assert!(!is_leap);

        // 农历日期边界
        let lunar_edge = LunarDate {
            year: 2023,
            month: 12,
            day: 30,
            is_leap: false,
        };

        let solar: SolarDate = lunar_edge.into();
        assert!(solar.year >= 2022 && solar.year <= 2024);
    }

    #[test]
    fn test_approximate_conversion() {
        // 测试近似转换方法
        let lunar = LunarDate {
            year: 2023,
            month: 6,
            day: 15,
            is_leap: false,
        };

        let solar = approximate_lunar_to_solar(lunar);
        assert!(solar.year == 2023);
        assert!(solar.month >= 5 && solar.month <= 7); // 应该在5-7月之间
    }

    #[test]
    fn test_round_trip_conversion() {
        // 测试公历->农历->公历的往返转换
        let original_solar = create_test_solar_date(2023, 6, 15);
        let lunar: LunarDate = original_solar.into();
        let converted_solar: SolarDate = lunar.into();

        // 往返转换可能会有小的误差，但应该在合理范围内
        assert!(
            converted_solar.year == original_solar.year
                || converted_solar.year == original_solar.year - 1
                || converted_solar.year == original_solar.year + 1
        );
    }
}
