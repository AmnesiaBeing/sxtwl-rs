//! 岁差计算模块
//!
//! 实现多种岁差模型的坐标转换，包括：
//! - IAU1976 岁差模型
//! - IAU2000 岁差模型  
//! - P03 岁差模型
//!
//! 岁差是地球自转轴在空间中的长期缓慢运动，主要由月球和太阳的引力引起。

use crate::astronomy::coefficients::{PRECE_TAB_IAU1976, PRECE_TAB_IAU2000, PRECE_TAB_P03};
use crate::astronomy::llr_conv;
use crate::astronomy::math_utils::{Vector3, normalize_rad};
use crate::consts::RAD;
use libm::{asin, atan2, cos, sin};

// =============================================================================
// 类型定义
// =============================================================================

/// 岁差计算模型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecessionModel {
    /// IAU1976 岁差模型（Lieske 模型）
    IAU1976,
    /// IAU2000 岁差模型
    IAU2000,
    /// P03 岁差模型（Capitaine 等人，2003）
    P03,
}

/// 岁差量类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecessionQuantity {
    /// 黄经岁差角 (φ)
    Fi,
    /// 倾角相关参数 (ω)
    W,
    /// 赤经岁差 (P)
    P,
    /// 赤纬岁差 (Q)
    Q,
    /// 黄赤交角 (ε)
    E,
    /// 黄道岁差 (χ)
    X,
    /// 赤道岁差 (π)
    Pi,
    /// 赤道岁差 (II)
    II,
    /// 赤道岁差 (p)
    P_,
    /// 赤道岁差 (θ)
    Th,
    /// 赤道岁差 (Z)
    Z,
    /// 赤道岁差 (z)
    Z_,
}

// =============================================================================
// 岁差量计算
// =============================================================================

/// 计算指定岁差量的值
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
/// - `quantity`: 要计算的岁差量类型
/// - `model`: 使用的岁差模型
///
/// # 返回值
/// 岁差量值（弧度）
///
/// # 说明
/// 不同岁差模型使用不同的多项式系数表
pub fn calculate_precession_quantity(
    julian_centuries: f64,
    quantity: &PrecessionQuantity,
    model: &PrecessionModel,
) -> f64 {
    let (coefficient_count, coefficients) = match model {
        PrecessionModel::IAU1976 => (4, &PRECE_TAB_IAU1976[..]),
        PrecessionModel::IAU2000 => (6, &PRECE_TAB_IAU2000[..]),
        PrecessionModel::P03 => (6, &PRECE_TAB_P03[..]),
    };

    let quantity_index = match quantity {
        PrecessionQuantity::Fi => 0,
        PrecessionQuantity::W => 1,
        PrecessionQuantity::P => 2,
        PrecessionQuantity::Q => 3,
        PrecessionQuantity::E => 4,
        PrecessionQuantity::X => 5,
        PrecessionQuantity::Pi => 6,
        PrecessionQuantity::II => 7,
        PrecessionQuantity::P_ => 8,
        PrecessionQuantity::Th => 9,
        PrecessionQuantity::Z => 10,
        PrecessionQuantity::Z_ => 11,
    };

    // 计算多项式值
    let mut result = 0.0;
    let mut time_power = 1.0;

    for i in 0..coefficient_count {
        let coefficient_index = quantity_index * coefficient_count + i;
        result += coefficients[coefficient_index] * time_power;
        time_power *= julian_centuries;
    }

    // 转换为弧度（原始系数单位为角秒）
    result / RAD
}

/// 计算 P03 模型的黄赤交角
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
///
/// # 返回值
/// 黄赤交角（弧度）
///
/// # 说明
/// 基于 Capitaine 等人 2003 年的模型
pub fn calculate_obliquity_p03(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;

    // 多项式系数基于 P03 模型
    (84381.4060 - 46.836769 * t - 0.0001831 * t2 + 0.00200340 * t3 - 5.76e-7 * t4 - 4.34e-8 * t5)
        / RAD
}

// =============================================================================
// 赤道坐标转换
// =============================================================================

/// 将赤道坐标从 J2000.0 历元转换到指定历元
///
/// # 参数
/// - `julian_centuries`: 目标历元相对于 J2000.0 的儒略世纪数
/// - `equatorial_coords`: J2000.0 历元的赤道坐标 (赤经, 赤纬, 距离)
/// - `model`: 使用的岁差模型
///
/// # 返回值
/// 目标历元的赤道坐标
pub fn transform_equatorial_j2000_to_date(
    julian_centuries: f64,
    equatorial_coords: Vector3,
    model: PrecessionModel,
) -> Vector3 {
    let zeta = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Z, &model);
    let z = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Z_, &model);
    let theta = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Th, &model);

    let right_ascension = equatorial_coords.x;
    let declination = equatorial_coords.y;
    let distance = equatorial_coords.z;

    let cos_declination = cos(declination);
    let cos_theta = cos(theta);
    let sin_declination = sin(declination);
    let sin_theta = sin(theta);

    // 应用岁差旋转矩阵
    let a = cos_declination * sin(right_ascension + zeta);
    let b = cos_theta * cos_declination * cos(right_ascension + zeta) - sin_theta * sin_declination;
    let c = sin_theta * cos_declination * cos(right_ascension + zeta) + cos_theta * sin_declination;

    Vector3::new(normalize_rad(atan2(a, b) + z), asin(c), distance)
}

/// 将赤道坐标从指定历元转换到 J2000.0 历元
///
/// # 参数
/// - `julian_centuries`: 原始历元相对于 J2000.0 的儒略世纪数
/// - `equatorial_coords`: 原始历元的赤道坐标
/// - `model`: 使用的岁差模型
///
/// # 返回值
/// J2000.0 历元的赤道坐标
pub fn transform_equatorial_date_to_j2000(
    julian_centuries: f64,
    equatorial_coords: Vector3,
    model: PrecessionModel,
) -> Vector3 {
    let zeta = -calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Z_, &model);
    let z = -calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Z, &model);
    let theta = -calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Th, &model);

    let right_ascension = equatorial_coords.x;
    let declination = equatorial_coords.y;
    let distance = equatorial_coords.z;

    let cos_declination = cos(declination);
    let cos_theta = cos(theta);
    let sin_declination = sin(declination);
    let sin_theta = sin(theta);

    // 应用逆岁差旋转矩阵
    let a = cos_declination * sin(right_ascension + zeta);
    let b = cos_theta * cos_declination * cos(right_ascension + zeta) - sin_theta * sin_declination;
    let c = sin_theta * cos_declination * cos(right_ascension + zeta) + cos_theta * sin_declination;

    Vector3::new(normalize_rad(atan2(a, b) + z), asin(c), distance)
}

// =============================================================================
// 黄道坐标转换
// =============================================================================

/// 将黄道坐标从 J2000.0 历元转换到指定历元
///
/// # 参数
/// - `julian_centuries`: 目标历元相对于 J2000.0 的儒略世纪数
/// - `ecliptic_coords`: J2000.0 历元的黄道坐标 (黄经, 黄纬, 距离)
/// - `model`: 使用的岁差模型
///
/// # 返回值
/// 目标历元的黄道坐标
pub fn transform_ecliptic_j2000_to_date(
    julian_centuries: f64,
    ecliptic_coords: Vector3,
    model: PrecessionModel,
) -> Vector3 {
    let mut transformed = ecliptic_coords;

    // 应用三次旋转转换黄道坐标
    let phi = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Fi, &model);
    let omega = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::W, &model);
    let chi = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::X, &model);
    let epsilon = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::E, &model);

    // 第一步：黄经岁差
    transformed.x += phi;

    // 第二步：绕新黄极旋转
    transformed = llr_conv(transformed, omega);

    // 第三步：黄道岁差
    transformed.x -= chi;

    // 第四步：黄赤交角变化
    transformed = llr_conv(transformed, -epsilon);

    transformed
}

/// 将黄道坐标从指定历元转换到 J2000.0 历元
///
/// # 参数
/// - `julian_centuries`: 原始历元相对于 J2000.0 的儒略世纪数
/// - `ecliptic_coords`: 原始历元的黄道坐标
/// - `model`: 使用的岁差模型
///
/// # 返回值
/// J2000.0 历元的黄道坐标
pub fn transform_ecliptic_date_to_j2000(
    julian_centuries: f64,
    ecliptic_coords: Vector3,
    model: PrecessionModel,
) -> Vector3 {
    let mut transformed = ecliptic_coords;

    // 应用逆转换步骤
    let phi = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::Fi, &model);
    let omega = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::W, &model);
    let chi = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::X, &model);
    let epsilon = calculate_precession_quantity(julian_centuries, &PrecessionQuantity::E, &model);

    // 逆序执行转换步骤
    transformed = llr_conv(transformed, epsilon);
    transformed.x += chi;
    transformed = llr_conv(transformed, -omega);
    transformed.x -= phi;

    Vector3::new(normalize_rad(transformed.x), transformed.y, transformed.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precession_quantity_calculation() {
        let result =
            calculate_precession_quantity(0.1, &PrecessionQuantity::Z, &PrecessionModel::P03);
        // 验证结果在合理范围内
        assert!(result.abs() < 0.1);
    }

    #[test]
    fn test_obliquity_calculation() {
        let obliquity = calculate_obliquity_p03(0.1);
        // 黄赤交角应该在合理范围内（约23.5度）
        assert!(obliquity > 0.4 && obliquity < 0.5);
    }

    #[test]
    fn test_equatorial_transformation() {
        let j2000_coords = Vector3::new(1.0, 0.5, 1.0);
        let transformed =
            transform_equatorial_j2000_to_date(0.1, j2000_coords, PrecessionModel::P03);

        // 验证坐标仍然有效
        assert!(transformed.x.abs() < 10.0);
        assert!(transformed.y.abs() <= 1.57); // 赤纬范围 [-π/2, π/2]
    }

    #[test]
    fn test_ecliptic_transformation() {
        let j2000_coords = Vector3::new(1.0, 0.2, 1.0);
        let transformed = transform_ecliptic_j2000_to_date(0.1, j2000_coords, PrecessionModel::P03);

        // 验证坐标仍然有效
        assert!(transformed.x.abs() < 10.0);
        assert!(transformed.y.abs() <= 1.57);
    }
}
