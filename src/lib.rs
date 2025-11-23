//! sxtwl-rust - 农历计算库
//! 提供公历/农历转换、节气、干支等功能

// #![no_std]
#![warn(missing_docs)]

// 导入各模块
pub mod consts;
pub mod date;
pub mod gz;
pub mod julianday;
pub mod ssq;
pub mod types;
pub mod utils;

// 从各模块中重新导出公共API
pub use crate::gz::GanZhi;
pub use crate::types::{JieQiInfo, JulianDay, QSType, SolarDate};

/// 公共API函数模块
/// 提供主要的对外接口函数
pub mod api {
    use super::date::ChineseCalendar;
    use super::ssq;
    use super::types::{JieQiInfo, SolarDate};
    use super::GanZhi;

    /// 从公历日期创建日历实例
    pub fn from_solar(year: i32, month: u8, day: u8) -> ChineseCalendar {
        ChineseCalendar::from_solar(year, month, day)
    }

    /// 从农历日期创建日历实例
    pub fn from_lunar(year: i32, month: u8, day: u8, is_leap: bool) -> ChineseCalendar {
        ChineseCalendar::from_lunar(year, month, day, is_leap)
    }

    /// 获取时辰天干地支
    pub fn get_hour_gan_zhi(day_tian_gan: u8, hour: u8, _is_early_late_zi: bool) -> GanZhi {
        // 这里需要临时创建一个日历实例来使用get_hour_gan_zhi方法
        // 在实际实现中，可能需要优化为更直接的计算
        // 不需要创建日历实例，直接计算

        // 计算时地支
        let mut hour_index = (hour as i32 - 1) / 2;
        if hour >= 23 || hour < 1 {
            hour_index = 0; // 子时
        }
        let di_zhi = ((hour_index + 11) % 12) as u8;

        // 计算时天干：(日天干 * 2 + 时地支) % 10
        let tian_gan = ((day_tian_gan as i32 * 2 + di_zhi as i32) % 10 + 10) % 10;

        GanZhi {
            tian_gan: tian_gan as u8,
            di_zhi,
        }
    }

    /// 获取某年的闰月（不存在则返回0）
    pub fn get_leap_month(year: i32) -> u8 {
        if let Some(month) = super::utils::calculate_leap_month(year) {
            month
        } else {
            0
        }
    }

    /// 获取农历月的天数
    pub fn get_lunar_month_days(year: i32, month: u8, is_leap: bool) -> u8 {
        // 简化实现：大多数农历月是29或30天
        // 实际应该通过计算两个朔日之间的天数来确定
        // 简化实现，实际应该更精确地计算

        // 计算下个月的朔日
        let ssq_calculator = ssq::SSQCalculator::new();
        // let new_moons = ssq_calculator.find_new_moons(year, year + 1);

        // 简化实现：查找目标月附近的两个朔日
        // 实际需要更精确的计算

        // 默认返回29天
        29
    }

    /// 获取某年的节气信息
    pub fn get_jie_qi_by_year(year: i32) -> Vec<JieQiInfo> {
        let ssq_calculator = ssq::SSQCalculator::new();
        let terms = ssq_calculator.find_terms(year);

        // 转换为JieQiInfo格式
        terms
            .into_iter()
            .enumerate()
            .map(|(idx, jd)| JieQiInfo {
                julian_day: jd,
                jq_index: idx as u8,
            })
            .collect()
    }
}
