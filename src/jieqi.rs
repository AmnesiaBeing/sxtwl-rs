//! 节气计算模块
//! 提供节气查询与精确时间计算

use crate::{
    consts::J2000,
    error::CalendarError,
    julian::{from_julian, to_julian},
    types::{JieQi, JieQiInfo},
};

/// 计算某年的所有节气
///
/// # 参数
/// - `year`: 公历年
///
/// # 返回值
/// 节气信息列表（含儒略日和索引）
pub fn get_jieqi_by_year(year: i32) -> Result<Vec<JieQiInfo>, CalendarError> {
    if year < 1900 || year > 2100 {
        return Err(CalendarError::JieQiCalculationError(
            "仅支持1900-2100年的节气计算".into(),
        ));
    }

    let mut jieqis = Vec::with_capacity(24);
    // 节气太阳黄经：立春315°，雨水330°，每个节气增加15°
    for i in 0..24 {
        let lon = 315.0 + i as f64 * 15.0;
        let jd = calc_jieqi_jd(year, lon)?;
        jieqis.push(JieQiInfo {
            jd,
            jq_index: i as u8,
        });
    }

    Ok(jieqis)
}

/// 计算单个节气的儒略日
///
/// # 参数
/// - `year`: 公历年
/// - `lon`: 节气对应的太阳黄经（度）
///
/// # 返回值
/// 节气发生的儒略日
fn calc_jieqi_jd(year: i32, lon: f64) -> Result<f64, CalendarError> {
    // 估算节气日期（每月4/19日左右）
    let month = (lon / 30.0).floor() as u8 + 1;
    let day = if lon % 30.0 < 15.0 { 4.0 } else { 19.0 };

    let mut solar = from_julian(J2000);
    solar.year = year;
    solar.month = month;
    solar.day = day as u8;
    solar.hour = 12;
    solar.minute = 0;
    solar.second = 0.0;

    let mut jd = to_julian(&solar)?;
    // 迭代精确计算
    for _ in 0..10 {
        let t = (jd - J2000) / 36525.0; // 儒略世纪数
        let sun_lon = sun_longitude(t); // 太阳黄经（度）
        let delta = (lon - sun_lon + 180.0) % 360.0 - 180.0;
        jd += delta * 0.985647366; // 1度≈0.9856天
        if delta.abs() < 0.0001 {
            break;
        }
    }

    Ok(jd)
}

/// 计算太阳黄经（度）
///
/// # 参数
/// - `t`: 儒略世纪数（相对于J2000）
///
/// # 返回值
/// 太阳黄经（度，0-360）
fn sun_longitude(t: f64) -> f64 {
    // 太阳几何平黄经（度）
    let l0 = 280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    // 太阳平近点角（弧度）
    let m = (357.52911 + 35999.05029 * t - 0.0001537 * t * t).to_radians();
    // 太阳中心差（度）
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m.sin().to_degrees()
        + (0.019993 - 0.000101 * t) * (2.0 * m).sin().to_degrees()
        + 0.000289 * (3.0 * m).sin().to_degrees();
    // 真黄经（度）
    let true_lon = l0 + c;
    // 修正章动（简化计算）
    let omega =
        (125.04452 - 1934.136261 * t + 0.0020708 * t * t + t.powi(3) / 450000.0).to_radians();
    let lon = true_lon - 0.00569 - 0.00478 * omega.sin().to_degrees();
    // 转换为0-360度
    lon % 360.0
}

/// 节气索引转枚举
pub fn jieqi_from_index(index: u8) -> Result<JieQi, CalendarError> {
    match index {
        0 => Ok(JieQi::LiChun),
        1 => Ok(JieQi::YuShui),
        2 => Ok(JieQi::JingZhe),
        3 => Ok(JieQi::ChunFen),
        4 => Ok(JieQi::QingMing),
        5 => Ok(JieQi::GuYu),
        6 => Ok(JieQi::LiXia),
        7 => Ok(JieQi::XiaoMan),
        8 => Ok(JieQi::MangZhong),
        9 => Ok(JieQi::XiaZhi),
        10 => Ok(JieQi::XiaoShu),
        11 => Ok(JieQi::DaShu),
        12 => Ok(JieQi::LiQiu),
        13 => Ok(JieQi::ChuShu),
        14 => Ok(JieQi::BaiLu),
        15 => Ok(JieQi::QiuFen),
        16 => Ok(JieQi::HanLu),
        17 => Ok(JieQi::ShuangJiang),
        18 => Ok(JieQi::LiDong),
        19 => Ok(JieQi::XiaoXue),
        20 => Ok(JieQi::DaXue),
        21 => Ok(JieQi::DongZhi),
        22 => Ok(JieQi::XiaoHan),
        23 => Ok(JieQi::DaHan),
        _ => Err(CalendarError::JieQiCalculationError(
            "无效的节气索引".into(),
        )),
    }
}
