mod coefficients;
mod generated_compressed_qishuo_correction_data;
mod consts;

use alloc::vec::Vec;

use crate::sxtwl::coefficients::DT_AT;
use crate::sxtwl::coefficients::XL1;
use crate::sxtwl::coefficients::{NUT_B, QI_KB, SHUO_KB, XL0};

use crate::sxtwl::generated_compressed_qishuo_correction_data::{get_qi_value, get_shuo_value};

use libm::{cos, floor, sin};

use core::f64::consts::PI;

pub const PI_2: f64 = PI * 2.0;
pub const ONE_THIRD: f64 = 1.0 / 3.0;
const SECOND_PER_DAY: f64 = 86400.0;
const SECOND_PER_RAD: f64 = 180.0 * 3600.0 / PI;

/// 寿星天文历工具
pub struct Sxtwl {}

impl Sxtwl {
    pub fn nutation_lon2(t: f64) -> f64 {
        let mut a: f64 = -1.742 * t;
        let t2: f64 = t * t;
        let mut dl: f64 = 0.0;
        let mut i: usize = 0;
        let size: usize = NUT_B.len();
        while i < size {
            dl += (NUT_B[i + 3] + a) * sin(NUT_B[i] + NUT_B[i + 1] * t + NUT_B[i + 2] * t2);
            a = 0.0;
            i += 5;
        }
        dl / 100.0 / SECOND_PER_RAD
    }

    pub fn elon(pt: f64, n: isize) -> f64 {
        let t: f64 = pt / 10.0;
        let mut v: f64 = 0.0;
        let mut tn: f64 = 1.0;
        let mut m: usize;
        let pn: usize = 1;
        let m0: f64 = XL0[pn + 1] - XL0[pn];
        for i in 0..6 {
            let n1: usize = XL0[pn + i] as usize;
            let n2: usize = XL0[pn + 1 + i] as usize;
            let n0: f64 = (n2 - n1) as f64;
            if n0 == 0.0 {
                continue;
            }
            if n < 0 {
                m = n2;
            } else {
                m = ((3.0 * (n as f64) * n0 / m0 + 0.5) as usize) + n1;
                if i != 0 {
                    m += 3;
                }
                if m > n2 {
                    m = n2;
                }
            }
            let mut c: f64 = 0.0;
            let mut j: usize = n1;
            while j < m {
                c += XL0[j] * (XL0[j + 1] + t * XL0[j + 2]).cos();
                j += 3;
            }
            v += c * tn;
            tn *= t;
        }
        v /= XL0[0];
        let t2: f64 = t * t;
        v += (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t2 * t) / SECOND_PER_RAD;
        v
    }

    pub fn mlon(t: f64, pn: isize) -> f64 {
        let obl: isize = XL1[0].len() as isize;
        let mut tn: f64 = 1.0;
        let mut v: f64 = 0.0;
        let mut t2: f64 = t * t;
        let mut t3: f64 = t2 * t;
        let mut t4: f64 = t3 * t;
        let t5: f64 = t4 * t;
        let tx: f64 = t - 10.0;
        v += (3.81034409 + 8399.684730072 * t - 3.319e-05 * t2 + 3.11e-08 * t3 - 2.033e-10 * t4)
            * SECOND_PER_RAD;
        v += 5028.792262 * t + 1.1124406 * t2 + 0.00007699 * t3
            - 0.000023479 * t4
            - 0.0000000178 * t5;
        if tx > 0.0 {
            v += -0.866 + 1.43 * tx + 0.054 * tx * tx;
        }
        t2 /= 1e4;
        t3 /= 1e8;
        t4 /= 1e8;

        let mut n: isize = pn * 6;
        if n < 0 {
            n = obl;
        }
        for i in 0..XL1.len() {
            let f: Vec<f64> = XL1[i].clone();
            let l: usize = f.len();
            let mut m: usize = (((n * (l as isize) / obl) as f64) + 0.5) as usize;
            if i > 0 {
                m += 6;
            }
            if m >= l {
                m = l;
            }
            let mut c: f64 = 0.0;
            let mut j: usize = 0;
            while j < m {
                c += f[j]
                    * cos(f[j + 1] + t * f[j + 2] + t2 * f[j + 3] + t3 * f[j + 4] + t4 * f[j + 5]);
                j += 6;
            }
            v += c * tn;
            tn *= t;
        }
        v /= SECOND_PER_RAD;
        v
    }

    pub fn gxc_sun_lon(t: f64) -> f64 {
        let t2: f64 = t * t;
        let v: f64 = -0.043126 + 628.301955 * t - 0.000002732 * t2;
        let e: f64 = 0.016708634 - 0.000042037 * t - 0.0000001267 * t2;
        -20.49552 * (1.0 + e * cos(v)) / SECOND_PER_RAD
    }

    pub fn ev(t: f64) -> f64 {
        let f: f64 = 628.307585 * t;
        628.332
            + 21.0 * sin(1.527 + f)
            + 0.44 * sin(1.48 + f * 2.0)
            + 0.129 * sin(5.82 + f) * t
            + 0.00055 * sin(4.21 + f) * t * t
    }

    pub fn sa_lon(t: f64, n: isize) -> f64 {
        Self::elon(t, n) + Self::nutation_lon2(t) + Self::gxc_sun_lon(t) + PI
    }

    pub fn dt_ext(y: f64, jsd: f64) -> f64 {
        let dy: f64 = (y - 1820.0) / 100.0;
        -20.0 + jsd * dy * dy
    }

    pub fn dt_calc(y: f64) -> f64 {
        let size: usize = DT_AT.len();
        let y0: f64 = DT_AT[size - 2];
        let t0: f64 = DT_AT[size - 1];
        if y >= y0 {
            let jsd: f64 = 31.0;
            if y > y0 + 100.0 {
                return Self::dt_ext(y, jsd);
            }
            return Self::dt_ext(y, jsd) - (Self::dt_ext(y0, jsd) - t0) * (y0 + 100.0 - y) / 100.0;
        }
        let mut i: usize = 0;
        while i < size {
            if y < DT_AT[i + 5] {
                break;
            }
            i += 5;
        }
        let t1: f64 = (y - DT_AT[i]) / (DT_AT[i + 5] - DT_AT[i]) * 10.0;
        let t2: f64 = t1 * t1;
        let t3: f64 = t2 * t1;
        DT_AT[i + 1] + DT_AT[i + 2] * t1 + DT_AT[i + 3] * t2 + DT_AT[i + 4] * t3
    }

    pub fn dtt(t: f64) -> f64 {
        Self::dt_calc(t / 365.2425 + 2000.0) / SECOND_PER_DAY
    }

    pub fn mv(t: f64) -> f64 {
        let mut v: f64 = 8399.71 - 914.0 * sin(0.7848 + 8328.691425 * t + 0.0001523 * t * t);
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

    pub fn sa_lon_t(w: f64) -> f64 {
        let mut v: f64 = 628.3319653318;
        let mut t = (w - 1.75347 - PI) / v;
        v = Self::ev(t);
        t += (w - Self::sa_lon(t, 10)) / v;
        v = Self::ev(t);
        t += (w - Self::sa_lon(t, -1)) / v;
        t
    }

    pub fn m_sa_lon(t: f64, mn: isize, sn: isize) -> f64 {
        Self::mlon(t, mn) + (-3.4E-6) - (Self::elon(t, sn) + Self::gxc_sun_lon(t) + PI)
    }

    pub fn m_sa_lon_t(w: f64) -> f64 {
        let mut v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        t += (w - Self::m_sa_lon(t, 3, 3)) / v;
        v = Self::mv(t) - Self::ev(t);
        t += (w - Self::m_sa_lon(t, 20, 10)) / v;
        t += (w - Self::m_sa_lon(t, -1, 60)) / v;
        t
    }

    pub fn sa_lon_t2(w: f64) -> f64 {
        let v: f64 = 628.3319653318;
        let mut t: f64 = (w - 1.75347 - PI) / v;
        t -= (0.000005297 * t * t
            + 0.0334166 * cos(4.669257 + 628.307585 * t)
            + 0.0002061 * cos(2.67823 + 628.307585 * t) * t)
            / v;
        t += (w - Self::elon(t, 8) - PI
            + (20.5 + 17.2 * sin(2.1824 - 33.75705 * t)) / SECOND_PER_RAD)
            / v;
        t
    }

    pub fn m_sa_lon_t2(w: f64) -> f64 {
        let mut v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        let mut t2: f64 = t * t;
        t -= (-0.00003309 * t2
            + 0.10976 * cos(0.784758 + 8328.6914246 * t + 0.000152292 * t2)
            + 0.02224 * cos(0.18740 + 7214.0628654 * t - 0.00021848 * t2)
            - 0.03342 * cos(4.669257 + 628.307585 * t))
            / v;
        t2 = t * t;
        let l: f64 = Self::mlon(t, 20)
            - (4.8950632
                + 628.3319653318 * t
                + 0.000005297 * t2
                + 0.0334166 * cos(4.669257 + 628.307585 * t)
                + 0.0002061 * cos(2.67823 + 628.307585 * t) * t
                + 0.000349 * cos(4.6261 + 1256.61517 * t)
                - 20.5 / SECOND_PER_RAD);
        v = 7771.38
            - 914.0 * sin(0.7848 + 8328.691425 * t + 0.0001523 * t2)
            - 179.0 * sin(2.543 + 15542.7543 * t)
            - 160.0 * sin(0.1874 + 7214.0629 * t);
        t += (w - l) / v;
        t
    }

    pub fn qi_high(w: f64) -> f64 {
        let mut t: f64 = Self::sa_lon_t2(w) * 36525.0;
        t = t - Self::dtt(t) + ONE_THIRD;
        let v: f64 = ((t + 0.5) % 1.0) * SECOND_PER_DAY;
        if v < 1200.0 || v > (SECOND_PER_DAY - 1200.0) {
            t = Self::sa_lon_t(w) * 36525.0 - Self::dtt(t) + ONE_THIRD;
        }
        t
    }

    pub fn shuo_high(w: f64) -> f64 {
        let mut t: f64 = Self::m_sa_lon_t2(w) * 36525.0;
        t = t - Self::dtt(t) + ONE_THIRD;
        let v: f64 = ((t + 0.5) % 1.0) * SECOND_PER_DAY;
        if v < 1800.0 || v > (SECOND_PER_DAY - 1800.0) {
            t = Self::m_sa_lon_t(w) * 36525.0 - Self::dtt(t) + ONE_THIRD;
        }
        t
    }

    pub fn qi_low(w: f64) -> f64 {
        let v: f64 = 628.3319653318;
        let mut t: f64 = (w - 4.895062166) / v;
        t -= (53.0 * t * t
            + 334116.0 * cos(4.67 + 628.307585 * t)
            + 2061.0 * cos(2.678 + 628.3076 * t) * t)
            / v
            / 10000000.0;
        let n: f64 = 48950621.66
            + 6283319653.318 * t
            + 53.0 * t * t
            + 334166.0 * cos(4.669257 + 628.307585 * t)
            + 3489.0 * cos(4.6261 + 1256.61517 * t)
            + 2060.6 * cos(2.67823 + 628.307585 * t) * t
            - 994.0
            - 834.0 * sin(2.1824 - 33.75705 * t);
        t -= (n / 10000000.0 - w) / 628.332
            + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
        t * 36525.0 + ONE_THIRD
    }

    pub fn shuo_low(w: f64) -> f64 {
        let v: f64 = 7771.37714500204;
        let mut t: f64 = (w + 1.08472) / v;
        t -= (-0.0000331 * t * t
            + 0.10976 * cos(0.785 + 8328.6914 * t)
            + 0.02224 * cos(0.187 + 7214.0629 * t)
            - 0.03342 * cos(4.669 + 628.3076 * t))
            / v
            + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
        t * 36525.0 + ONE_THIRD
    }

    pub fn calc_shuo(pjd: f64) -> f64 {
        let size: usize = SHUO_KB.len();
        let mut d: f64 = 0.0;
        let pc: f64 = 14.0;
        let mut i: usize = 0;
        let jd: f64 = pjd + 2451545.0;
        let f1: f64 = SHUO_KB[0] - pc;
        let f2: f64 = SHUO_KB[size - 1] - pc;
        let f3: f64 = 2436935.0;
        if jd < f1 || jd >= f3 {
            d = floor(Self::shuo_high(floor((jd + pc - 2451551.0) / 29.5306) * PI_2) + 0.5);
        } else if jd >= f1 && jd < f2 {
            while i < size {
                if jd + pc < SHUO_KB[i + 2] {
                    break;
                }
                i += 2;
            }
            d = SHUO_KB[i] + SHUO_KB[i + 1] * floor((jd + pc - SHUO_KB[i]) / SHUO_KB[i + 1]);
            d = floor(d + 0.5);
            if d == 1683460.0 {
                d += 1.0;
            }
            d -= 2451545.0;
        } else if jd >= f2 {
            d = floor(Self::shuo_low(floor((jd + pc - 2451551.0) / 29.5306) * PI_2) + 0.5);
            let from: usize = ((jd - f2) / 29.5306) as usize;

            let n = get_shuo_value(from);
            if n == 1 {
                d += 1.0;
            } else if n == 2 {
                d -= 1.0;
            }
        }
        d
    }

    pub fn calc_qi(pjd: f64) -> f64 {
        let size: usize = QI_KB.len();
        let mut d: f64 = 0.0;
        let pc: f64 = 7.0;
        let mut i: usize = 0;
        let jd: f64 = pjd + 2451545.0;
        let f1: f64 = QI_KB[0] - pc;
        let f2: f64 = QI_KB[size - 1] - pc;
        let f3: f64 = 2436935.0;
        if jd < f1 || jd >= f3 {
            d = floor(
                Self::qi_high(floor((jd + pc - 2451259.0) / 365.2422 * 24.0) * PI / 12.0) + 0.5,
            );
        } else if jd >= f1 && jd < f2 {
            while i < size {
                if jd + pc < QI_KB[i + 2] {
                    break;
                }
                i += 2;
            }
            d = QI_KB[i] + QI_KB[i + 1] * floor((jd + pc - QI_KB[i]) / QI_KB[i + 1]);
            d = floor(d + 0.5);
            if d == 1683460.0 {
                d += 1.0;
            }
            d -= 2451545.0;
        } else if jd >= f2 {
            d = floor(
                Self::qi_low(floor((jd + pc - 2451259.0) / 365.2422 * 24.0) * PI / 12.0) + 0.5,
            );
            let from: usize = ((jd - f2) / 365.2422 * 24.0) as usize;
            let n = get_qi_value(from);
            if n == 1 {
                d += 1.0;
            } else if n == 2 {
                d -= 1.0;
            }
        }
        d
    }

    pub fn qi_accurate(w: f64) -> f64 {
        let t: f64 = Self::sa_lon_t(w) * 36525.0;
        t - Self::dtt(t) + ONE_THIRD
    }

    pub fn qi_accurate2(jd: f64) -> f64 {
        let d: f64 = PI / 12.0;
        let w: f64 = floor((jd + 293.0) / 365.2422 * 24.0) * d;
        let a: f64 = Self::qi_accurate(w);
        if a - jd > 5.0 {
            return Self::qi_accurate(w - d);
        }
        if a - jd < -5.0 {
            return Self::qi_accurate(w + d);
        }
        a
    }
}
