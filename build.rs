use std::fs::File;
use std::io::Write;
use std::path::Path;

mod original_qishuo_strings;
use original_qishuo_strings::{QI_S, SHUO_S};

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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=original_qishuo_strings.rs");

    // 生成到 src 目录
    let dest_path = Path::new("src").join("compressed_qishuo_correction_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    // 处理朔日表
    let shuo_decompressed = jieya(SHUO_S);
    let (shuo_bytes, shuo_len) = string_to_two_bits(&shuo_decompressed);

    // 处理节气表
    let qi_decompressed = jieya(QI_S);
    let (qi_bytes, qi_len) = string_to_two_bits(&qi_decompressed);

    // 生成 Rust 代码
    writeln!(&mut f, "// 自动生成的压缩数据 - 请勿手动修改").unwrap();
    writeln!(&mut f, "// 此文件由 build.rs 自动生成").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "use libm::floor;").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(
        &mut f,
        "use crate::consts::{{LUNAR_MONTH_DAYS, JIEQI_PER_YEAR, SOLAR_YEAR_DAYS}};"
    )
    .unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "// 每个字符用2位存储: 00=0, 01=1, 10=2").unwrap();
    writeln!(&mut f, "pub const SHUO_BYTES: &[u8] = &{shuo_bytes:?};").unwrap();
    writeln!(&mut f, "pub const SHUO_LEN: usize = {shuo_len};").unwrap();
    writeln!(&mut f, "pub const QI_BYTES: &[u8] = &{qi_bytes:?};").unwrap();
    writeln!(&mut f, "pub const QI_LEN: usize = {qi_len};").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "/// 从2位压缩数据中获取指定索引的值 (0, 1, 或 2)").unwrap();
    writeln!(&mut f, "fn get_shuo_value(index: usize) -> u8 {{").unwrap();
    writeln!(&mut f, "    if index >= SHUO_LEN {{").unwrap();
    writeln!(&mut f, "        return 0;").unwrap();
    writeln!(&mut f, "    }}").unwrap();
    writeln!(&mut f).unwrap();
    writeln!(&mut f, "    let byte_index = index / 4; // 每字节存储4个值").unwrap();
    writeln!(
        &mut f,
        "    let bit_offset = (index % 4) * 2; // 每个值占2位"
    )
    .unwrap();
    writeln!(
        &mut f,
        "    let shift = 6 - bit_offset; // 大端序，第一个值在最高2位"
    )
    .unwrap();
    writeln!(&mut f).unwrap();
    writeln!(
        &mut f,
        "    ((SHUO_BYTES[byte_index] >> shift) & 0b11) as u8"
    )
    .unwrap();
    writeln!(&mut f, "}}").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "/// 从2位压缩数据中获取指定索引的值 (0, 1, 或 2)").unwrap();
    writeln!(&mut f, "fn get_qi_value(index: usize) -> u8 {{").unwrap();
    writeln!(&mut f, "    if index >= QI_LEN {{").unwrap();
    writeln!(&mut f, "        return 0;").unwrap();
    writeln!(&mut f, "    }}").unwrap();
    writeln!(&mut f).unwrap();
    writeln!(&mut f, "    let byte_index = index / 4; // 每字节存储4个值").unwrap();
    writeln!(
        &mut f,
        "    let bit_offset = (index % 4) * 2; // 每个值占2位"
    )
    .unwrap();
    writeln!(
        &mut f,
        "    let shift = 6 - bit_offset; // 大端序，第一个值在最高2位"
    )
    .unwrap();
    writeln!(&mut f).unwrap();
    writeln!(&mut f, "    ((QI_BYTES[byte_index] >> shift) & 0b11) as u8").unwrap();
    writeln!(&mut f, "}}").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(
        &mut f,
        "pub(crate) fn get_shuo_correction(jd: f64, f2: f64) -> u8 {{"
    )
    .unwrap();
    writeln!(
        &mut f,
        "    let index = floor((jd - f2) / LUNAR_MONTH_DAYS) as usize;"
    )
    .unwrap();
    writeln!(&mut f, "    get_shuo_value(index)").unwrap();
    writeln!(&mut f, "}}").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(
        &mut f,
        "pub(crate) fn get_qi_correction(jd: f64, f2: f64) -> u8 {{"
    )
    .unwrap();
    writeln!(
        &mut f,
        "    let index = floor((jd - f2) / SOLAR_YEAR_DAYS * JIEQI_PER_YEAR) as usize;"
    )
    .unwrap();
    writeln!(&mut f, "    get_qi_value(index)").unwrap();
    writeln!(&mut f, "}}").unwrap();

    // 验证解压结果
    writeln!(&mut f).unwrap();
    writeln!(&mut f, "// 验证数据").unwrap();
    writeln!(&mut f, "#[cfg(test)]").unwrap();
    writeln!(&mut f, "mod tests {{").unwrap();
    writeln!(&mut f, "    use super::*;").unwrap();
    writeln!(&mut f).unwrap();
    writeln!(&mut f, "    #[test]").unwrap();
    writeln!(&mut f, "    fn verify_lengths() {{").unwrap();
    writeln!(&mut f, "        assert_eq!(SHUO_LEN, {shuo_len});").unwrap();
    writeln!(&mut f, "        assert_eq!(QI_LEN, {qi_len});").unwrap();
    writeln!(&mut f, "    }}").unwrap();
    writeln!(&mut f, "}}").unwrap();

    println!("生成压缩数据到: {dest_path:?}");
    println!(
        "朔日表: {}字符 -> {}字节 (压缩率: {:.1}%)",
        SHUO_S.len(),
        shuo_bytes.len(),
        (1.0 - shuo_bytes.len() as f32 / shuo_decompressed.len() as f32) * 100.0
    );
    println!(
        "节气表: {}字符 -> {}字节 (压缩率: {:.1}%)",
        QI_S.len(),
        qi_bytes.len(),
        (1.0 - qi_bytes.len() as f32 / qi_decompressed.len() as f32) * 100.0
    );
    println!("总大小: {}字节", shuo_bytes.len() + qi_bytes.len());

    // 验证解压是否正确
    println!("\n验证解压:");
    println!("朔日表解压后长度: {shuo_len} (应为16599)");
    println!("节气表解压后长度: {qi_len} (应为7567)");
}
