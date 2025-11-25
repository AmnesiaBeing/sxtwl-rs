use std::fs::File;
use std::io::Write;
use std::path::Path;

mod original_qishuo_strings;
use original_qishuo_strings::{QI_S, SHUO_S};

mod original_holiday_strings;
use original_holiday_strings::LEGAL_HOLIDAY_DATA;

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
    writeln!(&mut f, "// 自动生成的压缩数据 - 请勿手动修改").unwrap();
    writeln!(&mut f, "// 此文件由 build.rs 自动生成").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "// 每个字符用2位存储: 00=0, 01=1, 10=2").unwrap();
    writeln!(&mut f, "pub const SHUO_BYTES: &[u8] = &{shuo_bytes:?};").unwrap();
    writeln!(&mut f, "pub const SHUO_LEN: usize = {shuo_len};").unwrap();
    writeln!(&mut f, "pub const QI_BYTES: &[u8] = &{qi_bytes:?};").unwrap();
    writeln!(&mut f, "pub const QI_LEN: usize = {qi_len};").unwrap();
    writeln!(&mut f).unwrap();

    writeln!(&mut f, "/// 从2位压缩数据中获取指定索引的值 (0, 1, 或 2)").unwrap();
    writeln!(&mut f, "pub fn get_shuo_value(index: usize) -> u8 {{").unwrap();
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
    writeln!(&mut f, "pub fn get_qi_value(index: usize) -> u8 {{").unwrap();
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

    ///

    // 1. 定义原始数据和字符集
    const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_@";
    const MONTHS: [&str; 12] = [
        "080b0r0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C0F0j0V0V0V0u0j0j0C0j0j0j0j0V0C0j1v0u0C0V1v0C0b080u110u0C0j0C1v9K1v2z0j1vmZbl1veN3s1v0V0C2S1v0V0C2S2o0C0j1Z1c2S1v0j1c0j2z1v0j1c0j392H0b2_2S0C0V0j1c0j2z0C0C0j0j1c0j0N250j0C0j0b081n080b0C0C0C1c0j0N",
        "0r1v1c1v0V0V0F0V0j0C0j0C0j0V0j0u1O0j0C0V0j0j0j0V0b080u0r0u080b0j0j0C0V0C0V0j0b080V0u080b0j0j0u0j1v0u080b1c0j080b0j0V0j0j0V0C0N1v0j1c0j0j1v2g1v420j1c0j2z1v0j1v5Q9z1v4l0j1vfn1v420j9z4l1v1v2S1c0j1v2S3s1v0V0C2S1v1v2S1c0j1v2S2_0b0j2_2z0j1c0j",
        "0z0j0j0j0C0j0j0C0j0j0j0C0j0C0j0j0j0j0m0j0C0j0j0C0j0j0j0j0b0V0j0j0C0j0j0j0j0V0j0j0j0V0b0V0V0C0V0C0j0j0b080u110u0V0C0j0N0j0b080b080b0j0r0b0r0b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b110j0b0j0j0j0C0j0C0j0j0j0j0b080b080b0V080b080b0j0j0j0j0j0j0V0j0j0u1v0j0j0j0C0j0j0j0V0C0N1c0j0C0C0j0j0j1n080b0j0V0C0j0C0C2g0j1c0j0j1v2g1v0j0j1v7N0j1c0j3L0j0j1v5Q1Z5Q1v4lfn1v420j1v5Q1Z5Q1v4l1v2z1v",
        "0H140r0N0r140r0u0r0V171c11140C0j0u110j0u0j1v0j0C0j0j0j0b080V0u080b0C1v0j0j0j0C0j0b080V0j0j0b080b0j0j0j0j0b080b0C080j0b080b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080C0b0j0j0j0j0j0j0b08080b0j0C0j0j0j0b0j0j0K0b0j0C0j0j0j0b080b080j0C0b0j080b080b0j0j0j0j080b0j0b0r0j0j0j0b0j0C0r0b0j0j0j0j0j0j0j0b080j0b0r0C0j0b0j0j0j0r0b0j0C0j0j0j0u0r0b0C0j080b0j0j0j0j0j0j0j1c0j0b0j0j0j0C0j0j0j0j0j0j0j0b080j1c0u0j0j0j0C0j1c0j0u0j1c0j0j0j0j0j0j0j0j1c0j0u1v0j0j0V0j0j2g0j0j0j0C1v0C1G0j0j0V0C1Z1O0j0V0j0j2g1v0j0j0V0C2g5x1v4l1v421O7N0V0C4l1v2S1c0j1v2S2_",
        "050b080C0j0j0j0C0j0j0C0j0j0j0C0j0C0j0C030j0j0j0j0j0j0j0j0j0C0j0b080u0V080b0j0j0V0j0j0j0j0j0j0j0j0j0V0N0j0C0C0j0j0j0j0j0j0j0j1c0j0u0j1v0j0j0j0j0j0b080b080j0j0j0b080b080b080b080b0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0b080j0j0j0j0b080b080j0b080j0b080b080b080b0j0j0r0b0j0b080j0j0j0j0b080b0j0j0C080b0b080j0j0j0j0j0j0j0b080u080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080b080b0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0C0b080b080j0b0j0j0C080b0j0j0j0j0j0j0b0j0j080C0b0j080b0j0j0j0j0j0j0j0C0j0j0j0b0j0j0C080b0j0j0j0j0j0j0b080b080b0K0b080b080b0j0j0j0j0j0j0j0C0j0j0u0j0j0V0j080b0j0C0j0j0j0b0j0r0C0b0j0j0j0j0j0j0j0j0j0C0j0b080b080b0j0C0C0j0C0j0j0j0u110u0j0j0j0j0j0j0j0j0C0j0j0u0j1c0j0j0j0j0j0j0j0j0V0C0u0j0C0C0V0C1Z0j0j0j0C0j0j0j1v0u0j1c0j0j0j0C0j0j2g0j1c1v0C1Z0V0j4l0j0V0j0j2g0j1v0j1v2S1c7N1v",
        "0w0j1c0j0V0j0j0V0V0V0j0m0V0j0C1c140j0j0j0C0V0C0j1v0j0N0j0C0j0j0j0V0j0j1v0N0j0j0V0j0j0j0j0j0j080b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080C0b0j080b080b0j0j0j0j080b0j0C0j0j0j0b0j0j080u080b0j0j0j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j080b0j0b0r0j0j0b080b0j0j080b0j080b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j080b0b080b080j0j0j0b080b080b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j0j0r0b080b0j0j0j0j0j0b080b080j0b0r0b080j0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b0j080b0r0b0j080b080b0j0j0j0j0j0b080b0r0C0b080b0j0j0j0j0j0j0b080j0j0j0b080b080b080b0j0j0j0r0b0j0b080j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j080b0j0C0j0j0j0b0j0C0r0b0j0j0j0j0j0j0b080b080u0r0b0j080b0j0j0j0j0j0j0j0b0r0C0u0j0j0j0C0j080b0j0C0j0j0j0u110b0j0j0j0j0j0j0j0j0j0C0j0b080b0j0j0C0C0j0C0j0j0j0b0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j1c0j0j0j0C0j0j2g0j0j0j0C0j0j0V0j0b080b1c0C0V0j0j2g0j0j0V0j0j1c0j1Z0j0j0C0C0j1v",
        "160j0j0V0j1c0j0C0j0C0j1f0j0V0C0j0j0C0j0j0j1G080b080u0V080b0j0j0V0j1v0j0u0j1c0j0j0j0C0j0j0j0C0C0j1D0b0j080b0j0j0j0j0C0j0b0r0C0j0b0j0C0C0j0j0j0j0j0j0j0j0j0b0r0b0r0j0b0j0j0j0C0j0b0r0j0j0j0b080b080j0b0C0j080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0j0j0j0b080b080j0b0C0r0j0b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0j0j0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080b080b080C0b080b080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b0C080j0j0b0j0j0j0j0j0j0b080j0b080C0b080b080b080b0j0j0j0j080b0j0C0j0j0b080b0j0j0C080b0j0j0j0j0j0j0b080j0b080u080j0j0b0j0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b080u080b080b080b0j0j0j0C0j0b080j0j0j0j0b0j0j0j0C0j0j080b0j0j0j0j0j0b080b0C0r0b080b0j0j0j0j0j0j0b080j0b0r0b080b080b080b0j0j0j0r0b0j0b0r0j0j0j0b0j0j0j0r0b0j080b0j0j0j0j0j0j0j0b0r0C0b0j0j0j0j0j0j0j080b0j0C0u080b080b0j0j0j0r0b0j0C0C0j0b0j110b0j080b0j0j0j0j0j0j0u0r0C0b0j0j0j0j0j0j0j0j0j0C0j0j0j0b0j1c0j0C0j0j0j0b0j0814080b080b0j0j0j0j0j0j1c0j0u0j0j0V0j0j0j0j0j0j0j0u110u0j0j0j",
        "020b0r0C0j0j0j0C0j0j0V0j0j0j0j0j0C0j1f0j0C0j0V1G0j0j0j0j0V0C0j0C1v0u0j0j0j0V0j0j0C0j0j0j1v0N0C0V0j0j0j0K0C250b0C0V0j0j0V0j0j2g0C0V0j0j0C0j0j0b081v0N0j0j0V0V0j0j0u0j1c0j080b0j0j0j0j0j0j0V0j0j0u0j0j0V0j0j0j0C0j0b080b080V0b0j080b0j0j0j0j0j0j0j0b0r0C0j0b0j0j0j0C0j080b0j0j0j0j0j0j0u0r0C0u0j0j0j0j0j0j0b080j0C0j0b080b080b0j0C0j080b0j0j0j0j0j0j0b080b110b0j0j0j0j0j0j0j0j0j0b0r0j0j0j0b0j0j0j0r0b0j0b080j0j0j0j0b080b080b080b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0b080b0j0j0j0j080b0j0b080j0j0j0j0b080b080b0j0j0j0r0b0j0j0j0j0j0j0b080b0j080C0b0j080b080b0j0j0j0j080b0j0b0r0C0b080b0j0j0j0j080b0j0j0j0j0j0b080b080b080b0j0j080b0r0b0j0j0j0j0j0j0b0j0j080C0b0j080b080b0j0j0j0j0j0b080C0j0j0b080b0j0j0C0j0b080j0j0j0j0b080b080b080b0C0C080b0j0j0j0j0j0j0b0C0C080b080b080b0j0j0j0j0j0j0b0C080j0j0b0j0j0j0C0j0b080j0b080j0j0b080b080b080b0C0r0b0j0j0j0j0j0j0b080b0r0b0r0b0j080b080b0j0j0j0j0j0j0b0r0C0j0b0j0j0j0j0j0j0b080j0C0j0b080j0b0j0j0K0b0j0C0j0j0j0b080b0j0K0b0j080b0j0j0j0j0j0j0V0j0j0b0j0j0j0C0j0j0j0j",
        "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_1Z0V2o141v0j0C0C0j2z1v0j1c0j7N1O420j1c0j1v2S1c0j1v2S2_0b0j0V0j0j1v0N1v0j0j1c0j1v140j0V0j0j0C0C0b080u1v0C0V0u110u0j0j0j0C0j0j0j0C0C0N0C0V0j0j0C0j0j0b080u110u0C0j0C0u0r0C0u080b0j0j0C0j0j0j",
    ];

    // 2. 解析数据
    let mut leap_month_data = Vec::new();
    let mut max_days_in_month = 0;

    for &month_str in MONTHS.iter() {
        let mut month_values = Vec::new();
        let mut n: isize = 0;

        // 检查字符串长度是否为偶数
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

    // 3. 生成 Rust 代码
    let dest_path = Path::new("src").join("generated_leap_year_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    // 3.1. 生成一个巨大的二维数组
    // 为了简单起见，我们使用一个 Vec<Vec<isize>> 并提供访问函数
    // 如果所有月份的天数都相同，可以生成 [[isize; N]; 12]
    writeln!(f, "//! 此文件由 build.rs 自动生成，不要手动修改。").unwrap();
    writeln!(f, "//! 包含了预计算的闰月查找表数据。").unwrap();
    writeln!(f).unwrap();

    writeln!(f, "use core::option::Option;").unwrap();
    writeln!(f).unwrap();

    writeln!(f, "/// 预计算的闰月数据，格式为 [月份][日期] -> 累计值").unwrap();
    writeln!(f, "/// 注意：并非所有月份都有相同的天数。").unwrap();
    writeln!(f, "pub static LEAP_MONTH_YEAR_DATA: &[&[isize]] = &[").unwrap();
    for month_values in &leap_month_data {
        writeln!(f, "    &{:?},", month_values).unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // 3.2. 生成一个方便的查询接口
    writeln!(
        f,
        "/// 根据年份和月份获取累计值。\n/// 索引从 0 开始。\n/// 如果索引超出范围，返回 None。"
    )
    .unwrap();
    writeln!(
        f,
        "pub fn get_leap_month_value(month: usize, day: usize) -> Option<isize> {{"
    )
    .unwrap();
    writeln!(f, "    if month >= LEAP_MONTH_YEAR_DATA.len() {{").unwrap();
    writeln!(f, "        return None;").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "    let month_data = LEAP_MONTH_YEAR_DATA[month];").unwrap();
    writeln!(f, "    if day >= month_data.len() {{").unwrap();
    writeln!(f, "        return None;").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "    Some(month_data[day])").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f).unwrap();

    // 3.3. 生成一个判断是否为闰月的辅助函数（示例）
    // 这个逻辑是假设性的，你需要根据你的实际数据含义来调整
    writeln!(
        f,
        "/// 判断某一年的某个月是否为闰月（示例逻辑）。\n/// 如果 `get_leap_month_value` 返回 Some(_)，则认为是闰月。"
    )
    .unwrap();
    writeln!(f, "pub fn is_leap_month(month: usize) -> bool {{").unwrap();
    writeln!(
        f,
        "    month < LEAP_MONTH_YEAR_DATA.len() && !LEAP_MONTH_YEAR_DATA[month].is_empty()"
    )
    .unwrap();
    writeln!(f, "}}").unwrap();

    // 节假日
    let dest_path = Path::new("src").join("generated_holidays_data.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "// 自动生成的法定节假日数据").unwrap();
    writeln!(f).unwrap();

    writeln!(f, "/// 法定节假日条目").unwrap();
    writeln!(f, "#[derive(Debug, Clone, Copy)]").unwrap();
    writeln!(f, "pub struct LegalHolidayEntry {{").unwrap();
    writeln!(f, "    pub year: u16,").unwrap();
    writeln!(f, "    pub month: u8,").unwrap();
    writeln!(f, "    pub day: u8,").unwrap();
    writeln!(f, "    pub work: bool,").unwrap();
    writeln!(f, "    pub index: u8,").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f, "").unwrap();

    writeln!(
        f,
        "pub const LEGAL_HOLIDAY_TABLE: [LegalHolidayEntry; {}] = [",
        LEGAL_HOLIDAY_DATA.len() / 12
    )
    .unwrap();

    let record_count = LEGAL_HOLIDAY_DATA.len() / 12;

    // 每12个字符一条记录，格式为：YYYYMMDDWIXY
    // YYYY=年份, MM=月份, DD=日期, W=是否上班(0=休,1=班), I=节日索引, X=符号(+/-), Y=两位数字(可能是调整天数)
    for i in 0..record_count {
        let start = i * 12;
        let end = start + 12;
        let record = &LEGAL_HOLIDAY_DATA[start..end];

        let year = &record[0..4];
        let month = &record[4..6];
        let day = &record[6..8];
        let work_char = &record[8..9];
        let index_char = &record[9..10];
        // 最后两位暂时忽略

        let work = work_char == "0"; // '0'表示休息，'1'表示上班
        let index = index_char.parse::<u8>().unwrap();

        writeln!(
            f,
            "    LegalHolidayEntry {{ year: {}, month: {}, day: {}, work: {}, index: {} }},",
            year, month, day, work, index
        )
        .unwrap();
    }

    writeln!(f, "];").unwrap();
}
