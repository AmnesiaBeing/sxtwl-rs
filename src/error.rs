//! 自定义错误类型模块
//! 定义历法计算中可能出现的各类错误

use thiserror_no_std::Error;

/// 历法计算相关错误类型
#[derive(Debug, Error, PartialEq)]
pub enum CalendarError {
    /// 日期无效（如月份超出1-12范围）
    #[error("无效日期: {0}")]
    InvalidDate(String),

    /// 数字转换失败（如超出中文数字支持范围）
    #[error("无效数字: {0}")]
    InvalidNumber(String),

    /// 天干地支索引无效（如超出0-9或0-12范围）
    #[error("无效的天干地支索引")]
    InvalidGanZhi,

    /// 农历月份无效（超出1-12范围）
    #[error("无效的农历月份")]
    InvalidLunarMonth,

    /// 农历日期无效（超出1-30范围）
    #[error("无效的农历日期")]
    InvalidLunarDay,

    /// 星期索引无效（超出0-6范围）
    #[error("无效的星期索引")]
    InvalidWeek,

    /// 儒略日转换失败（如日期超出计算范围）
    #[error("儒略日转换失败: {0}")]
    JulianConversionError(String),

    /// 节气计算失败（如年份超出支持范围）
    #[error("节气计算失败: {0}")]
    JieQiCalculationError(String),

    /// 内存分配失败（no_std环境下可能出现）
    #[error("内存分配失败")]
    AllocationError,
}
