//! 节气计算模块
//! 提供节气查询与精确时间计算

use crate::{JieQiInfo, JulianDay, SolarDate, types::JieQi};

use alloc::vec::Vec;

use libm::{floor, pow, sin};

/// 2000年1月1日12:00的儒略日
pub const J2000: f64 = 2451545.0;

impl JieQi {
    /// 获取节气名称
    pub fn name(&self) -> &'static str {
        match self {
            JieQi::LiChun => "立春",
            JieQi::YuShui => "雨水",
            JieQi::JingZhe => "惊蛰",
            JieQi::ChunFen => "春分",
            JieQi::QingMing => "清明",
            JieQi::GuYu => "谷雨",
            JieQi::LiXia => "立夏",
            JieQi::XiaoMan => "小满",
            JieQi::MangZhong => "芒种",
            JieQi::XiaZhi => "夏至",
            JieQi::XiaoShu => "小暑",
            JieQi::DaShu => "大暑",
            JieQi::LiQiu => "立秋",
            JieQi::ChuShu => "处暑",
            JieQi::BaiLu => "白露",
            JieQi::QiuFen => "秋分",
            JieQi::HanLu => "寒露",
            JieQi::ShuangJiang => "霜降",
            JieQi::LiDong => "立冬",
            JieQi::XiaoXue => "小雪",
            JieQi::DaXue => "大雪",
            JieQi::DongZhi => "冬至",
            JieQi::XiaoHan => "小寒",
            JieQi::DaHan => "大寒",
        }
    }

    /// 将节气枚举转换为索引（0-23）
    pub fn to_index(self) -> u8 {
        self as u8
    }

    /// 从索引获取节气
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(JieQi::LiChun),
            1 => Some(JieQi::YuShui),
            2 => Some(JieQi::JingZhe),
            3 => Some(JieQi::ChunFen),
            4 => Some(JieQi::QingMing),
            5 => Some(JieQi::GuYu),
            6 => Some(JieQi::LiXia),
            7 => Some(JieQi::XiaoMan),
            8 => Some(JieQi::MangZhong),
            9 => Some(JieQi::XiaZhi),
            10 => Some(JieQi::XiaoShu),
            11 => Some(JieQi::DaShu),
            12 => Some(JieQi::LiQiu),
            13 => Some(JieQi::ChuShu),
            14 => Some(JieQi::BaiLu),
            15 => Some(JieQi::QiuFen),
            16 => Some(JieQi::HanLu),
            17 => Some(JieQi::ShuangJiang),
            18 => Some(JieQi::LiDong),
            19 => Some(JieQi::XiaoXue),
            20 => Some(JieQi::DaXue),
            21 => Some(JieQi::DongZhi),
            22 => Some(JieQi::XiaoHan),
            23 => Some(JieQi::DaHan),
            _ => None,
        }
    }

    /// 计算某年的所有节气
    ///
    /// # 参数
    /// - `year`: 公历年
    ///
    /// # 返回值
    /// 节气信息列表（含儒略日和节气枚举）
    pub fn get_all_jieqi_by_solar_year(year: i32) -> Vec<JieQiInfo> {
        let mut jieqis = Vec::with_capacity(24);
        // 节气太阳黄经：立春315°，雨水330°，每个节气增加15°
        for i in 0..24 {
            if let Some(jieqi) = JieQi::from_index(i as u8) {
                let jd = jieqi.calc_jieqi_jd(year);
                jieqis.push(JieQiInfo {
                    jd: JulianDay(jd),
                    jq_index: jieqi,
                });
            }
        }

        jieqis
    }

    /// 计算单个节气的儒略日
    ///
    /// # 参数
    /// - `year`: 公历年
    ///
    /// # 返回值
    /// 节气发生的儒略日
    pub fn calc_jieqi_jd(self, year: i32) -> f64 {
        let lon = 315.0 + self.to_index() as f64 * 15.0;
        // 估算节气日期（每月4/19日左右）
        let month = floor(lon / 30.0) as u8 + 1;
        let day = if lon % 30.0 < 15.0 { 4.0 } else { 19.0 };

        let solar = SolarDate {
            year,
            month,
            day: day as u8,
            hour: 12,
            minute: 0,
            second: 0.0,
        };

        let jd: JulianDay = solar.into();
        let mut jd = jd.0;

        // 迭代精确计算
        const MAX_ITERATIONS: usize = 20;
        const CONVERGENCE_THRESHOLD: f64 = 1e-6;

        for _ in 0..MAX_ITERATIONS {
            let t = (jd - J2000) / 36525.0; // 儒略世纪数
            let sun_lon = sun_longitude(t); // 太阳黄经（度）

            // 计算角度差，处理360度环绕
            let mut delta = lon - sun_lon;
            if delta > 180.0 {
                delta -= 360.0;
            } else if delta < -180.0 {
                delta += 360.0;
            }

            // 使用更精确的步长计算
            jd += delta * 0.985647366 / 360.0; // 转换为天数的比例

            if delta.abs() < CONVERGENCE_THRESHOLD {
                break;
            }
        }

        jd
    }
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
    let m_rad = (357.52911 + 35999.05029 * t - 0.0001537 * t * t).to_radians();

    // 太阳中心差（度）
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * sin(m_rad)
        + (0.019993 - 0.000101 * t) * sin(2.0 * m_rad)
        + 0.000289 * sin(3.0 * m_rad);
    let c_deg = c.to_degrees();

    // 真黄经（度）
    let true_lon = l0 + c_deg;

    // 修正章动（简化计算）
    let omega_rad =
        (125.04452 - 1934.136261 * t + 0.0020708 * t * t + pow(t, 3.0) / 450000.0).to_radians();
    let delta_psi = -0.00478 * sin(omega_rad);

    let lon = true_lon + delta_psi.to_degrees();

    // 转换为0-360度
    lon - 360.0 * floor(lon / 360.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jieqi_from_index() {
        assert_eq!(JieQi::from_index(0), Some(JieQi::LiChun));
        assert_eq!(JieQi::from_index(23), Some(JieQi::DaHan));
        assert_eq!(JieQi::from_index(24), None);
    }

    #[test]
    fn test_sun_longitude() {
        // 测试J2000时刻的太阳黄经
        let t = 0.0;
        let lon = sun_longitude(t);
        assert!(lon >= 0.0 && lon < 360.0);
    }
}
