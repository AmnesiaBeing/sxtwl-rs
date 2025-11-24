// src/astronomy.rs
//! 天文计算模块
//! 基于寿星天文历算法实现

use core::f64::consts::{PI, TAU as PI2};
use crate::consts::{J2000, JULIAN_CENTURY_DAYS};
use libm::{sin, cos, atan2, asin, sqrt};

mod coefficients;
mod delta_t;
mod math_utils;
mod nutation;
mod planetary_orbits;
mod precession;

// 从子模块重新导出常用函数
pub use self::planetary_orbits::xl::{E_Lon, M_Lon, E_v, M_v, moonIll, moonRad, E_Lon_t, M_Lon_t};
pub use self::nutation::{nutation, nutation_simple, true_obliquity, mean_obliquity};
pub use self::precession::{CDllr_J2D, CDllr_J2D_Fast};
pub use self::delta_t::{dt_T, dt_dT};

// 保留原有公共导出
pub use coefficients::*;
pub use delta_t::*;
pub use math_utils::*;
pub use nutation::*;
pub use planetary_orbits::*;
pub use precession::*;

/// 天文计算主模块
pub struct Astronomy;

impl Astronomy {
    /// 计算儒略世纪数 (相对于J2000.0)
    pub fn julian_century(jd: f64) -> f64 {
        (jd - J2000) / JULIAN_CENTURY_DAYS
    }
    
    /// 计算太阳平黄经（高精度）
    pub fn mean_solar_lon(jd: f64) -> f64 {
        // 使用高精度的行星轨道计算
        planetary_orbits::xl::E_Lon_t(jd, 3) // 使用中等精度
    }
    
    /// 计算月亮平黄经（高精度）
    pub fn mean_lunar_lon(jd: f64) -> f64 {
        // 使用高精度的行星轨道计算
        planetary_orbits::xl::M_Lon_t(jd, 3) // 使用中等精度
    }
    
    /// 计算太阳黄经（包含光行差修正，高精度）
    pub fn solar_lon(jd: f64) -> f64 {
        // 使用高精度的太阳黄经计算
        let t = Self::julian_century(jd);
        
        // 获取高精度太阳黄经
        let mut lon = planetary_orbits::xl::E_Lon(jd);
        
        // 添加章动修正
        let (nut_lon, _) = nutation(t);
        lon += nut_lon;
        
        // 归一化到0-2π范围
        lon % PI2
    }
    
    /// 计算节气太阳黄经
    pub fn solar_term_lon(term_index: i32) -> f64 {
        // 24节气，每个节气间隔15度
        let degree = term_index as f64 * 15.0;
        degree * PI / 180.0 // 转换为弧度
    }
    
    /// 计算春分点儒略日（高精度）
    pub fn spring_equinox_jd(year: i32) -> f64 {
        // 计算春分点的精确位置
        // 从近似值开始迭代
        let mut jd0 = 2451623.80984 + 365242.37404 * (year - 2000) as f64 / 100.0;
        let mut iter = 0;
        
        // 迭代搜索春分点（太阳黄经为0）
        while iter < 5 {
            let lon = Self::solar_lon(jd0);
            let lon_deg = lon * 180.0 / PI;
            
            // 如果黄经接近0度，认为找到了春分点
            if lon_deg.abs() < 0.001 {
                break;
            }
            
            // 计算太阳速度
            let v = planetary_orbits::xl::E_v(jd0) * 180.0 / PI * 86400.0; // 度/天
            
            // 修正儒略日
            let correction = -lon_deg / v; // 天
            jd0 += correction;
            
            iter += 1;
        }
        
        // 调整到北京时间（UTC+8）
        jd0 + 8.0 / 24.0
    }
    
    /// 计算月球视黄经（高精度）
    pub fn lunar_lon(jd: f64) -> f64 {
        // 使用高精度的月球黄经计算
        let t = Self::julian_century(jd);
        
        // 获取高精度月球黄经
        let mut lon = planetary_orbits::xl::M_Lon(jd);
        
        // 添加章动修正
        let (nut_lon, _) = nutation(t);
        lon += nut_lon;
        
        // 归一化到0-2π范围
        lon % PI2
    }
    
    /// 计算月球照亮比例（高精度）
    pub fn moon_illumination(jd: f64) -> f64 {
        // 使用高精度的月球照亮计算
        planetary_orbits::xl::moonIll(jd)
    }
    
    /// 计算月球视半径（高精度）
    pub fn moon_radius(jd: f64) -> f64 {
        // 使用高精度的月球半径计算，并转换为弧度
        planetary_orbits::xl::moonRad(jd) * PI / (180.0 * 3600.0)
    }
    
    /// 计算月相
    pub fn moon_phase(jd: f64) -> f64 {
        // 计算太阳和月球的黄经差
        let solar_lon = Self::solar_lon(jd);
        let lunar_lon = Self::lunar_lon(jd);
        
        // 计算相位角（弧度）
        let phase_angle = lunar_lon - solar_lon;
        let phase_angle = phase_angle % PI2;
        
        // 转换为0-1的月相值（0=新月，0.25=上弦，0.5=满月，0.75=下弦）
        phase_angle / PI2
    }
    
    /// 计算日月合朔时间（新月）
    pub fn new_moon_jd(near_jd: f64) -> f64 {
        // 从近似值开始迭代搜索日月合朔
        let mut jd0 = near_jd;
        let mut iter = 0;
        
        while iter < 10 {
            let solar_lon = Self::solar_lon(jd0);
            let lunar_lon = Self::lunar_lon(jd0);
            
            // 计算黄经差
            let mut diff = lunar_lon - solar_lon;
            if diff > PI {
                diff -= PI2;
            } else if diff < -PI {
                diff += PI2;
            }
            
            // 如果黄经差足够小，认为找到了合朔
            if diff.abs() < 0.0001 { // 约0.0057度
                break;
            }
            
            // 计算月球相对于太阳的速度
            let solar_v = planetary_orbits::xl::E_v(jd0);
            let lunar_v = planetary_orbits::xl::M_v(jd0);
            let relative_v = lunar_v - solar_v;
            
            // 修正儒略日
            let correction = -diff / relative_v;
            jd0 += correction;
            
            iter += 1;
        }
        
        jd0
    }
}