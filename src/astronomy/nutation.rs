//! 章动计算
//! 实现高精度章动计算和简化章动计算

use crate::{
    astronomy::{
        Vector2, Vector3,
        coefficients::{NUT_B, NUTATION_TABLE},
        rad2mrad,
    },
    consts::RAD,
};
use core::f64::consts::PI;
use libm::{cos, sin, tan};

/// 章动计算
/// t: J2000.0起算的儒略世纪数
/// zq: 只计算周期大于zq(天)的项
pub fn nutation(t: f64, zq: f64) -> Vector2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;

    let l = 485868.249036 + 1717915923.2178 * t + 31.8792 * t2 + 0.051635 * t3 - 0.00024470 * t4;
    let l1 = 1287104.79305 + 129596581.0481 * t - 0.5532 * t2 - 0.000136 * t3 - 0.00001149 * t4;
    let f = 335779.526232 + 1739527262.8478 * t - 12.7512 * t2 - 0.001037 * t3 + 0.00000417 * t4;
    let d = 1072260.70369 + 1602961601.2090 * t - 6.3706 * t2 + 0.006593 * t3 - 0.00003169 * t4;
    let om = 450160.398036 - 6962890.5431 * t + 7.4722 * t2 + 0.007702 * t3 - 0.00005939 * t4;

    let mut dl = 0.0;
    let mut de = 0.0;

    // 遍历章动表，每次处理11个元素
    for i in (0..NUTATION_TABLE.len()).step_by(11) {
        let b = &NUTATION_TABLE[i..i + 11];

        let c = (b[0] * l + b[1] * l1 + b[2] * f + b[3] * d + b[4] * om) / RAD;

        // 如果指定了zq，只计算周期大于zq天的项
        if zq != 0.0 {
            let q = 36526.0 * 2.0 * PI * RAD
                / (1717915923.2178 * b[0]
                    + 129596581.0481 * b[1]
                    + 1739527262.8478 * b[2]
                    + 1602961601.2090 * b[3]
                    + 6962890.5431 * b[4]);
            if q < zq {
                continue;
            }
        }

        dl += (b[5] + b[6] * t) * sin(c) + b[7] * cos(c);
        de += (b[8] + b[9] * t) * cos(c) + b[10] * sin(c);
    }

    // 返回IAU2000B章动值，dl是黄经章动，de是交角章动
    Vector2::new(dl / (10000000.0 * RAD), de / (10000000.0 * RAD))
}

/// 计算赤经章动及赤纬章动
pub fn cd_nutation(z: Vector3, e: f64, dl: f64, de: f64) -> Vector3 {
    let mut r = z;

    // 赤经章动
    r.x += (cos(e) + sin(e) * sin(z.x) * tan(z.y)) * dl - cos(z.x) * tan(z.y) * de;

    // 赤纬章动
    r.y += sin(e) * cos(z.x) * dl + sin(z.x) * de;

    r.x = rad2mrad(r.x);
    r
}

/// 中精度章动计算，t是世纪数
pub fn nutation2(t: f64) -> Vector2 {
    let t2 = t * t;
    let mut dl = 0.0;
    let mut de = 0.0;

    for i in (0..NUT_B.len()).step_by(5) {
        let b = &NUT_B[i..i + 5];
        let c = b[0] + b[1] * t + b[2] * t2;

        let a = if i == 0 { -1.742 * t } else { 0.0 };

        dl += (b[3] + a) * sin(c);
        de += b[4] * cos(c);
    }

    Vector2::new(dl / (100.0 * RAD), de / (100.0 * RAD))
}

/// 只计算黄经章动
pub fn nutation_lon2(t: f64) -> f64 {
    let t2 = t * t;
    let mut dl = 0.0;

    for i in (0..NUT_B.len()).step_by(5) {
        let b = &NUT_B[i..i + 5];
        let a = if i == 0 { -1.742 * t } else { 0.0 };
        dl += (b[3] + a) * sin(b[0] + b[1] * t + b[2] * t2);
    }

    dl / (100.0 * RAD)
}
