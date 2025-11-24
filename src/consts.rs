//! 常量定义

/// J2000儒略日基准点（2000年1月1日12:00）
pub const J2000: f64 = 2451545.0;

/// 农历月平均天数
pub const LUNAR_MONTH_DAYS: f64 = 29.5306;

/// 太阳年平均天数
pub const SOLAR_YEAR_DAYS: f64 = 365.2422;

/// 太阳年平均二十四节气数
pub const JIEQI_PER_YEAR: f64 = 24.0;

/// 太阳年平均二十四节气间隔
pub const JIEQI_INTERVAL: f64 = SOLAR_YEAR_DAYS / JIEQI_PER_YEAR;

/// 一天的秒数
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// 儒略世纪天数
pub const JULIAN_CENTURY_DAYS: f64 = 36525.0;

use core::f64::consts::PI;

// pub const PI: f64 = core::f64::consts::PI;
// pub const PI2: f64 = PI * 2.0; //TAU
// pub const PI_2: f64 = PI / 2.0;//FRAC_PI_2

/// 地球赤道半径(千米)
pub const CS_R_EAR: f64 = 6378.1366;

/// 平均半径
pub const CS_R_EAR_A: f64 = 0.99834 * CS_R_EAR;

/// 地球极赤半径比
pub const CS_BA: f64 = 0.99664719;

/// 地球极赤半径比的平方
pub const CS_BA2: f64 = CS_BA * CS_BA;

/// 天文单位长度(千米)
pub const CS_AU: f64 = 1.49597870691e8;

/// sin(太阳视差)
pub const CS_SINP: f64 = CS_R_EAR / CS_AU;

/// 太阳视差
// pub const CS_PI: f64 = CS_SINP.asin(); // 太阳视差 Rust暂不支持常量计算asin
pub const CS_PI: f64 = 4.263_520_979_591_08e-5;

/// 光速(千米/秒)
pub const CS_GS: f64 = 299792.458; // 光速(千米/秒)

/// 每天文单位的光行时间(儒略世纪)
pub const CS_AGX: f64 = CS_AU / CS_GS / SECONDS_PER_DAY / JULIAN_CENTURY_DAYS;

/// 每弧度的角秒数
pub const RAD: f64 = 180.0 * 3600.0 / PI;

/// 每弧度的度数
pub const RADD: f64 = 180.0 / PI;

/// 月亮与地球的半径比(用于半影计算)
pub const CS_K: f64 = 0.2725076;

/// 月亮与地球的半径比(用于本影计算)
pub const CS_K2: f64 = 0.2722810;

/// 太阳与地球的半径比(对应959.64)
pub const CS_K0: f64 = 109.1222;

/// 用于月亮视半径计算
pub const CS_S_MOON: f64 = CS_K * CS_R_EAR * 1.0000036 * RAD;

/// 用于月亮视半径计算
pub const CS_S_MOON2: f64 = CS_K2 * CS_R_EAR * 1.0000036 * RAD;

/// 用于太阳视半径计算
pub const CS_S_SUN: f64 = 959.64;

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
