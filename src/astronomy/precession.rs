//! 岁差计算
//! 实现各种岁差模型（IAU1976、IAU2000、P03）的坐标转换

use crate::astronomy::coefficients::{PRECE_TAB_IAU1976, PRECE_TAB_IAU2000, PRECE_TAB_P03};
use crate::astronomy::llr_conv;
use crate::astronomy::math_utils::{Vector3, rad2mrad};
use crate::consts::RAD;
use libm::{asin, atan2, cos, sin};

// 岁差模型枚举
#[derive(PartialEq)]
pub enum PrecessionModel {
    IAU1976,
    IAU2000,
    P03,
}

// 岁差量名称枚举
#[derive(PartialEq)]
pub enum PrecessionQuantity {
    Fi, // fi
    W,  // w
    P,  // P
    Q,  // Q
    E,  // E
    X,  // x
    Pi, // pi
    II, // II
    P_, // p
    Th, // th
    Z,  // Z
    Z_, // z
}

/// 计算岁差量
/// t: 儒略世纪数
/// sc: 岁差量名称
/// mx: 岁差模型
pub fn prece(t: f64, sc: PrecessionQuantity, mx: &PrecessionModel) -> f64 {
    let (n, p) = match mx {
        PrecessionModel::IAU1976 => (4, &PRECE_TAB_IAU1976[..]),
        PrecessionModel::IAU2000 => (6, &PRECE_TAB_IAU2000[..]),
        PrecessionModel::P03 => (6, &PRECE_TAB_P03[..]),
    };

    let isc = match sc {
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

    let mut result = 0.0;
    let mut tn = 1.0;
    for i in 0..n {
        result += p[isc * n + i] * tn;
        tn *= t;
    }

    result / RAD
}

/// 返回P03黄赤交角，t是世纪数
pub fn hcjj(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;
    (84381.4060 - 46.836769 * t - 0.0001831 * t2 + 0.00200340 * t3 - 5.76e-7 * t4 - 4.34e-8 * t5)
        / RAD
}

/// J2000赤道转Date赤道
pub fn cdllr_j2d(t: f64, llr: Vector3, mx: &PrecessionModel) -> Vector3 {
    let z_val = prece(t, PrecessionQuantity::Z, mx) + llr.x;
    let z_small = prece(t, PrecessionQuantity::Z_, mx);
    let th = prece(t, PrecessionQuantity::Th, mx);

    let cos_w = cos(llr.y);
    let cos_h = cos(th);
    let sin_w = sin(llr.y);
    let sin_h = sin(th);

    let a = cos_w * sin(z_val);
    let b = cos_h * cos_w * cos(z_val) - sin_h * sin_w;
    let c = sin_h * cos_w * cos(z_val) + cos_h * sin_w;

    Vector3::new(rad2mrad(atan2(a, b) + z_small), asin(c), llr.z)
}

/// Date赤道转J2000赤道
pub fn cdllr_d2j(t: f64, llr: Vector3, mx: &PrecessionModel) -> Vector3 {
    let z_val = -prece(t, PrecessionQuantity::Z_, mx) + llr.x;
    let z_small = -prece(t, PrecessionQuantity::Z, mx);
    let th = -prece(t, PrecessionQuantity::Th, mx);

    let cos_w = cos(llr.y);
    let cos_h = cos(th);
    let sin_w = sin(llr.y);
    let sin_h = sin(th);

    let a = cos_w * sin(z_val);
    let b = cos_h * cos_w * cos(z_val) - sin_h * sin_w;
    let c = sin_h * cos_w * cos(z_val) + cos_h * sin_w;

    Vector3::new(rad2mrad(atan2(a, b) + z_small), asin(c), llr.z)
}

/// 黄道球面坐标_J2000转Date分点，t为儒略世纪数
pub fn hdllr_j2d(t: f64, llr: Vector3, mx: &PrecessionModel) -> Vector3 {
    // J2000黄道旋转到Date黄道(球面对球面)
    let mut r = Vector3::new(llr.x, llr.y, llr.z);
    r.x += prece(t, PrecessionQuantity::Fi, mx);
    r = llr_conv(r, prece(t, PrecessionQuantity::W, mx));
    r.x -= prece(t, PrecessionQuantity::X, mx);
    r = llr_conv(r, -prece(t, PrecessionQuantity::E, mx));
    r
}

/// 黄道球面坐标_Date分点转J2000，t为儒略世纪数
pub fn hdllr_d2j(t: f64, llr: Vector3, mx: &PrecessionModel) -> Vector3 {
    let mut r = Vector3::new(llr.x, llr.y, llr.z);
    r = llr_conv(r, prece(t, PrecessionQuantity::E, mx));
    r.x += prece(t, PrecessionQuantity::X, mx);
    r = llr_conv(r, -prece(t, PrecessionQuantity::W, mx));
    r.x -= prece(t, PrecessionQuantity::Fi, mx);
    Vector3::new(rad2mrad(r.x), r.y, r.z)
}
