use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

mod original_holiday_strings;
use original_holiday_strings::LEGAL_HOLIDAY_DATA;

pub const HOLIDAYS_HEADER: &str = r#"// 自动生成的法定节假日数据

/// 法定节假日条目
#[derive(Debug, Clone, Copy)]
pub struct LegalHolidayEntry {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub work: bool,
    pub index: u8,
}"#;

pub fn generate_holidays_data() -> Result<()> {
    // 生成 Rust 代码
    let mut content = format!("{}\n\n", HOLIDAYS_HEADER);

    let record_count = LEGAL_HOLIDAY_DATA.len() / 13;
    content.push_str(&format!(
        "pub const LEGAL_HOLIDAY_TABLE: [LegalHolidayEntry; {}] = [\n",
        record_count
    ));

    for i in 0..record_count {
        let start = i * 13;
        let record = &LEGAL_HOLIDAY_DATA[start..start + 13];

        let year = &record[0..4];
        let month = &record[4..6];
        let day = &record[6..8];
        let work_char = &record[8..9];
        let index_char = &record[9..10];

        let work = work_char == "0";
        let index = index_char.parse::<u8>().unwrap();

        content.push_str(&format!(
            "    LegalHolidayEntry {{ year: {}, month: {}, day: {}, work: {}, index: {} }},\n",
            year, month, day, work, index
        ));
    }

    content.push_str("];\n");

    let dest_path = Path::new("src").join("generated_holidays_data.rs");

    // 写入文件
    let mut f = File::create(&dest_path).unwrap();
    writeln!(f, "{}", content)?;

    Ok(())
}
