//! 天文计算常量定义
//!
//! 包含天文计算中使用的物理常数、天文常数和转换因子。
//! 所有常量都基于国际天文学联合会(IAU)推荐值。

use core::f64::consts::PI;

// =============================================================================
// 时间相关常量
// =============================================================================

/// J2000儒略日基准点（2000年1月1日12:00）
pub const J2000: f64 = 2451545.0;

/// 农历朔望月平均长度（天）
pub const LUNAR_MONTH_DAYS: f64 = 29.5306;

/// 回归年长度（天） - 基于春分点到春分点
pub const TROPICAL_YEAR_DAYS: f64 = 365.2422;

/// 太阳年平均二十四节气数
pub const JIEQI_PER_YEAR: f64 = 24.0;

/// 太阳年平均二十四节气间隔
pub const JIEQI_INTERVAL: f64 = TROPICAL_YEAR_DAYS / JIEQI_PER_YEAR;

/// 一天的秒数
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// 儒略世纪天数
pub const JULIAN_CENTURY_DAYS: f64 = 36525.0;

// =============================================================================
// 地球物理常数
// =============================================================================

/// 地球赤道半径(千米)
pub const EARTH_EQUATORIAL_RADIUS_KM: f64 = 6378.1366;

/// 地球平均半径（千米）
pub const EARTH_MEAN_RADIUS_KM: f64 = 0.99834 * EARTH_EQUATORIAL_RADIUS_KM;

/// 地球极赤半径比
pub const EARTH_POLAR_FLATTENING: f64 = 0.99664719;

/// 地球极赤半径比的平方
pub const EARTH_POLAR_FLATTENING_SQUARED: f64 = EARTH_POLAR_FLATTENING * EARTH_POLAR_FLATTENING;

// =============================================================================
// 天文单位常数
// =============================================================================

/// 天文单位长度（千米） - IAU 2012 定义
pub const ASTRONOMICAL_UNIT_KM: f64 = 1.49597870691e8;

/// sin(太阳视差)
pub const SOLAR_PARALLAX_RADIANS_SIN: f64 = EARTH_EQUATORIAL_RADIUS_KM / ASTRONOMICAL_UNIT_KM;

/// 太阳视差（弧度）
// pub const CS_PI: f64 = CS_SINP.asin(); // 太阳视差 Rust暂不支持常量计算asin
pub const SOLAR_PARALLAX_RADIANS: f64 = 4.263_520_979_591_08e-5;

/// 光速（千米/秒） - 定义值
pub const SPEED_OF_LIGHT_KM_PER_SEC: f64 = 299792.458; // 光速(千米/秒)

/// 每天文单位的光行时间（儒略世纪）
pub const LIGHT_TIME_PER_AU_CENTURIES: f64 = ASTRONOMICAL_UNIT_KM / SPEED_OF_LIGHT_KM_PER_SEC / SECONDS_PER_DAY / JULIAN_CENTURY_DAYS;

// =============================================================================
// 角度转换常数
// =============================================================================

/// 每弧度的角秒数
pub const RAD: f64 = 180.0 * 3600.0 / PI;

/// 每弧度的度数
pub const RADD: f64 = 180.0 / PI;

// =============================================================================
// 月球相关常数
// =============================================================================

/// 月球与地球半径比（用于半影计算）
pub const MOON_EARTH_RADIUS_RATIO_PENUMBRA: f64 = 0.2725076;

/// 月球与地球半径比（用于本影计算）
pub const MOON_EARTH_RADIUS_RATIO_UMBRA: f64 = 0.2722810;

/// 太阳与地球半径比
pub const SUN_EARTH_RADIUS_RATIO: f64 = 109.1222;

/// 月球视半径计算常数（角秒）
pub const LUNAR_APPARENT_RADIUS: f64 = MOON_EARTH_RADIUS_RATIO_PENUMBRA * EARTH_EQUATORIAL_RADIUS_KM * 1.0000036 * RAD;

/// 月球视半径计算常数（本影，角秒）
pub const LUNAR_APPARENT_RADIUS_UMBRA: f64 = MOON_EARTH_RADIUS_RATIO_UMBRA * EARTH_EQUATORIAL_RADIUS_KM * 1.0000036 * RAD;

/// 太阳视半径（角秒） - 平均値
pub const SOLAR_APPARENT_RADIUS_ARCSEC: f64 = 959.64;

// =============================================================================
// 格式化相关常量
// =============================================================================

/// 节气名称
pub const JIEQI_NAMES: [&str; 24] = [
    "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至", "小暑", "大暑",
    "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "冬至", "小寒", "大寒",
];

/// 月份名称（农历）
pub const LUNAR_MONTH_NAMES: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "腊月",
];

/// 星座名称
pub const CONSTELLATION_NAMES: [&str; 12] = [
    "水瓶座",
    "双鱼座",
    "白羊座",
    "金牛座",
    "双子座",
    "巨蟹座",
    "狮子座",
    "处女座",
    "天秤座",
    "天蝎座",
    "射手座",
    "摩羯座",
];

/// 农历新年月索引映射
pub const YUE_INDEX: [i32; 12] = [11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
