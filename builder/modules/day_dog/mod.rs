use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

pub const DAY_DOG_HEADER: &str = r#"//! 此文件由 build.rs 自动生成，不要手动修改。
"#;

mod original_strings;
use original_strings::DAY_GODS;

pub fn generate_day_dog_data() -> Result<()> {
    let dest_path = Path::new("src")
        .join("culture")
        .join("generated_day_dog_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "{}", DAY_DOG_HEADER)?;

    writeln!(f, "// 自动生成的 Day Gods 数据").unwrap();

    writeln!(f, "#[rustfmt::skip]").unwrap();
    writeln!(
        f,
        "pub static DAY_GODS_TABLE: [[Option<&[u8]>; 60]; 12] = ["
    )
    .unwrap();

    for month_data in DAY_GODS.iter() {
        writeln!(f, "    [").unwrap();

        let mut day_entries: [Option<&'static [u8]>; 60] = [None; 60];

        for segment in month_data.split(';') {
            if segment.len() >= 2 {
                if let Ok(day_index) = u8::from_str_radix(&segment[0..2], 16) {
                    if day_index < 60 {
                        let data_str = &segment[2..];
                        // 生成静态数组
                        let data_vec: Vec<u8> = data_str
                            .as_bytes()
                            .chunks(2)
                            .filter_map(|chunk| {
                                if chunk.len() == 2 {
                                    std::str::from_utf8(chunk)
                                        .ok()
                                        .and_then(|s| u8::from_str_radix(s, 16).ok())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        if !data_vec.is_empty() {
                            writeln!(f, "        Some(&{:?}),", data_vec).unwrap();
                            day_entries[day_index as usize] =
                                Some(Box::leak(data_vec.into_boxed_slice()));
                        }
                    }
                }
            }
        }

        // 对于没有数据的天，写入 None
        for day_idx in 0..60 {
            if day_entries[day_idx].is_none() {
                writeln!(f, "        None, // 天 {}", day_idx).unwrap();
            }
        }

        writeln!(f, "    ],").unwrap();
    }
    
    writeln!(f, "];").unwrap();

    Ok(())
}
