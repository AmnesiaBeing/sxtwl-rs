//! 辅助工具函数模块
//! 包含日期合法性校验、中文转换、常量映射等辅助功能

use alloc::string::{String, ToString};

use crate::{
    consts::{JIEQI_NAMES, LUNAR_DAY_NAMES, LUNAR_MONTH_NAMES, SHENGXIAO, WEEK_CN},
    error::CalendarError,
    types::{GanZhi, JieQi, LunarDate, SolarDate},
};

/// 校验公历日期合法性
///
/// # 参数
/// - `solar`: 公历日期
///
/// # 返回值
/// 成功返回`Ok(())`，失败返回`CalendarError::InvalidDate`
pub fn validate_solar_date(solar: &SolarDate) -> Result<(), CalendarError> {
    // 年份范围限制（可根据实际需求调整）
    if solar.year < 1 || solar.year > 9999 {
        return Err(CalendarError::InvalidDate("年份超出有效范围".into()));
    }
    if solar.month < 1 || solar.month > 12 {
        return Err(CalendarError::InvalidDate("月份必须在1-12之间".into()));
    }
    // 计算当月最大天数
    let max_day = match solar.month {
        2 => {
            // 判断闰年
            if (solar.year % 4 == 0 && solar.year % 100 != 0) || (solar.year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if solar.day < 1 || solar.day > max_day {
        return Err(CalendarError::InvalidDate(format!(
            "日期超出范围（1-{}）",
            max_day
        )));
    }
    // 时间部分校验
    if solar.hour < 0 || solar.hour > 23 {
        return Err(CalendarError::InvalidDate("小时必须在0-23之间".into()));
    }
    if solar.minute < 0 || solar.minute > 59 {
        return Err(CalendarError::InvalidDate("分钟必须在0-59之间".into()));
    }
    if solar.second < 0.0 || solar.second >= 60.0 {
        return Err(CalendarError::InvalidDate("秒数必须在0-60之间".into()));
    }
    Ok(())
}

/// 校验农历日期合法性
///
/// # 参数
/// - `lunar`: 农历日期
///
/// # 返回值
/// 成功返回`Ok(())`，失败返回`CalendarError::InvalidDate`
pub fn validate_lunar_date(lunar: &LunarDate) -> Result<(), CalendarError> {
    // 年份范围限制（可根据实际需求调整）
    if lunar.year < 1 || lunar.year > 9999 {
        return Err(CalendarError::InvalidDate("农历年份超出有效范围".into()));
    }
    if lunar.month < 1 || lunar.month > 12 {
        return Err(CalendarError::InvalidDate("农历月份必须在1-12之间".into()));
    }
    // 农历每月天数范围（29-30天，具体需结合实际历法，但此处做基础校验）
    if lunar.day < 1 || lunar.day > 30 {
        return Err(CalendarError::InvalidDate("农历日期必须在1-30之间".into()));
    }
    Ok(())
}

/// 获取天干地支对应的字符串
///
/// # 参数
/// - `ganzhi`: 天干地支结构体
///
/// # 返回值
/// 天干地支组合字符串（如"甲子"）
pub fn ganzhi_to_str(ganzhi: &GanZhi) -> Result<String, CalendarError> {
    Ok(format!("{}{}", GAN[ganzhi.tg], ZHI[ganzhi.dz]))
}

/// 获取生肖对应的字符串
///
/// # 参数
/// - `year`: 农历年份
///
/// # 返回值
/// 生肖字符串（如"鼠"）
pub fn shengxiao_by_year(year: i32) -> String {
    // 以农历1900年为鼠年作为基准（可根据实际历法调整）
    let index = ((year - 1900) % 12 + 12) % 12;
    SHENGXIAO[index as usize].to_string()
}

/// 获取节气名称
///
/// # 参数
/// - `jieqi`: 节气枚举
///
/// # 返回值
/// 节气名称字符串
pub fn jieqi_to_str(jieqi: JieQi) -> String {
    JIEQI_NAMES[jieqi as usize].to_string()
}

/// 获取农历月份名称（含闰月标识）
///
/// # 参数
/// - `month`: 农历月份（1-12）
/// - `is_leap`: 是否为闰月
///
/// # 返回值
/// 农历月份名称（如"闰五月"）
pub fn lunar_month_to_str(month: u8, is_leap: bool) -> Result<String, CalendarError> {
    if month < 1 || month > 12 {
        return Err(CalendarError::InvalidLunarMonth);
    }
    let base = LUNAR_MONTH_NAMES[(month - 1) as usize].to_string();
    Ok(if is_leap {
        format!("闰{}", base)
    } else {
        base
    })
}

/// 获取农历日期名称
///
/// # 参数
/// - `day`: 农历日期（1-30）
///
/// # 返回值
/// 农历日期名称（如"初一"）
pub fn lunar_day_to_str(day: u8) -> Result<String, CalendarError> {
    if day < 1 || day > 30 {
        return Err(CalendarError::InvalidLunarDay);
    }
    Ok(LUNAR_DAY_NAMES[(day - 1) as usize].to_string())
}

/// 获取星期几的中文名称
///
/// # 参数
/// - `week`: 星期索引（0=周日，6=周六）
///
/// # 返回值
/// 星期中文名称
pub fn week_to_cn(week: u8) -> Result<String, CalendarError> {
    if week as usize >= WEEK_CN.len() {
        return Err(CalendarError::InvalidWeek);
    }
    Ok(WEEK_CN[week as usize].to_string())
}

/// 检查是否为合法的天干索引（0-9）
pub fn is_valid_tg(tg: usize) -> bool {
    tg < 10
}

/// 检查是否为合法的地支索引（0-11）
pub fn is_valid_dz(dz: usize) -> bool {
    dz < 12
}
