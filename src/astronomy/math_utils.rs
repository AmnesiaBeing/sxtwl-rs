//! 数学工具函数
//! 提供天文计算中常用的数学运算

use core::f64::consts::{PI, TAU as PI2};
use libm::{asin, atan2, cos, floor, fmod, sin, sqrt, tan};

/// 角度转弧度
#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// 弧度转角度
#[inline]
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

/// 将角度归一化到 [0.0, 360.0) 范围
#[inline]
pub fn normalize_angle(angle: f64) -> f64 {
    let mut result = angle % 360.0;
    if result < 0.0 {
        result += 360.0;
    }
    result
}

/// 将弧度归一化到 [0.0, TAU) 范围
#[inline]
pub fn normalize_rad(rad: f64) -> f64 {
    let mut result = rad % PI2;
    if result < 0.0 {
        result += PI2;
    }
    result
}

/// 计算两个角度之间的最小差值（考虑周期性）
///
/// # 返回值
/// 范围在 [-180.0, 180.0] 的角度差
#[inline]
pub fn angle_diff(angle1: f64, angle2: f64) -> f64 {
    let diff = normalize_angle(angle1) - normalize_angle(angle2);
    if diff > 180.0 {
        diff - 360.0
    } else if diff < -180.0 {
        diff + 360.0
    } else {
        diff
    }
}

/// 计算两个弧度之间的最小差值（考虑周期性）
///
/// # 返回值  
/// 范围在 [-PI, PI] 的弧度差
#[inline]
pub fn rad_diff(rad1: f64, rad2: f64) -> f64 {
    let diff = normalize_rad(rad1) - normalize_rad(rad2);
    if diff > PI {
        diff - PI2
    } else if diff < -PI {
        diff + PI2
    } else {
        diff
    }
}

/// 使用霍纳法则计算多项式值
///
/// # 参数
/// - `coeffs`: 多项式系数，从最高次到常数项
/// - `x`: 自变量值
pub fn polynomial_evaluate(coeffs: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    for &coeff in coeffs.iter() {
        result = result * x + coeff;
    }
    result
}

/// 将弧度归一化到 [-PI, PI] 范围
#[inline]
pub fn rad2rrad(v: f64) -> f64 {
    let v = fmod(v, PI2);
    if v <= -PI {
        v + PI2
    } else if v > PI {
        v - PI2
    } else {
        v
    }
}

/// 临界余数 (a 与最近的整倍数 b 相差的距离)
#[inline]
pub fn mod2(a: f64, b: f64) -> f64 {
    let c = a / b;
    let c = c - floor(c);
    let c = if c > 0.5 { c - 1.0 } else { c };
    c * b
}

/// 球面坐标转直角坐标
///
/// # 参数
/// - `jw`: 球面坐标 (经度, 纬度, 半径)
pub fn llr2xyz(jw: Vector3) -> Vector3 {
    let j = jw.x;
    let w = jw.y;
    let r = jw.z;
    Vector3::new(r * cos(w) * cos(j), r * cos(w) * sin(j), r * sin(w))
}

/// 直角坐标转球面坐标
///
/// # 参数
/// - `xyz`: 直角坐标 (x, y, z)
pub fn xyz2llr(xyz: Vector3) -> Vector3 {
    let x = xyz.x;
    let y = xyz.y;
    let z = xyz.z;

    let r = sqrt(x * x + y * y + z * z);
    let w = asin(z / r);
    let j = normalize_rad(atan2(y, x));

    Vector3::new(j, w, r)
}

/// 球面坐标旋转
///
/// # 参数
/// - `jw`: 原始球面坐标
/// - `e`: 旋转角度
pub fn llr_conv(jw: Vector3, e: f64) -> Vector3 {
    let j = jw.x;
    let w = jw.y;
    let cos_j = cos(j);
    let sin_j = sin(j);
    let cos_w = cos(w);
    let sin_w = sin(w);
    let cos_e = cos(e);
    let sin_e = sin(e);

    let new_j = atan2(sin_j * cos_e - tan(w) * sin_e, cos_j);
    let new_w = asin(cos_e * sin_w + sin_e * cos_w * sin_j);

    Vector3::new(normalize_rad(new_j), new_w, jw.z)
}

/// 日心球面坐标转地心球面坐标
///
/// # 参数
/// - `z`: 星体球面坐标
/// - `a`: 地球球面坐标
pub fn h2g(z: Vector3, a: Vector3) -> Vector3 {
    let a_xyz = llr2xyz(a);
    let z_xyz = llr2xyz(z);

    let result_xyz = Vector3::new(z_xyz.x - a_xyz.x, z_xyz.y - a_xyz.y, z_xyz.z - a_xyz.z);

    xyz2llr(result_xyz)
}

/// 带符号的取余函数
#[inline]
pub fn fmod2(v: f64, n: f64) -> f64 {
    ((v % n) + n) % n
}

/// 二次幂
#[inline]
pub fn pow2(v: f64) -> f64 {
    v * v
}

/// 三维向量
#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// 创建新向量
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 零向量
    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// 点积
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 叉积
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// 向量模长
    #[inline]
    pub fn norm(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// 单位向量
    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        if norm > 0.0 {
            Self {
                x: self.x / norm,
                y: self.y / norm,
                z: self.z / norm,
            }
        } else {
            *self
        }
    }

    /// 向量缩放
    #[inline]
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    /// 向量加法
    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// 向量减法
    #[inline]
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// 二维向量
#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// 创建新向量
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// 零向量
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// 点积
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// 向量模长
    #[inline]
    pub fn norm(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y)
    }

    /// 单位向量
    pub fn normalized(&self) -> Self {
        let norm = self.norm();
        if norm > 0.0 {
            Self {
                x: self.x / norm,
                y: self.y / norm,
            }
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_conversions() {
        assert!((deg_to_rad(180.0) - PI).abs() < 1e-10);
        assert!((rad_to_deg(PI) - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalization() {
        assert!((normalize_angle(450.0) - 90.0).abs() < 1e-10);
        assert!((normalize_angle(-90.0) - 270.0).abs() < 1e-10);
        assert!((normalize_rad(3.0 * PI) - PI).abs() < 1e-10);
    }
}
