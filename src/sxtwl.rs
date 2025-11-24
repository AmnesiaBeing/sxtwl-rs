//! 农历计算的全局接口

use crate::date::Day;
use crate::gz::GanZhi;
use crate::types::{JieQiInfo, LunarDate, SolarDate};

/// 获取时天干地支
pub fn get_shi_gz(day_tian_gan: u8, hour: u8, is_zao_wan_zi_shi: bool) -> GanZhi {
    // 计算时地支
    let mut shi_zhi = (hour / 2) % 12;

    // 处理早晚子时
    if is_zao_wan_zi_shi && hour == 23 {
        shi_zhi = 0; // 子
    }

    // 计算时天干
    let shi_gan = (day_tian_gan * 2 + shi_zhi) % 10;

    GanZhi::new(shi_gan, shi_zhi).unwrap_or(GanZhi {
        tian_gan: shi_gan,
        di_zhi: shi_zhi,
    })
}

/// 公历转农历
pub fn from_solar(year: i32, month: u8, day: i32) -> LunarDate {
    let mut day_obj = Day::from_solar(year, month, day);
    day_obj.to_lunar_date()
}

/// 公历SolarDate转农历
pub fn from_solar_date(solar_date: SolarDate) -> LunarDate {
    let mut day_obj = Day::from_solar_date(solar_date);
    day_obj.to_lunar_date()
}

/// 农历转公历
pub fn from_lunar(year: i32, month: u8, day: i32, is_leap: bool) -> SolarDate {
    let mut day_obj = Day::from_lunar(year, month, day, is_leap);
    day_obj.to_solar_date()
}

/// 获取指定日期的农历信息
pub fn get_lunar_date(year: i32, month: u8, day: i32) -> LunarDate {
    from_solar(year, month, day)
}

/// 获取指定日期的节气信息
pub fn get_jie_qi_info(year: i32, month: u8, day: i32) -> JieQiInfo {
    let mut day_obj = Day::from_solar(year, month, day);

    JieQiInfo {
        jq_index: if day_obj.has_jie_qi() {
            day_obj.get_jie_qi()
        } else {
            255
        },
        julian_day: day_obj.get_jie_qi_jd(),
    }
}

/// 获取指定日期的星座
pub fn get_constellation(year: i32, month: u8, day: i32) -> u8 {
    let mut day_obj = Day::from_solar(year, month, day);
    day_obj.get_constellation()
}

/// 获取指定日期的星期几
pub fn get_week(year: i32, month: u8, day: i32) -> u8 {
    let mut day_obj = Day::from_solar(year, month, day);
    day_obj.get_week()
}

// /// 获取指定日期的日天干地支
// pub fn get_day_gz(year: i32, month: u8, day: i32) -> GanZhi {
//     let mut day_obj = Day::from_solar(year, month, day);
//     day_obj.get_day_gz()
// }

// /// 获取指定日期的月天干地支
// pub fn get_month_gz(year: i32, month: u8, day: i32) -> GanZhi {
//     let mut day_obj = Day::from_solar(year, month, day);
//     day_obj.get_month_gz()
// }

// /// 获取指定日期的年天干地支
// pub fn get_year_gz(year: i32, month: u8, day: i32, chinese_new_year_boundary: bool) -> GanZhi {
//     let mut day_obj = Day::from_solar(year, month, day);
//     day_obj.get_year_gz(chinese_new_year_boundary)
// }

// /// 检查是否为闰月
// pub fn is_leap_month(year: i32, month: u8) -> bool {
//     // 获取农历正月初一
//     let mut first_day = Day::from_lunar(year, 1, 1, false);

//     // 检查全年的月份
//     for _ in 0..13 {
//         if first_day.is_lunar_leap() && first_day.get_lunar_month() == month {
//             return true;
//         }
//         // 前进到下一个月
//         first_day = first_day.after(first_day.get_lunar_day() as i32);
//     }

//     false
// }
