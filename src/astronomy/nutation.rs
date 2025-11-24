//! 章动计算模块
//!
//! 实现高精度和中精度的章动计算，包括 IAU2000B 模型和简化模型。
//! 章动是地球自转轴在空间中的周期性摆动，主要由月球和太阳的引力引起

use crate::{
    astronomy::{
        Vector2, Vector3,
        coefficients::{NUT_B, NUTATION_TABLE},
        normalize_rad,
    },
    consts::RAD,
};
use core::f64::consts::PI;
use libm::{cos, sin, tan};

/// 章动计算的基本参数（弧度）
/// 基于 IAU2000B 章动模型
#[derive(Debug, Clone, Copy)]
struct NutationAngles {
    /// 月球平近点角 (l)
    pub moon_mean_anomaly: f64,
    /// 太阳平近点角 (l1)
    pub sun_mean_anomaly: f64,
    /// 月球平升交角距 (f)
    pub moon_argument_latitude: f64,
    /// 日月平角距 (d)
    pub moon_sun_mean_elongation: f64,
    /// 月球轨道升交点平黄经 (Ω)
    pub moon_ascending_node: f64,
}

impl NutationAngles {
    /// 计算从 J2000.0 起算的儒略世纪数对应的章动角
    pub fn new(julian_centuries: f64) -> Self {
        let t = julian_centuries;
        let t2 = t * t;
        let t3 = t2 * t;
        let t4 = t3 * t;

        // 系数基于 IAU2000B 章动模型
        Self {
            moon_mean_anomaly: 485868.249036 + 1717915923.2178 * t + 31.8792 * t2 + 0.051635 * t3
                - 0.00024470 * t4,
            sun_mean_anomaly: 1287104.79305 + 129596581.0481 * t
                - 0.5532 * t2
                - 0.000136 * t3
                - 0.00001149 * t4,
            moon_argument_latitude: 335779.526232 + 1739527262.8478 * t
                - 12.7512 * t2
                - 0.001037 * t3
                + 0.00000417 * t4,
            moon_sun_mean_elongation: 1072260.70369 + 1602961601.2090 * t - 6.3706 * t2
                + 0.006593 * t3
                - 0.00003169 * t4,
            moon_ascending_node: 450160.398036 - 6962890.5431 * t + 7.4722 * t2 + 0.007702 * t3
                - 0.00005939 * t4,
        }
    }

    /// 将所有角度转换为弧度
    pub fn to_radians(&self) -> Self {
        Self {
            moon_mean_anomaly: self.moon_mean_anomaly / RAD,
            sun_mean_anomaly: self.sun_mean_anomaly / RAD,
            moon_argument_latitude: self.moon_argument_latitude / RAD,
            moon_sun_mean_elongation: self.moon_sun_mean_elongation / RAD,
            moon_ascending_node: self.moon_ascending_node / RAD,
        }
    }
}

/// 计算高精度章动（IAU2000B 模型）
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
/// - `min_period_days`: 最小周期（天），只计算周期大于此值的项。设为 0.0 计算所有项
///
/// # 返回值
/// 黄经章动和交角章动 (Δψ, Δε)，单位为弧度
///
/// # 说明
/// 基于 IAU2000B 章动模型，包含 77 个周期项
pub fn calculate_nutation(julian_centuries: f64, min_period_days: f64) -> Vector2 {
    let angles = NutationAngles::new(julian_centuries);
    let angles_rad = angles.to_radians();
    let t = julian_centuries;

    let mut delta_longitude = 0.0; // 黄经章动 Δψ
    let mut delta_obliquity = 0.0; // 交角章动 Δε

    // 遍历章动表，每11个元素为一组系数
    for chunk in NUTATION_TABLE.chunks_exact(11) {
        let [a1, a2, a3, a4, a5, s1, s2, c1, c2, c3, s3] = [
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10],
        ];

        // 检查周期过滤条件
        if min_period_days > 0.0 && !should_include_term(a1, a2, a3, a4, a5, min_period_days) {
            continue;
        }

        // 计算组合角
        let combination_angle = a1 * angles_rad.moon_mean_anomaly
            + a2 * angles_rad.sun_mean_anomaly
            + a3 * angles_rad.moon_argument_latitude
            + a4 * angles_rad.moon_sun_mean_elongation
            + a5 * angles_rad.moon_ascending_node;

        // 累加黄经章动项
        delta_longitude += (s1 + s2 * t) * sin(combination_angle) + c1 * cos(combination_angle);

        // 累加交角章动项
        delta_obliquity += (c2 + c3 * t) * cos(combination_angle) + s3 * sin(combination_angle);
    }

    // 转换为弧度 (原始系数单位为 0.1 微角秒)
    const CONVERSION_FACTOR: f64 = 1.0e-7 / RAD; // 10000000.0 * RAD 的倒数
    Vector2::new(
        delta_longitude * CONVERSION_FACTOR,
        delta_obliquity * CONVERSION_FACTOR,
    )
}

/// 判断是否应该包含某个章动项（基于周期过滤）
fn should_include_term(a1: f64, a2: f64, a3: f64, a4: f64, a5: f64, min_period_days: f64) -> bool {
    // 角速度 (弧秒/世纪)
    let angular_velocity = 1717915923.2178 * a1
        + 129596581.0481 * a2
        + 1739527262.8478 * a3
        + 1602961601.2090 * a4
        + 6962890.5431 * a5;

    if angular_velocity.abs() < 1e-10 {
        return true; // 避免除零，包含零周期项
    }

    // 周期 (天) = (36525 * 2π * RAD) / 角速度
    let period_days = 36525.0 * 2.0 * PI * RAD / angular_velocity.abs();
    period_days >= min_period_days
}

/// 计算赤经章动和赤纬章动对天体坐标的修正
///
/// # 参数
/// - `equatorial_coords`: 天体的赤道坐标 (赤经, 赤纬, 距离)
/// - `obliquity_ecliptic`: 黄赤交角 (弧度)
/// - `delta_psi`: 黄经章动 (弧度)
/// - `delta_epsilon`: 交角章动 (弧度)
///
/// # 返回值
/// 修正后的赤道坐标
pub fn apply_nutation_correction(
    equatorial_coords: Vector3,
    obliquity_ecliptic: f64,
    delta_psi: f64,
    delta_epsilon: f64,
) -> Vector3 {
    let mut corrected = equatorial_coords;
    let ra = corrected.x; // 赤经
    let dec = corrected.y; // 赤纬

    // 赤经章动修正
    let ra_correction = (cos(obliquity_ecliptic) + sin(obliquity_ecliptic) * sin(ra) * tan(dec))
        * delta_psi
        - cos(ra) * tan(dec) * delta_epsilon;

    // 赤纬章动修正
    let dec_correction = sin(obliquity_ecliptic) * cos(ra) * delta_psi + sin(ra) * delta_epsilon;

    corrected.x = normalize_rad(ra + ra_correction);
    corrected.y += dec_correction;

    corrected
}

/// 中精度章动计算（简化模型）
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
///
/// # 返回值
/// 黄经章动和交角章动 (Δψ, Δε)，单位为弧度
///
/// # 说明
/// 使用简化模型，适用于中等精度要求的应用
pub fn calculate_nutation_medium(julian_centuries: f64) -> Vector2 {
    let t = julian_centuries;
    let t2 = t * t;

    let mut delta_longitude = 0.0;
    let mut delta_obliquity = 0.0;

    // 遍历简化章动表，每5个元素为一组
    for chunk in NUT_B.chunks_exact(5) {
        let [arg1, arg2, arg3, coeff_psi, coeff_epsilon] =
            [chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]];

        // 计算参数角
        let argument = arg1 + arg2 * t + arg3 * t2;

        // 第一项的特殊修正
        let psi_coeff = if delta_longitude == 0.0 && delta_obliquity == 0.0 {
            coeff_psi - 1.742 * t // 首项的时间相关修正
        } else {
            coeff_psi
        };

        delta_longitude += psi_coeff * sin(argument);
        delta_obliquity += coeff_epsilon * cos(argument);
    }

    // 转换为弧度 (原始系数单位为角秒)
    const CONVERSION_FACTOR_MEDIUM: f64 = 1.0 / (100.0 * RAD);
    Vector2::new(
        delta_longitude * CONVERSION_FACTOR_MEDIUM,
        delta_obliquity * CONVERSION_FACTOR_MEDIUM,
    )
}

/// 只计算黄经章动（中精度简化版）
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
///
/// # 返回值
/// 黄经章动 Δψ (弧度)
pub fn calculate_longitude_nutation_medium(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let t2 = t * t;

    let mut delta_longitude = 0.0;

    for (i, chunk) in NUT_B.chunks_exact(5).enumerate() {
        let [arg1, arg2, arg3, coeff_psi, _] = [chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]];

        let argument = arg1 + arg2 * t + arg3 * t2;

        // 第一项的特殊修正
        let psi_coeff = if i == 0 {
            coeff_psi - 1.742 * t
        } else {
            coeff_psi
        };

        delta_longitude += psi_coeff * sin(argument);
    }

    delta_longitude / (100.0 * RAD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutation_angles_calculation() {
        let angles = NutationAngles::new(0.1);
        // 验证角度值在合理范围内
        assert!(angles.moon_mean_anomaly > 0.0);
        assert!(angles.sun_mean_anomaly > 0.0);
    }

    #[test]
    fn test_nutation_calculation() {
        let result = calculate_nutation(0.1, 0.0);
        // 验证章动值在合理范围内（几角秒级别）
        assert!(result.x.abs() < 0.01); // 小于 0.01 弧度
        assert!(result.y.abs() < 0.01);
    }

    #[test]
    fn test_medium_nutation() {
        let result = calculate_nutation_medium(0.1);
        assert!(result.x.abs() < 0.01);
        assert!(result.y.abs() < 0.01);
    }
}
