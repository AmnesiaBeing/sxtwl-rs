//! 行星轨道计算
//! 实现太阳、月球和其他行星的坐标计算

use crate::{
    astronomy::{
        Vector2, Vector3, XL0, XL0_XZB, XL1, dt_t, hcjj, llr_conv, nutation_lon2, rad2rrad,
    },
    consts::{CS_R_EAR, CS_S_MOON, RAD},
};
use core::f64::consts::{PI, TAU as PI2};
use libm::{acos, asin, atan2, cos, fabs, floor, sin};

/// xt星体,zn坐标号,t儒略世纪数,n计算项数
pub fn xl0_calc(xt: usize, zn: usize, t: f64, n: i32) -> f64 {
    let t = t / 10.0; // 转为儒略千年数
    let mut v = 0.0;
    let mut tn = 1.0;

    let f = XL0[xt];
    let pn = zn * 6 + 1;
    let n0_total = f[pn + 1] - f[pn]; // N0序列总数

    for i in 0..6 {
        let n1 = f[pn + i];
        let n2 = f[pn + 1 + i];
        let n0 = n2 - n1;

        if n0 == 0.0 {
            tn *= t;
            continue;
        }

        let n_val = if n < 0 {
            n2
        } else {
            let mut n_calc = floor(3.0 * n as f64 * n0 / n0_total + 0.5) + n1;
            if i > 0 {
                n_calc += 3.0;
            }
            if n_calc > n2 { n2 } else { n_calc }
        };

        let mut c = 0.0;
        let mut j = n1 as i32;
        while j < n_val as i32 {
            c += f[j as usize] * cos(f[j as usize + 1] + t * f[j as usize + 2]);
            j += 3;
        }

        v += c * tn;
        tn *= t;
    }

    v /= f[0];

    if xt == 0 {
        // 地球
        let t2 = t * t;
        let t3 = t2 * t;
        match zn {
            0 => v += (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t3) / RAD,
            1 => v += (0.0000 + 0.0004 * t + 0.0004 * t2 - 0.0026 * t3) / RAD,
            2 => v += (-0.0020 + 0.0044 * t + 0.0213 * t2 - 0.0250 * t3) / 1000000.0,
            _ => (),
        }
    } else {
        // 其它行星
        let dv = XL0_XZB[(xt - 1) * 3 + zn];
        match zn {
            0 => v += -3.0 * t / RAD,
            2 => v += dv / 1000000.0,
            _ => v += dv / RAD,
        }
    }

    v
}

/// xt星体,T儒略世纪数,n1,n2,n3为各坐标所取的项数
pub fn p_coord(xt: usize, t: f64, n1: i32, n2: i32, n3: i32) -> Vector3 {
    let mut z = Vector3::new(0.0, 0.0, 0.0);

    if xt < 8 {
        z.x = xl0_calc(xt, 0, t, n1);
        z.y = xl0_calc(xt, 1, t, n2);
        z.z = xl0_calc(xt, 2, t, n3);
    } else if xt == 8 {
        // 冥王星
        unimplemented!()
    } else if xt == 9 {
        // 太阳
        z.x = 0.0;
        z.y = 0.0;
        z.z = 0.0;
    }

    z
}

/// 返回地球坐标,t为世纪数
pub fn e_coord(t: f64, n1: i32, n2: i32, n3: i32) -> Vector3 {
    Vector3::new(
        xl0_calc(0, 0, t, n1),
        xl0_calc(0, 1, t, n2),
        xl0_calc(0, 2, t, n3),
    )
}

/// 计算月亮坐标
pub fn xl1_calc(zn: usize, t: f64, n: i32) -> f64 {
    let ob = XL1[zn];
    let mut v = 0.0;
    let mut tn = 1.0;

    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;
    let tx = t - 10.0;

    if zn == 0 {
        // 月球平黄经(弧度)
        v += (3.81034409 + 8399.684730072 * t - 3.319e-05 * t2 + 3.11e-08 * t3 - 2.033e-10 * t4)
            * RAD;
        // 岁差(角秒)
        v += 5028.792262 * t + 1.1124406 * t2 + 0.00007699 * t3
            - 0.000023479 * t4
            - 0.0000000178 * t5;
        // 对公元3000年至公元5000年的拟合
        if tx > 0.0 {
            v += -0.866 + 1.43 * tx + 0.054 * tx * tx;
        }
    }

    let t2 = t2 / 10000.0;
    let t3 = t3 / 100000000.0;
    let t4 = t4 / 100000000.0;

    let mut n_adj = n * 6;
    if n_adj < 0 {
        n_adj = 3;
    }

    for i in 0..3 {
        let f = ob[i];
        let f_len = f.len();

        let n_val = floor(n_adj as f64 * f_len as f64 / 3.0 + 0.5) as usize;
        let n_val = if i > 0 { n_val + 6 } else { n_val };
        let n_val = if n_val >= f_len { f_len } else { n_val };

        let mut c = 0.0;
        let mut j = 0;
        while j < n_val {
            c +=
                f[j] * cos(f[j + 1] + t * f[j + 2] + t2 * f[j + 3] + t3 * f[j + 4] + t4 * f[j + 5]);
            j += 6;
        }

        v += c * tn;
        tn *= t;
    }

    if zn != 2 {
        v /= RAD;
    }

    v
}

/// 返回月球坐标,n1,n2,n3为各坐标所取的项数
pub fn m_coord(t: f64, n1: i32, n2: i32, n3: i32) -> Vector3 {
    Vector3::new(xl1_calc(0, t, n1), xl1_calc(1, t, n2), xl1_calc(2, t, n3))
}

/// 返回朔日的编号,jd应在朔日附近，允许误差数天
pub fn shuo_n(jd: f64) -> i32 {
    floor((jd + 8.0) / 29.5306) as i32
}

/// 太阳光行差,t是世纪数
pub fn gxc_sun_lon(t: f64) -> f64 {
    let v = -0.043126 + 628.301955 * t - 0.000002732 * t * t; // 平近点角
    let e = 0.016708634 - 0.000042037 * t - 0.0000001267 * t * t;
    (-20.49552 * (1.0 + e * cos(v))) / RAD // 黄经光行差
}

pub const GXC_SUN_LAT: f64 = 0.0; // 黄纬光行差
pub const GXC_MOON_LON: f64 = -3.4E-6; // 月球经度光行差

/// 月球纬度光行差
pub fn gxc_moon_lat(t: f64) -> f64 {
    0.063 * sin(0.057 + 8433.4662 * t + 0.000064 * t * t) / RAD
}

/// 返回格林尼治平恒星时
pub fn p_gst(t: f64, dt: f64) -> f64 {
    let t_century = (t + dt) / 36525.0;
    let t2 = t_century * t_century;
    let t3 = t2 * t_century;
    let t4 = t3 * t_century;

    PI2 * (0.7790572732640 + 1.002_737_811_911_354_6 * t)
        + (0.014506 + 4612.15739966 * t_century + 1.39667721 * t2 - 0.00009344 * t3
            + 0.00001882 * t4)
            / RAD
}

/// 传入力学时J2000起算日数，返回平恒星时
pub fn p_gst2(jd: f64) -> f64 {
    let dt = dt_t(jd);
    p_gst(jd - dt, dt)
}

/// 太阳升降计算
pub fn sun_sheng_j(jd: f64, l: f64, fa: f64, sj: f64) -> f64 {
    let mut jd = floor(jd + 0.5) - l / PI2;

    for _ in 0..2 {
        let t = jd / 36525.0;
        let e = (84381.4060 - 46.836769 * t) / RAD; // 黄赤交角
        let t_mech = t + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / 86400.0 / 36525.0; // 儒略世纪年数,力学时

        let j = (48950621.66 + 6283319653.318 * t_mech + 53.0 * t_mech * t_mech - 994.0
            + 334166.0 * cos(4.669257 + 628.307585 * t_mech)
            + 3489.0 * cos(4.6261 + 1256.61517 * t_mech)
            + 2060.6 * cos(2.67823 + 628.307585 * t_mech) * t_mech)
            / 10000000.0;

        let sin_j = sin(j);
        let cos_j = cos(j);

        // 恒星时(子午圈位置)
        let gst = (0.7790572732640 + 1.002_737_811_911_354_6 * jd) * 2.0 * PI
            + (0.014506 + 4612.15739966 * t + 1.39667721 * t * t) / RAD;

        let a = atan2(sin_j * cos(e), cos_j); // 太阳赤经
        let d = asin(sin(e) * sin_j); // 太阳赤纬

        let cos_h0 = (sin(-50.0 * 60.0 / RAD) - sin(fa) * sin(d)) / (cos(fa) * cos(d));

        if fabs(cos_h0) >= 1.0 {
            return 0.0;
        }

        jd += rad2rrad(sj * acos(cos_h0) - (gst + l - a)) / 6.28;
    }

    jd
}

/// 时差计算(高精度),t力学时儒略世纪数
pub fn pty_zty(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;

    let mut l = (1753470142.0 + 628331965331.8 * t + 5296.74 * t2 + 0.432 * t3
        - 0.1124 * t4
        - 0.00009 * t5)
        / 1000000000.0
        + PI
        - 20.5 / RAD;

    let dl = -17.2 * sin(2.1824 - 33.75705 * t) / RAD; // 黄经章
    let de = 9.2 * cos(2.1824 - 33.75705 * t) / RAD; // 交角章
    let e = hcjj(t) + de; // 真黄赤交角

    // 地球坐标
    let mut z = Vector3::new(
        xl0_calc(0, 0, t, 50) + PI + gxc_sun_lon(t) + dl,
        -(2796.0 * cos(3.1987 + 8433.46616 * t)
            + 1016.0 * cos(5.4225 + 550.75532 * t)
            + 804.0 * cos(3.88 + 522.3694 * t))
            / 1000000000.0,
        0.0,
    );

    z = llr_conv(z, e); // z太阳地心赤道坐标
    z.x -= dl * cos(e);

    l = rad2rrad(l - z.x);
    l / (2.0 * PI) // 单位是周(天)
}

/// 时差计算(低精度)
pub fn pty_zty2(t: f64) -> f64 {
    let l = (1753470142.0 + 628331965331.8 * t + 5296.74 * t * t) / 1000000000.0 + PI;

    let e = (84381.4088 - 46.836051 * t) / RAD;
    let mut z = Vector3::new(xl0_calc(0, 0, t, 5) + PI, 0.0, 0.0); // 地球坐标

    z = llr_conv(z, e); // z太阳地心赤道坐标
    let l = rad2rrad(l - z.x);

    l / (2.0 * PI) // 单位是周(天)
}

/// 地球经度计算,返回Date分点黄经
pub fn e_lon(t: f64, n: i32) -> f64 {
    xl0_calc(0, 0, t, n)
}

/// 月球经度计算,返回Date分点黄经
pub fn m_lon(t: f64, n: i32) -> f64 {
    xl1_calc(0, t, n)
}

/// 地球速度,t是世纪数
pub fn e_v(t: f64) -> f64 {
    let f = 628.307585 * t;
    628.332
        + 21.0 * sin(1.527 + f)
        + 0.44 * sin(1.48 + f * 2.0)
        + 0.129 * sin(5.82 + f) * t
        + 0.00055 * sin(4.21 + f) * t * t
}

/// 月球速度计算
pub fn m_v(t: f64) -> f64 {
    let mut v = 8399.71 - 914.0 * sin(0.7848 + 8328.691425 * t + 0.0001523 * t * t);
    v -= 179.0 * sin(2.543 + 15542.7543 * t)
        + 160.0 * sin(0.1874 + 7214.0629 * t)
        + 62.0 * sin(3.14 + 16657.3828 * t)
        + 34.0 * sin(4.827 + 16866.9323 * t)
        + 22.0 * sin(4.9 + 23871.4457 * t)
        + 12.0 * sin(2.59 + 14914.4523 * t)
        + 7.0 * sin(0.23 + 6585.7609 * t)
        + 5.0 * sin(0.9 + 25195.624 * t)
        + 5.0 * sin(2.32 - 7700.3895 * t)
        + 5.0 * sin(3.88 + 8956.9934 * t)
        + 5.0 * sin(0.49 + 7771.3771 * t);
    v
}

/// 月日视黄经的差值
pub fn ms_a_lon(t: f64, mn: i32, sn: i32) -> f64 {
    m_lon(t, mn) + GXC_MOON_LON - (e_lon(t, sn) + gxc_sun_lon(t) + PI)
}

/// 太阳视黄经
pub fn s_a_lon(t: f64, n: i32) -> f64 {
    e_lon(t, n) + nutation_lon2(t) + gxc_sun_lon(t) + PI
}

/// 已知地球真黄经求时间
pub fn e_lon_t(w: f64) -> f64 {
    let mut t = (w - 1.75347) / 628.3319653318;
    let mut v = e_v(t);
    t += (w - e_lon(t, 10)) / v;
    v = e_v(t);
    t += (w - e_lon(t, -1)) / v;
    t
}

/// 已知真月球黄经求时间
pub fn m_lon_t(w: f64) -> f64 {
    let mut t = (w - 3.81034) / 8399.70911033384;
    t += (w - m_lon(t, 3)) / 8399.70911033384;
    let v = m_v(t);
    t += (w - m_lon(t, 20)) / v;
    t += (w - m_lon(t, -1)) / v;
    t
}

/// 已知月日视黄经差求时间
pub fn ms_a_lon_t(w: f64) -> f64 {
    let mut t = (w + 1.08472) / 7771.37714500204;
    t += (w - ms_a_lon(t, 3, 3)) / 7771.37714500204;
    let v = m_v(t) - e_v(t);
    t += (w - ms_a_lon(t, 20, 10)) / v;
    t += (w - ms_a_lon(t, -1, 60)) / v;
    t
}

/// 已知太阳视黄经反求时间
pub fn s_a_lon_t(w: f64) -> f64 {
    let mut t = (w - 1.75347 - PI) / 628.3319653318;
    let mut v = e_v(t);
    t += (w - s_a_lon(t, 10)) / v;
    v = e_v(t);
    t += (w - s_a_lon(t, -1)) / v;
    t
}

/// 已知月日视黄经差求时间,高速低精度
pub fn ms_a_lon_t2(w: f64) -> f64 {
    let v = 7771.37714500204;
    let mut t = (w + 1.08472) / v;
    let t2 = t * t;
    t -= (-0.00003309 * t2
        + 0.10976 * cos(0.784758 + 8328.6914246 * t + 0.000152292 * t2)
        + 0.02224 * cos(0.18740 + 7214.0628654 * t - 0.00021848 * t2)
        - 0.03342 * cos(4.669257 + 628.307585 * t))
        / v;

    let l = m_lon(t, 20)
        - (4.8950632
            + 628.3319653318 * t
            + 0.000005297 * t * t
            + 0.0334166 * cos(4.669257 + 628.307585 * t)
            + 0.0002061 * cos(2.67823 + 628.307585 * t) * t
            + 0.000349 * cos(4.6261 + 1256.61517 * t)
            - 20.5 / RAD);

    let v = 7771.38
        - 914.0 * sin(0.7848 + 8328.691425 * t + 0.0001523 * t * t)
        - 179.0 * sin(2.543 + 15542.7543 * t)
        - 160.0 * sin(0.1874 + 7214.0629 * t);

    t += (w - l) / v;
    t
}

/// 已知太阳视黄经反求时间,高速低精度
pub fn s_a_lon_t2(w: f64) -> f64 {
    let v = 628.3319653318;
    let mut t = (w - 1.75347 - PI) / v;
    t -= (0.000005297 * t * t
        + 0.0334166 * cos(4.669257 + 628.307585 * t)
        + 0.0002061 * cos(2.67823 + 628.307585 * t) * t)
        / v;
    t += (w - e_lon(t, 8) - PI + (20.5 + 17.2 * sin(2.1824 - 33.75705 * t)) / RAD) / v;
    t
}

/// 月亮被照亮部分的比例
pub fn moon_ill(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let dm = PI / 180.0;

    let d =
        (297.8502042 + 445267.1115168 * t - 0.0016300 * t2 + t3 / 545868.0 - t4 / 113065000.0) * dm;
    let m = (357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24490000.0) * dm;
    let m_moon =
        (134.9634114 + 477198.8676313 * t + 0.0089970 * t2 + t3 / 69699.0 - t4 / 14712000.0) * dm;

    let a = PI - d
        + (-6.289 * sin(m_moon) + 2.100 * sin(m)
            - 1.274 * sin(2.0 * d - m_moon)
            - 0.658 * sin(2.0 * d)
            - 0.214 * sin(2.0 * m_moon)
            - 0.110 * sin(d))
            * dm;

    (1.0 + cos(a)) / 2.0
}

/// 转入地平纬度及地月质心距离,返回站心视半径(角秒)
pub fn moon_rad(r: f64, h: f64) -> f64 {
    CS_S_MOON / r * (1.0 + sin(h) * CS_R_EAR / r)
}

/// 求月亮近点时间和距离,t为儒略世纪数力学时
/// min: 是否为近点（true为近点，false为远点）
pub fn moon_min_r(t: f64, min: bool) -> Vector2 {
    let a = 27.55454988 / 36525.0;
    let b = if min {
        -10.3302 / 36525.0
    } else {
        3.4471 / 36525.0
    };

    let mut t_calc = b + a * floor((t - b) / a + 0.5); // 平近(远)点时间

    let mut r1;
    let mut r2;
    let mut r3;
    let mut dt;

    // 初算二次
    dt = 1.0 / 36525.0;
    r1 = xl1_calc(2, t_calc - dt, 10);
    r2 = xl1_calc(2, t_calc, 10);
    r3 = xl1_calc(2, t_calc + dt, 10);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0;

    dt = 0.5 / 36525.0;
    r1 = xl1_calc(2, t_calc - dt, 20);
    r2 = xl1_calc(2, t_calc, 20);
    r3 = xl1_calc(2, t_calc + dt, 20);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0;

    // 精算
    dt = 1200.0 / 86400.0 / 36525.0;
    r1 = xl1_calc(2, t_calc - dt, -1);
    r2 = xl1_calc(2, t_calc, -1);
    r3 = xl1_calc(2, t_calc + dt, -1);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0;
    r2 += (r1 - r3) / (r1 + r3 - 2.0 * r2) * (r3 - r1) / 8.0;

    Vector2::new(t_calc, r2)
}

/// 月亮升交点
/// asc: 是否为升交点（true为升交点，false为降交点）
pub fn moon_node(t: f64, asc: bool) -> Vector3 {
    let a = 27.21222082 / 36525.0;
    let b = if asc { 21.0 / 36525.0 } else { 35.0 / 36525.0 };

    let mut t_calc = b + a * floor((t - b) / a + 0.5); // 平升(降)交点时间

    let mut w;
    let mut w2;
    let mut v;
    let mut dt;

    dt = 0.5 / 36525.0;
    w = xl1_calc(1, t_calc, 10);
    w2 = xl1_calc(1, t_calc + dt, 10);
    v = (w2 - w) / dt;
    t_calc -= w / v;

    dt = 0.05 / 36525.0;
    w = xl1_calc(1, t_calc, 40);
    w2 = xl1_calc(1, t_calc + dt, 40);
    v = (w2 - w) / dt;
    t_calc -= w / v;

    w = xl1_calc(1, t_calc, -1);
    t_calc -= w / v;

    // 原C++代码中返回的Vector3只有部分信息，这里保持类似结构
    Vector3::new(t_calc, 0.0, 0.0)
}

/// 地球近远点
/// min: 是否为近点（true为近点，false为远点）
pub fn earth_min_r(t: f64, min: bool) -> Vector2 {
    let a = 365.25963586 / 36525.0;
    let b = if min { 1.7 / 36525.0 } else { 184.5 / 36525.0 };

    let mut t_calc = b + a * floor((t - b) / a + 0.5); // 平近(远)点时间

    let mut r1;
    let mut r2;
    let mut r3;
    let mut dt;

    // 初算二次
    dt = 3.0 / 36525.0;
    r1 = xl0_calc(0, 2, t_calc - dt, 10);
    r2 = xl0_calc(0, 2, t_calc, 10);
    r3 = xl0_calc(0, 2, t_calc + dt, 10);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0; // 误差几个小时

    dt = 0.2 / 36525.0;
    r1 = xl0_calc(0, 2, t_calc - dt, 80);
    r2 = xl0_calc(0, 2, t_calc, 80);
    r3 = xl0_calc(0, 2, t_calc + dt, 80);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0; // 误差几分钟

    // 精算
    dt = 0.01 / 36525.0;
    r1 = xl0_calc(0, 2, t_calc - dt, -1);
    r2 = xl0_calc(0, 2, t_calc, -1);
    r3 = xl0_calc(0, 2, t_calc + dt, -1);
    t_calc += (r1 - r3) / (r1 + r3 - 2.0 * r2) * dt / 2.0; // 误差小于秒
    r2 += (r1 - r3) / (r1 + r3 - 2.0 * r2) * (r3 - r1) / 8.0;

    Vector2::new(t_calc, r2)
}
