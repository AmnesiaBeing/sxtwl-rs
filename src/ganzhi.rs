//! 天干地支计算模块
//! 包含年、月、日、时干支的计算逻辑

use crate::{
    consts::{GAN, ZHI},
    error::CalendarError,
    types::GanZhi,
    utils::is_valid_tg,
};

/// 计算年干支（以1984年为甲子年为基准）
///
/// # 参数
/// - `lunar_year`: 农历年份（以立春或春节为界的年份）
///
/// # 返回值
/// 年干支结构体
pub fn get_year_ganzhi(lunar_year: i32) -> GanZhi {
    let diff = lunar_year - 1984;
    let tg = (diff % 10 + 10) % 10; // 确保非负
    let dz = (diff % 12 + 12) % 12;
    GanZhi {
        tg: tg as usize,
        dz: dz as usize,
    }
}

/// 计算月干支（基于年干支和节气）
///
/// # 参数
/// - `year_ganzhi`: 年干支
/// - `jieqi_index`: 节气索引（0-23，用于确定月份）
///
/// # 返回值
/// 月干支结构体
pub fn get_month_ganzhi(year_ganzhi: &GanZhi, jieqi_index: usize) -> Result<GanZhi, CalendarError> {
    // 年干确定月干起点（甲己起丙寅，乙庚起戊寅等）
    let start_tg = match year_ganzhi.tg {
        0 | 5 => 2, // 甲、己
        1 | 6 => 4, // 乙、庚
        2 | 7 => 6, // 丙、辛
        3 | 8 => 8, // 丁、壬
        4 | 9 => 0, // 戊、癸
        _ => return Err(CalendarError::InvalidGanZhi),
    };

    // 以节气索引计算月份偏移（每两个节气为一个月）
    let month_offset = jieqi_index / 2;
    let tg = (start_tg + month_offset) % 10;
    let dz = (2 + month_offset) % 12; // 地支从寅月（2）开始

    Ok(GanZhi { tg, dz })
}

/// 计算日干支（基于儒略日偏移）
///
/// # 参数
/// - `jd_offset`: 相对于基准日（2000年1月7日）的儒略日偏移
///
/// # 返回值
/// 日干支结构体
pub fn get_day_ganzhi(jd_offset: i64) -> GanZhi {
    // 2000年1月7日为甲子日（基准点）
    let base = 9000000; // 基准偏移量（源自C++代码）
    let total = base + jd_offset;
    let tg = (total % 10 + 10) as usize % 10;
    let dz = (total % 12 + 12) as usize % 12;
    GanZhi { tg, dz }
}

/// 计算时干支（基于日干和小时）
///
/// # 参数
/// - `day_tg`: 日干索引（0-9）
/// - `hour`: 小时（24小时制）
/// - `is_zaowan_zishi`: 是否区分早晚子时
///
/// # 返回值
/// 时干支结构体
pub fn get_shi_ganzhi(
    day_tg: usize,
    hour: u8,
    is_zaowan_zishi: bool,
) -> Result<GanZhi, CalendarError> {
    if !is_valid_tg(day_tg) {
        return Err(CalendarError::InvalidGanZhi);
    }
    if hour > 23 {
        return Err(CalendarError::InvalidDate("小时必须在0-23之间".into()));
    }

    // 计算时辰偏移（每2小时一个时辰）
    let mut step = (hour + 1) / 2;
    // 特殊处理子时（23-1点）
    if !is_zaowan_zishi && hour == 23 {
        step = 0; // 晚子时归为次日
    }
    let step = step as usize;

    // 日干确定时干起点（甲己起甲子，乙庚起丙子等）
    let start_tg = match day_tg {
        0 | 5 => 0, // 甲、己
        1 | 6 => 2, // 乙、庚
        2 | 7 => 4, // 丙、辛
        3 | 8 => 6, // 丁、壬
        4 | 9 => 8, // 戊、癸
        _ => return Err(CalendarError::InvalidGanZhi),
    };

    let tg = (start_tg + step) % 10;
    let dz = step % 12;

    Ok(GanZhi { tg, dz })
}

/// 获取干支索引（60甲子循环中的位置）
pub fn get_ganzhi_index(ganzhi: &GanZhi) -> Result<usize, CalendarError> {
    if !is_valid_tg(ganzhi.tg) || ganzhi.dz >= 12 {
        return Err(CalendarError::InvalidGanZhi);
    }
    // 60甲子循环：找到天干地支匹配的索引
    for i in 0..6 {
        let idx = ganzhi.tg + i * 10;
        if idx % 12 == ganzhi.dz {
            return Ok(idx);
        }
    }
    Err(CalendarError::InvalidGanZhi)
}

/// 干支转字符串（如"甲子"）
pub fn ganzhi_to_string(ganzhi: &GanZhi) -> Result<String, CalendarError> {
    if !is_valid_tg(ganzhi.tg) || ganzhi.dz >= 12 {
        return Err(CalendarError::InvalidGanZhi);
    }
    Ok(format!("{}{}", GAN[ganzhi.tg], ZHI[ganzhi.dz]))
}
