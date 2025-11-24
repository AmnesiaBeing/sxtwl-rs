//! 行星轨道计算模块
//!
//! 实现太阳、月球和其他行星的高精度坐标计算，包括：
//! - 行星位置计算
//! - 月球运动模型
//! - 光行差修正
//! - 恒星时计算
//! - 特殊事件计算（近点、交点等）

use crate::{
    astronomy::{
        Vector2, Vector3, XL0, XL0_XZB, XL1, calculate_longitude_nutation_medium, delta_t_from_j2000, calculate_obliquity_p03, llr_conv, normalize_rad, pow2
    },
    consts::{EARTH_EQUATORIAL_RADIUS_KM, LUNAR_APPARENT_RADIUS, JULIAN_CENTURY_DAYS, LUNAR_MONTH_DAYS, RAD, SECONDS_PER_DAY},
};
use core::f64::consts::{PI, TAU as PI2};
use libm::{acos, asin, atan2, cos, floor, round, sin};

// =============================================================================
// 常量定义
// =============================================================================

/// 光行差常数
// const GXC_SUN_LAT: f64 = 0.0; // 太阳黄纬光行差
const GXC_MOON_LON: f64 = -3.4E-6; // 月球经度光行差

/// 月球轨道周期常数
const LUNAR_ANOMALISTIC_MONTH: f64 = 27.55454988; // 近点月长度（天）
const LUNAR_DRACONIC_MONTH: f64 = 27.21222082; // 交点月长度（天）

/// 地球轨道周期常数
const EARTH_ANOMALISTIC_YEAR: f64 = 365.25963586; // 近点年长度（天）

// =============================================================================
// 行星坐标计算
// =============================================================================

/// 计算行星坐标的基础函数
///
/// # 参数
/// - `planet_index`: 行星索引（0=地球, 1=水星, ..., 9=太阳）
/// - `coordinate_index`: 坐标分量索引（0=黄经, 1=黄纬, 2=距离）
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
/// - `term_count`: 计算项数，负数表示使用所有项
///
/// # 返回值
/// 坐标分量值
pub fn calculate_planet_coordinate(
    planet_index: usize,
    coordinate_index: usize,
    julian_centuries: f64,
    term_count: i32,
) -> f64 {
    let t = julian_centuries / 10.0; // 转换为儒略千年数
    let coefficients = XL0[planet_index];
    let base_index = coordinate_index * 6 + 1;

    let total_terms = coefficients[base_index + 1] - coefficients[base_index];
    let mut result = 0.0;
    let mut t_power = 1.0;

    for i in 0..6 {
        let start_index = coefficients[base_index + i] as usize;
        let end_index = coefficients[base_index + i + 1] as usize;

        if start_index == end_index {
            t_power *= t;
            continue;
        }

        let terms_to_use = if term_count < 0 {
            end_index
        } else {
            let calculated =
                round(3.0 * term_count as f64 * (end_index - start_index) as f64 / total_terms)
                    as usize
                    + start_index;
            if i > 0 { calculated + 3 } else { calculated }.min(end_index)
        };

        let mut sum = 0.0;
        let mut j = start_index;
        while j < terms_to_use {
            let amplitude = coefficients[j];
            let phase = coefficients[j + 1];
            let frequency = coefficients[j + 2];
            sum += amplitude * cos(phase + t * frequency);
            j += 3;
        }

        result += sum * t_power;
        t_power *= t;
    }

    result /= coefficients[0];
    apply_planet_corrections(planet_index, coordinate_index, result, t)
}

/// 应用行星特定修正项
fn apply_planet_corrections(
    planet_index: usize,
    coordinate_index: usize,
    base_value: f64,
    julian_millennia: f64,
) -> f64 {
    let t = julian_millennia;
    let t2 = t * t;
    let t3 = t2 * t;

    if planet_index == 0 {
        // 地球修正
        match coordinate_index {
            0 => base_value + (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t3) / RAD,
            1 => base_value + (0.0000 + 0.0004 * t + 0.0004 * t2 - 0.0026 * t3) / RAD,
            2 => base_value + (-0.0020 + 0.0044 * t + 0.0213 * t2 - 0.0250 * t3) / 1_000_000.0,
            _ => base_value,
        }
    } else {
        // 其他行星修正
        let correction = XL0_XZB[(planet_index - 1) * 3 + coordinate_index];
        match coordinate_index {
            0 => base_value - 3.0 * t / RAD,
            2 => base_value + correction / 1_000_000.0,
            _ => base_value + correction / RAD,
        }
    }
}

/// 计算行星的完整坐标
///
/// # 参数
/// - `planet_index`: 行星索引
/// - `julian_centuries`: 儒略世纪数
/// - `longitude_terms`: 黄经计算项数
/// - `latitude_terms`: 黄纬计算项数  
/// - `distance_terms`: 距离计算项数
pub fn calculate_planet_position(
    planet_index: usize,
    julian_centuries: f64,
    longitude_terms: i32,
    latitude_terms: i32,
    distance_terms: i32,
) -> Vector3 {
    if planet_index == 9 {
        // 太阳位置（地心系中为原点）
        return Vector3::new(0.0, 0.0, 0.0);
    }

    Vector3::new(
        calculate_planet_coordinate(planet_index, 0, julian_centuries, longitude_terms),
        calculate_planet_coordinate(planet_index, 1, julian_centuries, latitude_terms),
        calculate_planet_coordinate(planet_index, 2, julian_centuries, distance_terms),
    )
}

/// 计算地球坐标
pub fn calculate_earth_position(
    julian_centuries: f64,
    longitude_terms: i32,
    latitude_terms: i32,
    distance_terms: i32,
) -> Vector3 {
    calculate_planet_position(
        0,
        julian_centuries,
        longitude_terms,
        latitude_terms,
        distance_terms,
    )
}

// =============================================================================
// 月球坐标计算
// =============================================================================

/// 计算月球坐标分量
pub fn calculate_lunar_coordinate(
    coordinate_index: usize,
    julian_centuries: f64,
    term_count: i32,
) -> f64 {
    let coefficients = XL1[coordinate_index];
    let mut result = 0.0;
    let mut t_power = 1.0;

    // 黄经的特殊处理
    if coordinate_index == 0 {
        result += calculate_lunar_mean_longitude(julian_centuries);
    }

    let t = julian_centuries;
    let t2 = t * t / 10_000.0;
    let t3 = t * t2 / 10_000.0;
    let t4 = t * t3 / 10_000.0;

    let adjusted_term_count = if term_count < 0 { 3 } else { term_count * 6 };

    for (i, coefficient_group) in coefficients.iter().enumerate().take(3) {
        let group_size = coefficient_group.len();
        let terms_to_use = round(adjusted_term_count as f64 * group_size as f64 / 3.0) as usize;
        let terms_to_use = if i > 0 {
            terms_to_use + 6
        } else {
            terms_to_use
        }
        .min(group_size);

        let mut sum = 0.0;
        let mut j = 0;
        while j < terms_to_use {
            sum += coefficient_group[j]
                * cos(coefficient_group[j + 1]
                    + t * coefficient_group[j + 2]
                    + t2 * coefficient_group[j + 3]
                    + t3 * coefficient_group[j + 4]
                    + t4 * coefficient_group[j + 5]);
            j += 6;
        }

        result += sum * t_power;
        t_power *= t;
    }

    if coordinate_index != 2 {
        result / RAD
    } else {
        result
    }
}

/// 计算月球平黄经（包含岁差修正）
fn calculate_lunar_mean_longitude(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;

    let mut mean_longitude =
        (3.81034409 + 8399.684730072 * t - 3.319e-05 * t2 + 3.11e-08 * t3 - 2.033e-10 * t4) * RAD;

    // 岁差修正
    mean_longitude +=
        5028.792262 * t + 1.1124406 * t2 + 0.00007699 * t3 - 0.000023479 * t4 - 0.0000000178 * t5;

    // 长期项修正（公元3000-5000年）
    let tx = t - 10.0;
    if tx > 0.0 {
        mean_longitude += -0.866 + 1.43 * tx + 0.054 * tx * tx;
    }

    mean_longitude
}

/// 计算月球完整坐标
pub fn calculate_lunar_position(
    julian_centuries: f64,
    longitude_terms: i32,
    latitude_terms: i32,
    distance_terms: i32,
) -> Vector3 {
    Vector3::new(
        calculate_lunar_coordinate(0, julian_centuries, longitude_terms),
        calculate_lunar_coordinate(1, julian_centuries, latitude_terms),
        calculate_lunar_coordinate(2, julian_centuries, distance_terms),
    )
}

// =============================================================================
// 光行差计算
// =============================================================================

/// 计算太阳光行差对黄经的影响
pub fn calculate_solar_aberration_longitude(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let mean_anomaly = -0.043126 + 628.301955 * t - 0.000002732 * t * t;
    let eccentricity = 0.016708634 - 0.000042037 * t - 0.0000001267 * t * t;

    -20.49552 * (1.0 + eccentricity * cos(mean_anomaly)) / RAD
}

/// 计算月球光行差对黄纬的影响
pub fn calculate_lunar_aberration_latitude(julian_centuries: f64) -> f64 {
    0.063
        * sin(0.057 + 8433.4662 * julian_centuries + 0.000064 * julian_centuries * julian_centuries)
        / RAD
}

// =============================================================================
// 时间相关计算
// =============================================================================

/// 计算朔日编号，jd应在朔日附近，允许误差数天
pub fn calculate_new_moon_number(julian_day: f64) -> i32 {
    floor((julian_day + 8.0) / LUNAR_MONTH_DAYS) as i32
}

/// 计算格林尼治平恒星时
pub fn calculate_greenwich_mean_sidereal_time(julian_day: f64, delta_t: f64) -> f64 {
    let t_century = (julian_day + delta_t) / 36525.0;
    let t2 = t_century * t_century;
    let t3 = t2 * t_century;
    let t4 = t3 * t_century;

    PI2 * (0.7790572732640 + 1.002_737_811_911_354_6 * julian_day)
        + (0.014506 + 4612.15739966 * t_century + 1.39667721 * t2 - 0.00009344 * t3
            + 0.00001882 * t4)
            / RAD
}

/// 计算从 J2000 起算的力学时对应的平恒星时
pub fn calculate_sidereal_time_from_j2000(julian_days_since_j2000: f64) -> f64 {
    let delta_t = delta_t_from_j2000(julian_days_since_j2000);
    calculate_greenwich_mean_sidereal_time(julian_days_since_j2000 - delta_t, delta_t)
}

// =============================================================================
// 速度计算
// =============================================================================

pub fn calculate_earth_velocity(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let mean_anomaly = 628.307585 * t;

    628.332
        + 21.0 * sin(1.527 + mean_anomaly)
        + 0.44 * sin(1.48 + 2.0 * mean_anomaly)
        + 0.129 * sin(5.82 + mean_anomaly) * t
        + 0.00055 * sin(4.21 + mean_anomaly) * t * t
}

/// 计算月球轨道速度
pub fn calculate_lunar_velocity(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let mut velocity = 8399.71 - 914.0 * sin(0.7848 + 8328.691425 * t + 0.0001523 * t * t);

    // 主要摄动项
    velocity -= 179.0 * sin(2.543 + 15542.7543 * t)
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

    velocity
}

// =============================================================================
// 特殊事件计算
// =============================================================================

/// 计算月球近点或远点
pub fn calculate_lunar_apsis(
    julian_centuries: f64,
    is_perigee: bool, // true=近地点, false=远地点
) -> Vector2 {
    let period = LUNAR_ANOMALISTIC_MONTH / 36525.0;
    let phase_offset = if is_perigee { -10.3302 } else { 3.4471 } / 36525.0;

    let mut time = phase_offset + period * floor((julian_centuries - phase_offset) / period + 0.5);

    // 使用迭代法精化结果
    for &time_step in &[1.0, 0.5, 1200.0 / 86400.0] {
        let step = time_step / 36525.0;
        let (r1, r2, r3) = (
            calculate_lunar_coordinate(2, time - step, -1),
            calculate_lunar_coordinate(2, time, -1),
            calculate_lunar_coordinate(2, time + step, -1),
        );

        time += (r1 - r3) / (r1 + r3 - 2.0 * r2) * step / 2.0;
    }

    let final_distance = calculate_lunar_coordinate(2, time, -1);
    Vector2::new(time, final_distance)
}

/// 计算月球升交点或降交点
pub fn calculate_lunar_node(
    julian_centuries: f64,
    is_ascending: bool, // true=升交点, false=降交点
) -> f64 {
    let period = LUNAR_DRACONIC_MONTH / 36525.0;
    let phase_offset = if is_ascending { 21.0 } else { 35.0 } / 36525.0;

    let mut time = phase_offset + period * floor((julian_centuries - phase_offset) / period + 0.5);

    // 使用牛顿法求解
    for &time_step in &[0.5, 0.05] {
        let step = time_step / 36525.0;
        let (w1, w2) = (
            calculate_lunar_coordinate(1, time, 40),
            calculate_lunar_coordinate(1, time + step, 40),
        );

        let derivative = (w2 - w1) / step;
        time -= w1 / derivative;
    }

    // 最终精化
    let final_latitude = calculate_lunar_coordinate(1, time, -1);
    let step = 0.05 / 36525.0;
    let (w1, w2) = (
        calculate_lunar_coordinate(1, time, 40),
        calculate_lunar_coordinate(1, time + step, 40),
    );
    let derivative = (w2 - w1) / step;

    time - final_latitude / derivative
}

/// 计算太阳升降时间
///
/// # 参数
/// - `julian_day`: 儒略日
/// - `longitude`: 观测点地理经度（弧度，东经为正）
/// - `latitude`: 观测点地理纬度（弧度，北纬为正）  
/// - `time_type`: 时间类型（1=日出, -1=日落）
///
/// # 返回值
/// 太阳升降的儒略日时间，如果无升降返回0.0
pub fn calculate_sun_rise_set(
    julian_day: f64,
    longitude: f64,
    latitude: f64,
    time_type: f64,
) -> f64 {
    const SUN_ALTITUDE_CORRECTION: f64 = -50.0 * 60.0; // 太阳视半径和大气的综合修正（角秒）

    let mut current_jd = floor(julian_day + 0.5) - longitude / PI2;

    // 迭代两次以提高精度
    for _ in 0..2 {
        let julian_centuries = current_jd / 36525.0;

        // 计算黄赤交角
        let obliquity = (84381.4060 - 46.836769 * julian_centuries) / RAD;

        // 力学时修正
        let mechanical_time = julian_centuries
            + (32.0 * pow2(julian_centuries + 1.8) - 20.0) / SECONDS_PER_DAY / JULIAN_CENTURY_DAYS;

        // 计算太阳黄经（简化模型）
        let solar_longitude =
            (48950621.66 + 6283319653.318 * mechanical_time + 53.0 * pow2(mechanical_time) - 994.0
                + 334166.0 * cos(4.669257 + 628.307585 * mechanical_time)
                + 3489.0 * cos(4.6261 + 1256.61517 * mechanical_time)
                + 2060.6 * cos(2.67823 + 628.307585 * mechanical_time) * mechanical_time)
                / 10_000_000.0;

        let sin_longitude = sin(solar_longitude);
        let cos_longitude = cos(solar_longitude);

        // 计算格林尼治恒星时
        let sidereal_time = (0.7790572732640 + 1.002_737_811_911_354_6 * current_jd) * PI2
            + (0.014506 + 4612.15739966 * julian_centuries + 1.39667721 * pow2(julian_centuries))
                / RAD;

        // 计算太阳赤道坐标
        let right_ascension = atan2(sin_longitude * cos(obliquity), cos_longitude);
        let declination = asin(sin(obliquity) * sin_longitude);

        // 计算太阳时角
        let cos_hour_angle = (sin(SUN_ALTITUDE_CORRECTION / RAD)
            - sin(latitude) * sin(declination))
            / (cos(latitude) * cos(declination));

        if cos_hour_angle.abs() >= 1.0 {
            return 0.0; // 极昼或极夜情况
        }

        let hour_angle = time_type * acos(cos_hour_angle);
        // 修正儒略日
        current_jd +=
            normalize_rad(hour_angle - (sidereal_time + longitude - right_ascension)) / PI2;
    }

    current_jd
}

/// 计算高精度时差（真太阳时与平太阳时之差）
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数（力学时）
///
/// # 返回值
/// 时差（天）
pub fn calculate_equation_of_time_high_precision(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;

    // 计算平太阳黄经
    let mean_solar_longitude = (1753470142.0 + 628331965331.8 * t + 5296.74 * t2 + 0.432 * t3
        - 0.1124 * t4
        - 0.00009 * t5)
        / 1_000_000_000.0
        + PI
        - 20.5 / RAD;

    // 章动修正
    let nutation_longitude = -17.2 * sin(2.1824 - 33.75705 * t) / RAD;
    let nutation_obliquity = 9.2 * cos(2.1824 - 33.75705 * t) / RAD;
    let true_obliquity = calculate_obliquity_p03(t) + nutation_obliquity;

    // 计算真太阳坐标
    let mut solar_position = Vector3::new(
        calculate_planet_coordinate(0, 0, t, 50)
            + PI
            + calculate_solar_aberration_longitude(t)
            + nutation_longitude,
        -(2796.0 * cos(3.1987 + 8433.46616 * t)
            + 1016.0 * cos(5.4225 + 550.75532 * t)
            + 804.0 * cos(3.88 + 522.3694 * t))
            / 1_000_000_000.0,
        0.0,
    );

    // 转换到赤道坐标并修正章动影响
    solar_position = llr_conv(solar_position, true_obliquity);
    solar_position.x -= nutation_longitude * cos(true_obliquity);

    // 计算时差
    let time_difference = normalize_rad(mean_solar_longitude - solar_position.x);
    time_difference / PI2 // 转换为天
}

/// 计算低精度时差
///
/// 适用于对精度要求不高的应用，计算速度更快
pub fn calculate_equation_of_time_low_precision(julian_centuries: f64) -> f64 {
    let t = julian_centuries;

    // 平太阳黄经（简化计算）
    let mean_solar_longitude =
        (1753470142.0 + 628331965331.8 * t + 5296.74 * pow2(t)) / 1_000_000_000.0 + PI;

    // 黄赤交角
    let obliquity = (84381.4088 - 46.836051 * t) / RAD;

    // 真太阳坐标（简化）
    let mut solar_position = Vector3::new(calculate_planet_coordinate(0, 0, t, 5) + PI, 0.0, 0.0);

    // 转换到赤道坐标
    solar_position = llr_conv(solar_position, obliquity);

    // 计算时差
    let time_difference = normalize_rad(mean_solar_longitude - solar_position.x);
    time_difference / PI2
}

// =============================================================================
// 黄经相关计算
// =============================================================================

/// 计算地球黄经
pub fn calculate_earth_longitude(julian_centuries: f64, term_count: i32) -> f64 {
    calculate_planet_coordinate(0, 0, julian_centuries, term_count)
}

/// 计算月球黄经
pub fn calculate_lunar_longitude(julian_centuries: f64, term_count: i32) -> f64 {
    calculate_lunar_coordinate(0, julian_centuries, term_count)
}

/// 计算月日视黄经差
pub fn calculate_lunar_solar_longitude_difference(
    julian_centuries: f64,
    lunar_terms: i32,
    solar_terms: i32,
) -> f64 {
    calculate_lunar_longitude(julian_centuries, lunar_terms) + GXC_MOON_LON
        - (calculate_earth_longitude(julian_centuries, solar_terms)
            + calculate_solar_aberration_longitude(julian_centuries)
            + PI)
}

/// 计算太阳视黄经
pub fn calculate_apparent_solar_longitude(julian_centuries: f64, term_count: i32) -> f64 {
    calculate_earth_longitude(julian_centuries, term_count)
        + calculate_longitude_nutation_medium(julian_centuries)
        + calculate_solar_aberration_longitude(julian_centuries)
        + PI
}

/// 计算月球视半径
///
/// # 参数
/// - `distance`: 地月质心距离
/// - `altitude`: 地平高度（弧度）
///
/// # 返回值
/// 月球视半径（角秒）
pub fn calculate_lunar_apparent_radius(distance: f64, altitude: f64) -> f64 {
    LUNAR_APPARENT_RADIUS / distance * (1.0 + sin(altitude) * EARTH_EQUATORIAL_RADIUS_KM / distance)
}

// =============================================================================
// 反算时间函数（已知黄经求时间）
// =============================================================================

/// 根据地球真黄经反算时间
///
/// 使用牛顿迭代法求解
pub fn calculate_time_from_earth_longitude(true_longitude: f64) -> f64 {
    let mut time = (true_longitude - 1.75347) / 628.3319653318;

    // 两次迭代提高精度
    for _ in 0..2 {
        let velocity = calculate_earth_velocity(time);
        time += (true_longitude - calculate_earth_longitude(time, 10)) / velocity;
    }

    // 最终精化
    let final_velocity = calculate_earth_velocity(time);
    time += (true_longitude - calculate_earth_longitude(time, -1)) / final_velocity;

    time
}

/// 根据月球真黄经反算时间
pub fn calculate_time_from_lunar_longitude(true_longitude: f64) -> f64 {
    let mut time = (true_longitude - 3.81034) / 8399.70911033384;

    // 初步修正
    time += (true_longitude - calculate_lunar_longitude(time, 3)) / 8399.70911033384;

    // 迭代精化
    let velocity = calculate_lunar_velocity(time);
    time += (true_longitude - calculate_lunar_longitude(time, 20)) / velocity;
    time += (true_longitude - calculate_lunar_longitude(time, -1)) / velocity;

    time
}

/// 根据月日视黄经差反算时间
pub fn calculate_time_from_lunar_solar_difference(longitude_difference: f64) -> f64 {
    let mut time = (longitude_difference + 1.08472) / 7771.37714500204;

    // 初步修正
    time += (longitude_difference - calculate_lunar_solar_longitude_difference(time, 3, 3))
        / 7771.37714500204;

    // 迭代精化
    let relative_velocity = calculate_lunar_velocity(time) - calculate_earth_velocity(time);
    time += (longitude_difference - calculate_lunar_solar_longitude_difference(time, 20, 10))
        / relative_velocity;
    time += (longitude_difference - calculate_lunar_solar_longitude_difference(time, -1, 60))
        / relative_velocity;

    time
}

/// 根据太阳视黄经反算时间
pub fn calculate_time_from_apparent_solar_longitude(apparent_longitude: f64) -> f64 {
    let mut time = (apparent_longitude - 1.75347 - PI) / 628.3319653318;

    // 两次迭代
    for _ in 0..2 {
        let velocity = calculate_earth_velocity(time);
        time += (apparent_longitude - calculate_apparent_solar_longitude(time, 10)) / velocity;
    }

    // 最终精化
    let final_velocity = calculate_earth_velocity(time);
    time += (apparent_longitude - calculate_apparent_solar_longitude(time, -1)) / final_velocity;

    time
}

/// 快速计算月日视黄经差对应的时间（低精度）
pub fn calculate_time_from_lunar_solar_difference_fast(longitude_difference: f64) -> f64 {
    const BASE_VELOCITY: f64 = 7771.37714500204;
    let mut time = (longitude_difference + 1.08472) / BASE_VELOCITY;
    let time_squared = time * time;

    // 快速修正项
    let correction = (-0.00003309 * time_squared
        + 0.10976 * cos(0.784758 + 8328.6914246 * time + 0.000152292 * time_squared)
        + 0.02224 * cos(0.18740 + 7214.0628654 * time - 0.00021848 * time_squared)
        - 0.03342 * cos(4.669257 + 628.307585 * time))
        / BASE_VELOCITY;

    time -= correction;

    // 计算剩余误差并修正
    let remaining_difference = calculate_lunar_longitude(time, 20)
        - (4.8950632
            + 628.3319653318 * time
            + 0.000005297 * time_squared
            + 0.0334166 * cos(4.669257 + 628.307585 * time)
            + 0.0002061 * cos(2.67823 + 628.307585 * time) * time
            + 0.000349 * cos(4.6261 + 1256.61517 * time)
            - 20.5 / RAD);

    let refined_velocity = 7771.38
        - 914.0 * sin(0.7848 + 8328.691425 * time + 0.0001523 * time_squared)
        - 179.0 * sin(2.543 + 15542.7543 * time)
        - 160.0 * sin(0.1874 + 7214.0629 * time);

    time + remaining_difference / refined_velocity
}

/// 快速计算太阳视黄经对应的时间（低精度）
pub fn calculate_time_from_apparent_solar_longitude_fast(apparent_longitude: f64) -> f64 {
    const BASE_VELOCITY: f64 = 628.3319653318;
    let mut time = (apparent_longitude - 1.75347 - PI) / BASE_VELOCITY;

    // 快速修正项
    let time_squared = time * time;
    let correction = (0.000005297 * time_squared
        + 0.0334166 * cos(4.669257 + 628.307585 * time)
        + 0.0002061 * cos(2.67823 + 628.307585 * time) * time)
        / BASE_VELOCITY;

    time -= correction;

    // 最终修正
    let final_correction = (apparent_longitude - calculate_earth_longitude(time, 8) - PI
        + (20.5 + 17.2 * sin(2.1824 - 33.75705 * time)) / RAD)
        / BASE_VELOCITY;

    time + final_correction
}

// =============================================================================
// 月球光照计算
// =============================================================================

/// 计算月球被照亮部分的比例（月相）
///
/// # 参数
/// - `julian_centuries`: 从 J2000.0 起算的儒略世纪数
///
/// # 返回值
/// 照亮比例 (0.0 = 新月, 1.0 = 满月)
pub fn calculate_lunar_illumination(julian_centuries: f64) -> f64 {
    let t = julian_centuries;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let degrees_to_radians = PI / 180.0;

    // 计算月球相位角参数
    let elongation = (297.8502042 + 445267.1115168 * t - 0.0016300 * t2 + t3 / 545868.0
        - t4 / 113065000.0)
        * degrees_to_radians;
    let solar_anomaly =
        (357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24490000.0) * degrees_to_radians;
    let lunar_anomaly = (134.9634114 + 477198.8676313 * t + 0.0089970 * t2 + t3 / 69699.0
        - t4 / 14712000.0)
        * degrees_to_radians;

    // 计算相位角
    let phase_angle = PI - elongation
        + (-6.289 * sin(lunar_anomaly) + 2.100 * sin(solar_anomaly)
            - 1.274 * sin(2.0 * elongation - lunar_anomaly)
            - 0.658 * sin(2.0 * elongation)
            - 0.214 * sin(2.0 * lunar_anomaly)
            - 0.110 * sin(elongation))
            * degrees_to_radians;

    // 计算照亮比例
    (1.0 + cos(phase_angle)) / 2.0
}

// =============================================================================
// 地球轨道特殊点计算
// =============================================================================

/// 计算地球近日点或远日点
///
/// # 参数
/// - `julian_centuries`: 参考时间（儒略世纪数）
/// - `is_perihelion`: 是否为近日点（true=近日点, false=远日点）
///
/// # 返回值
/// Vector2: (发生时间, 日地距离)
pub fn calculate_earth_apsis(julian_centuries: f64, is_perihelion: bool) -> Vector2 {
    const ORBITAL_PERIOD: f64 = EARTH_ANOMALISTIC_YEAR / 36525.0;
    let phase_offset = if is_perihelion { 1.7 } else { 184.5 } / 36525.0;

    let mut event_time = phase_offset
        + ORBITAL_PERIOD * floor((julian_centuries - phase_offset) / ORBITAL_PERIOD + 0.5);

    // 三级精度迭代
    for &time_step in &[3.0, 0.2, 0.01] {
        let step = time_step / 36525.0;
        let (distance_before, distance_at, distance_after) = (
            calculate_planet_coordinate(
                0,
                2,
                event_time - step,
                if time_step == 0.01 { -1 } else { 80 },
            ),
            calculate_planet_coordinate(0, 2, event_time, if time_step == 0.01 { -1 } else { 80 }),
            calculate_planet_coordinate(
                0,
                2,
                event_time + step,
                if time_step == 0.01 { -1 } else { 80 },
            ),
        );

        // 使用抛物线插值精化时间
        event_time += (distance_before - distance_after)
            / (distance_before + distance_after - 2.0 * distance_at)
            * step
            / 2.0;
    }

    // 计算最终距离（包含插值修正）
    let step = 0.01 / 36525.0;
    let (distance_before, distance_at, distance_after) = (
        calculate_planet_coordinate(0, 2, event_time - step, -1),
        calculate_planet_coordinate(0, 2, event_time, -1),
        calculate_planet_coordinate(0, 2, event_time + step, -1),
    );

    let final_distance = distance_at
        + (distance_before - distance_after)
            / (distance_before + distance_after - 2.0 * distance_at)
            * (distance_after - distance_before)
            / 8.0;

    Vector2::new(event_time, final_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sun_rise_set_calculation() {
        let jd = 2451545.0; // J2000
        let longitude = 0.0; // 格林尼治
        let latitude = 0.0; // 赤道

        let rise_time = calculate_sun_rise_set(jd, longitude, latitude, 1.0);
        assert!(rise_time > 0.0);
    }

    #[test]
    fn test_lunar_illumination() {
        let illumination = calculate_lunar_illumination(0.1);
        assert!(illumination >= 0.0 && illumination <= 1.0);
    }

    #[test]
    fn test_earth_apsis() {
        let result = calculate_earth_apsis(0.1, true);
        assert!(result.x > 0.0 && result.y > 0.0);
    }
}
