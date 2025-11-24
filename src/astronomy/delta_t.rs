//! ΔT (Delta T) 计算模块
//!
//! 实现地球时(TT)与世界时(UT)时间差的计算，用于修正地球自转的不规则性。
//! 基于历史观测数据的插值和外推算法。

use crate::{
    astronomy::coefficients::DT_AT,
    consts::{SECONDS_PER_DAY, TROPICAL_YEAR_DAYS},
};

/// 使用二次曲线进行外推计算
///
/// # 参数
/// - `year`: 目标年份
/// - `acceleration_estimate`: 加速度估计值（秒/世纪²）
///
/// # 返回值
/// ΔT 值（秒）
fn extrapolate_quadratic(year: f64, acceleration_estimate: i32) -> f64 {
    let centuries = (year - 1820.0) / 100.0;
    -20.0 + (acceleration_estimate as f64) * centuries * centuries
}

/// 计算世界时(UT)与原子时(TAI)之差 ΔT
///
/// # 参数
/// - `year`: 年份（十进制，如 2023.5）
///
/// # 返回值
/// ΔT 值（秒）
///
/// # 算法说明
/// - 对于历史数据：使用三次样条插值
/// - 对于未来数据：使用二次曲线外推，并进行平滑过渡
pub fn calculate_delta_t(year: f64) -> f64 {
    let table_len = DT_AT.len();

    let last_year_index = table_len - 2;
    let reference_year = DT_AT[last_year_index]; // 表中最后一年的年份
    let reference_delta_t = DT_AT[last_year_index + 1]; // 表中最后一年的ΔT值

    // 处理未来年份的外推
    if year >= reference_year {
        // 不同数据源的加速度估计值：
        // - 瑞士星历表: 31
        // - NASA网站: 32
        // - skmap: 29
        let acceleration_estimate = 31;

        if year > reference_year + 100.0 {
            // 超过100年，直接使用二次外推
            return extrapolate_quadratic(year, acceleration_estimate);
        }

        // 平滑过渡：在100年范围内进行线性修正
        let extrapolated_value = extrapolate_quadratic(year, acceleration_estimate); // 二次曲线外推
        let correction =
            extrapolate_quadratic(reference_year, acceleration_estimate) - reference_delta_t; // ye年的二次外推与te的差
        return extrapolated_value - correction * (reference_year + 100.0 - year) / 100.0;
    }

    // 查找对应的数据区间进行插值
    let data_interval = find_data_interval(year);
    interpolate_cubic(year, data_interval)
}

/// 在数据表中查找对应的区间索引
fn find_data_interval(year: f64) -> usize {
    // 数据表结构：每5个元素为一组 [年份, 系数1, 系数2, 系数3, 系数4]
    for i in (0..DT_AT.len() - 5).step_by(5) {
        if year < DT_AT[i + 5] {
            return i;
        }
    }

    // 如果没有找到合适的区间，使用最后一个区间
    DT_AT.len() - 5
}

/// 使用三次多项式进行插值
fn interpolate_cubic(year: f64, interval_index: usize) -> f64 {
    let data = &DT_AT[interval_index..interval_index + 5];
    let next_year = DT_AT[interval_index + 5];

    // 归一化参数 t ∈ [0, 10]
    let t = (year - data[0]) / (next_year - data[0]) * 10.0;
    let t2 = t * t;
    let t3 = t2 * t;

    // 三次多项式：c0 + c1*t + c2*t² + c3*t³
    data[1] + data[2] * t + data[3] * t2 + data[4] * t3
}

/// 计算从J2000起算的儒略日对应的ΔT（单位：日）
///
/// # 参数
/// - `days_since_j2000`: 从J2000历元起算的儒略日
///
/// # 返回值
/// ΔT 值（日）
pub fn delta_t_from_j2000(days_since_j2000: f64) -> f64 {
    // 转换为年份：J2000 = 2000.0
    let year = days_since_j2000 / TROPICAL_YEAR_DAYS + 2000.0;
    calculate_delta_t(year) / SECONDS_PER_DAY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate_quadratic() {
        let result = extrapolate_quadratic(2100.0, 31);
        // 验证外推结果在合理范围内
        assert!(result > 0.0 && result < 1000.0);
    }

    #[test]
    fn test_delta_t_conversion() {
        let delta_t_days = delta_t_from_j2000(0.0); // J2000时刻
        assert!(delta_t_days.abs() < 1.0); // 应该在1天以内
    }
}
