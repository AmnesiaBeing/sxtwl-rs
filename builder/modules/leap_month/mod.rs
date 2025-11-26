use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

pub const LEAP_YEAR_HEADER: &str = r#"//! 此文件由 build.rs 自动生成，不要手动修改。
//! 包含了预计算的闰月查找表数据。

use core::option::Option;"#;

pub const LEAP_MONTH_FUNCTIONS: &str = r#"/// 根据年份和月份获取累计值。
/// 索引从 0 开始。
/// 如果索引超出范围，返回 None。
pub fn get_leap_month_value(month: usize, day: usize) -> Option<isize> {
    if month >= LEAP_MONTH_YEAR_DATA.len() {
        return None;
    }
    let month_data = LEAP_MONTH_YEAR_DATA[month];
    if day >= month_data.len() {
        return None;
    }
    Some(month_data[day])
}

/// 判断某一年的某个月是否为闰月（示例逻辑）。
/// 如果 `get_leap_month_value` 返回 Some(_)，则认为是闰月。
pub fn is_leap_month(month: usize) -> bool {
    month < LEAP_MONTH_YEAR_DATA.len() && !LEAP_MONTH_YEAR_DATA[month].is_empty()
}"#;

mod original_leap_month_strings;
use original_leap_month_strings::{CHARS, LEAP_MONTH};

pub fn generate_leap_year_data() -> Result<()> {
    let mut leap_month_data = Vec::new();
    let mut max_days_in_month = 0;

    for &month_str in LEAP_MONTH.iter() {
        let mut month_values = Vec::new();
        let mut n: isize = 0;

        assert!(
            month_str.len() % 2 == 0,
            "Month string length is not even: {}",
            month_str
        );

        for i in (0..month_str.len()).step_by(2) {
            let s = &month_str[i..i + 2];
            let c1 = s.chars().next().unwrap();
            let c2 = s.chars().nth(1).unwrap();

            let val1 = CHARS.find(c1).unwrap() as isize;
            let val2 = CHARS.find(c2).unwrap() as isize;

            let t = val1 * 64 + val2;
            n += t;
            month_values.push(n);
        }

        if month_values.len() > max_days_in_month {
            max_days_in_month = month_values.len();
        }
        leap_month_data.push(month_values);
    }

    let dest_path = Path::new("src").join("generated_leap_year_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "{}", LEAP_YEAR_HEADER)?;
    writeln!(f)?;
    writeln!(f, "/// 预计算的闰月数据，格式为 [月份][日期] -> 累计值")?;
    writeln!(f, "/// 注意：并非所有月份都有相同的天数。")?;
    writeln!(f, "pub static LEAP_MONTH_YEAR_DATA: &[&[isize]] = &[")?;
    for month_values in &leap_month_data {
        writeln!(f, "    &{:?},", month_values)?;
    }
    writeln!(f, "];")?;
    writeln!(f)?;
    writeln!(f, "{}", LEAP_MONTH_FUNCTIONS)?;

    Ok(())
}
