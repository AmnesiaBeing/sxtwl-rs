//! 工具函数模块

use crate::gz::GanZhi;

/// 计算两个角度之间的最小差异（考虑360度循环）
pub fn angle_diff(a: f64, b: f64) -> f64 {
    let diff = (b - a) % 360.0;
    if diff > 180.0 {
        diff - 360.0
    } else if diff < -180.0 {
        diff + 360.0
    } else {
        diff
    }
}

/// 二分搜索函数，用于查找数组中满足条件的元素
pub fn bisect_search<F>(low: f64, high: f64, f: F, tolerance: f64, max_iterations: u32) -> f64 
where F: Fn(f64) -> f64 {
    let mut left = low;
    let mut right = high;
    let mut iterations = 0;
    
    while (right - left).abs() > tolerance && iterations < max_iterations {
        let mid = (left + right) / 2.0;
        let value = f(mid);
        
        if value < 0.0 {
            left = mid;
        } else {
            right = mid;
        }
        
        iterations += 1;
    }
    
    (left + right) / 2.0
}

/// 将角度归一化到0-360度范围内
pub fn normalize_angle(angle: f64) -> f64 {
    let normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized + 360.0
    } else {
        normalized
    }
}

/// 计算天干地支索引（内部函数）
pub fn calculate_gz_index(gz: &GanZhi) -> Result<u8, &'static str> {
    if gz.tian_gan > 9 || gz.di_zhi > 11 {
        return Err("无效的天干地支值");
    }
    
    for i in 0..6 {
        if (gz.tian_gan + i * 10) % 12 == gz.di_zhi as u8 {
            return Ok(gz.tian_gan + i * 10);
        }
    }
    Err("无法找到对应的干支索引")
}

/// 根据年份差值计算对应年份的天干地支
pub fn calculate_gz_from_diff(diff: i32) -> GanZhi {
    // 以1984年为基准（甲子年）
    let tg = (diff.abs() % 10) as u8;
    let dz = (diff.abs() % 12) as u8;
    
    // 如果是负数年（公元前），需要调整
    let (tg, dz) = if diff < 0 {
        let tg_adj = if tg > 0 { 10 - tg } else { 0 };
        let dz_adj = if dz > 0 { 12 - dz } else { 0 };
        (tg_adj, dz_adj)
    } else {
        (tg, dz)
    };
    
    GanZhi { tian_gan: tg, di_zhi: dz }
}

/// 计算某年是否有闰月及闰哪个月
pub fn calculate_leap_month(year: i32) -> Option<u8> {
    // 这是一个简化实现
    // 实际应该根据天文计算或查表来确定闰月
    let leap_months = [
        (2023, 2), (2025, 6), (2028, 5), (2031, 3), (2033, 11),
        (2036, 6), (2039, 5), (2042, 2), (2044, 7), (2047, 5),
    ];
    
    for &(y, month) in leap_months.iter() {
        if y == year {
            return Some(month);
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_angle_functions() {
        assert_relative_eq!(normalize_angle(370.0), 10.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(-10.0), 350.0, epsilon = 1e-6);
        assert_relative_eq!(angle_diff(350.0, 10.0), 20.0, epsilon = 1e-6);
        assert_relative_eq!(angle_diff(10.0, 350.0), -20.0, epsilon = 1e-6);
    }
    
    #[test]
    fn test_gz_calculation() {
        // 测试1984年（基准年）
        let gz = calculate_gz_from_diff(0);
        assert_eq!(gz.tian_gan, 0); // 甲
        assert_eq!(gz.di_zhi, 0);  // 子
        
        // 测试2023年（1984+39）
        let gz = calculate_gz_from_diff(39);
        assert_eq!(gz.tian_gan, 9); // 癸
        assert_eq!(gz.di_zhi, 3);   // 卯
    }
    
    #[test]
    fn test_bisect_search() {
        // 测试二分搜索
        let f = |x: f64| x * x - 2.0; // 求解 sqrt(2)
        let result = bisect_search(1.0, 2.0, f, 1e-10, 100);
        assert_relative_eq!(result, 2.0f64.sqrt(), epsilon = 1e-8);
    }
}