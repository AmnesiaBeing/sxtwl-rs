//! ΔT计算
//! 实现TD-UT时间差的计算（地球自转的不规则性修正）

use crate::astronomy::coefficients::DT_AT;
use crate::consts::J2000;
use libm::{fabs, pow, sin, sqrt};

/// 二次曲线外推
/// y: 年份
/// jsd: 加速度估计
fn dt_ext(y: f64, jsd: i32) -> f64 {
    let dy = (y - 1820.0) / 100.0;
    -20.0 + (jsd as f64) * dy * dy
}

/// 计算世界时与原子时之差
/// y: 年份
pub fn dt_calc(y: f64) -> f64 {
    let y0 = DT_AT[DT_AT.len() - 2]; // 表中最后一年
    let t0 = DT_AT[DT_AT.len() - 1]; // 表中最后一年的deltatT

    if y >= y0 {
        let jsd = 31; // sjd是y1年之后的加速度估计。瑞士星历表jsd=31,NASA网站jsd=32,skmap的jsd=29
        if y > y0 + 100.0 {
            return dt_ext(y, jsd);
        }
        let v = dt_ext(y, jsd); // 二次曲线外推
        let dv = dt_ext(y0, jsd) - t0; // ye年的二次外推与te的差
        return v - dv * (y0 + 100.0 - y) / 100.0;
    }

    // 查找对应的区间
    let mut i = 0;
    for idx in (0..DT_AT.len() - 5).step_by(5) {
        if y < DT_AT[idx + 5] {
            i = idx;
            break;
        }
    }

    let d = &DT_AT[i..i + 5];
    let t1 = (y - d[0]) / (DT_AT[i + 5] - d[0]) * 10.0;
    let t2 = t1 * t1;
    let t3 = t2 * t1;

    d[1] + d[2] * t1 + d[3] * t2 + d[4] * t3
}

/// 传入儒略日(J2000起算)，计算TD-UT(单位:日)
/// t: 从J2000起算的儒略日
pub fn dt_t(t: f64) -> f64 {
    dt_calc(t / 365.2425 + 2000.0) / 86400.0
}
