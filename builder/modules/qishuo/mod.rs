use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

mod original_qishuo_strings;
use original_qishuo_strings::{QI_S, SHUO_S};

pub const QISHUO_HEADER: &str = r#"// 自动生成的压缩数据 - 请勿手动修改
// 此文件由 build.rs 自动生成

// 每个字符用2位存储: 00=0, 01=1, 10=2"#;

pub const GET_SHUO_FUNCTION: &str = r#"/// 从2位压缩数据中获取指定索引的值 (0, 1, 或 2)
pub fn get_shuo_value(index: usize) -> u8 {
    if index >= SHUO_LEN {
        return 0;
    }

    let byte_index = index / 4; // 每字节存储4个值
    let bit_offset = (index % 4) * 2; // 每个值占2位
    let shift = 6 - bit_offset; // 大端序，第一个值在最高2位

    ((SHUO_BYTES[byte_index] >> shift) & 0b11) as u8
}"#;

pub const GET_QI_FUNCTION: &str = r#"/// 从2位压缩数据中获取指定索引的值 (0, 1, 或 2)
pub fn get_qi_value(index: usize) -> u8 {
    if index >= QI_LEN {
        return 0;
    }

    let byte_index = index / 4; // 每字节存储4个值
    let bit_offset = (index % 4) * 2; // 每个值占2位
    let shift = 6 - bit_offset; // 大端序，第一个值在最高2位

    ((QI_BYTES[byte_index] >> shift) & 0b11) as u8
}"#;

fn jieya(s: &str) -> String {
    let o = "0000000000";
    let o2 = format!("{o}{o}");

    let mut result = s.to_string();

    let replacements = [
        ("J", "00"),
        ("I", "000"),
        ("H", "0000"),
        ("G", "00000"),
        ("t", "02"),
        ("s", "002"),
        ("r", "0002"),
        ("q", "00002"),
        ("p", "000002"),
        ("o", "0000002"),
        ("n", "00000002"),
        ("m", "000000002"),
        ("l", "0000000002"),
        ("k", "01"),
        ("j", "0101"),
        ("i", "001"),
        ("h", "001001"),
        ("g", "0001"),
        ("f", "00001"),
        ("e", "000001"),
        ("d", "0000001"),
        ("c", "00000001"),
        ("b", "000000001"),
        ("a", "0000000001"),
        ("A", &format!("{o2}{o2}{o2}")),
        ("B", &format!("{o2}{o2}{o}")),
        ("C", &format!("{o2}{o2}")),
        ("D", &format!("{o2}{o}")),
        ("E", &o2),
        ("F", o),
    ];

    for (from, to) in replacements.iter() {
        result = result.replace(from, to);
    }

    result
}

fn string_to_two_bits(s: &str) -> (Vec<u8>, usize) {
    let mut bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for ch in s.chars() {
        let value = match ch {
            '0' => 0b00, // 00 = 0
            '1' => 0b01, // 01 = 1
            '2' => 0b10, // 10 = 2
            _ => 0b00,   // 默认处理为0
        };

        // 将2位值放入当前字节
        current_byte |= value << (6 - bit_count);
        bit_count += 2;

        // 每4个字符填满一个字节 (4 * 2位 = 8位)
        if bit_count == 8 {
            bytes.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    // 处理最后不满4个字符的情况
    if bit_count > 0 {
        bytes.push(current_byte);
    }

    (bytes, s.len())
}

pub fn generate_qishuo_data() -> Result<()> {
    let dest_path = Path::new("src")
        .join("sxtwl")
        .join("generated_compressed_qishuo_correction_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    // 处理朔日表
    let shuo_decompressed = jieya(SHUO_S);
    let (shuo_bytes, shuo_len) = string_to_two_bits(&shuo_decompressed);

    // 处理节气表
    let qi_decompressed = jieya(QI_S);
    let (qi_bytes, qi_len) = string_to_two_bits(&qi_decompressed);

    // 生成 Rust 代码
    writeln!(&mut f, "{}", QISHUO_HEADER)?;
    writeln!(&mut f, "pub const SHUO_BYTES: &[u8] = &{shuo_bytes:?};")?;
    writeln!(&mut f, "pub const SHUO_LEN: usize = {shuo_len};")?;
    writeln!(&mut f, "pub const QI_BYTES: &[u8] = &{qi_bytes:?};")?;
    writeln!(&mut f, "pub const QI_LEN: usize = {qi_len};")?;
    writeln!(&mut f)?;
    writeln!(&mut f, "{}", GET_SHUO_FUNCTION)?;
    writeln!(&mut f)?;
    writeln!(&mut f, "{}", GET_QI_FUNCTION)?;

    Ok(())
}
