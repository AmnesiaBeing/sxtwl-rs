// build.rs
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

pub const RAB_BYUNG_MONTH_DAYS_HEADER: &str = r#"//! 此文件由 build.rs 自动生成，不要手动修改。
//! 自动生成的 RabByung 数据

#[derive(Debug, Clone)]
pub struct RabByungMonthData {
    pub year: u16,
    pub month: u8, 
    pub days: &'static [isize],
}
"#;

pub const RAB_BYUNG_MONTH_DAYS_FUNCTIONS: &str = r#"
pub fn get_rab_byung_month_days(year: usize, month: usize) -> Option<&'static [isize]> {
    RAB_BYUNG_DATA
        .iter()
        .find(|data| data.year == year as u16 && data.month == month as u8)
        .map(|data| data.days)
}"#;

mod original_strings;
use original_strings::RAW_DATA;

pub fn generate_rab_byung_data() -> Result<()> {
    // 生成 Rust 代码
    let mut content = format!("{}\n", RAB_BYUNG_MONTH_DAYS_HEADER);
    content.push_str("#[rustfmt::skip]\n");
    content.push_str("pub static RAB_BYUNG_DATA: &[RabByungMonthData] = &[\n");

    let years: Vec<&str> = RAW_DATA.split(',').collect();
    let mut y: usize = 1950;
    let mut m: usize = 11;

    for s in years {
        let mut ys = s;
        while !ys.is_empty() {
            let mut chars = ys.chars();
            let len_char = chars.next().unwrap();
            let len = (len_char as usize) - ('0' as usize);

            let mut days_array = Vec::new();
            for _ in 0..len {
                if let Some(ch) = chars.next() {
                    let day_value = ch as isize - '5' as isize - 30;
                    days_array.push(day_value);
                }
            }

            // 生成静态数组条目
            content.push_str(&format!(
                "    RabByungMonthData {{ year: {}, month: {}, days: &{:?} }},\n",
                y, m, days_array
            ));

            // 更新位置
            m += 1;
            ys = chars.as_str();
        }
        y += 1;
        m = 0;
    }

    content.push_str("];\n");
    content.push_str(RAB_BYUNG_MONTH_DAYS_FUNCTIONS);

    let dest_path = Path::new("src").join("generated_rab_byung.rs");

    // 写入文件
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "{}", content)?;

    Ok(())
}
