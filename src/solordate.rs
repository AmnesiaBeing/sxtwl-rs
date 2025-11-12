//! 公历处理模块
//! 提供公历结构体创建功能（含合法性校验）及打印功能

use alloc::string::String;
use alloc::string::ToString;
use libm::floor;
use core::fmt;

use crate::types::SolarDate;

/// SolarDate 的扩展功能实现
impl SolarDate {
    /// 创建一个新的 SolarDate 实例，对输入参数进行合法性校验
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> Option<Self> {
        // 校验月份范围
        if month < 1 || month > 12 {
            return None;
        }
        
        // 校验日期范围（考虑不同月份的天数）
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                // 闰年判断
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            },
            _ => 0, // 不应该到达这里，因为已经检查过月份范围
        };
        
        if day < 1 || day > days_in_month {
            return None;
        }
        
        // 校验小时范围
        if hour > 23 {
            return None;
        }
        
        // 校验分钟范围
        if minute > 59 {
            return None;
        }
        
        // 校验秒数范围
        if second < 0.0 || second >= 61.0 { // 包含闰秒的情况
            return None;
        }
        
        Some(SolarDate {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
    
    /// 生成格式化的年月日字符串，如：2025年11月12日
    pub fn format_ymd(&self) -> String {
        let mut result = String::new();
        
        // 处理年份（支持公元前）
        if self.year < 0 {
            result.push_str(&(-self.year + 1).to_string());
            result.push_str("年（前）");
        } else {
            result.push_str(&self.year.to_string());
            result.push('年');
        }
        
        // 添加月份
        result.push_str(&self.month.to_string());
        result.push('月');
        
        // 添加日期
        result.push_str(&self.day.to_string());
        result.push('日');
        
        result
    }
    
    /// 生成格式化的时分秒字符串，如：23:59:59
    pub fn format_hms(&self) -> String {
        let mut result = String::new();
        
        // 添加小时（两位数）
        if self.hour < 10 {
            result.push('0');
        }
        result.push_str(&self.hour.to_string());
        result.push(':');
        
        // 添加分钟（两位数）
        if self.minute < 10 {
            result.push('0');
        }
        result.push_str(&self.minute.to_string());
        result.push(':');
        
        // 添加秒数（取整，两位数）
        let second_int = floor(self.second) as u8;
        if second_int < 10 {
            result.push('0');
        }
        result.push_str(&second_int.to_string());
        
        result
    }
}

/// 实现 Display trait 以支持直接打印 SolarDate
impl fmt::Display for SolarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.format_ymd(), self.format_hms())
    }
}