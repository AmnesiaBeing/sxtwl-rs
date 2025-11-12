//! 基本用法示例：公历与农历的相互转换

use sxtwl_rs::{LunarDate, SolarDate};

fn main() {
    // 创建公历日期：2024年1月1日
    let solar = SolarDate {
        year: 2024,
        month: 1,
        day: 1,
        hour: 12,
        minute: 0,
        second: 0.0,
    };

    // 公历转农历
    let lunar: LunarDate = solar.into();
    // 使用SolarDate的Display实现和LunarDate的格式化方法
    println!(
        "公历 {} -> 农历 {}{}{}{}",
        solar.format_ymd(),
        lunar.year_to_chinese(),
        if lunar.is_leap_month { "闰" } else { "" },
        lunar.month_to_chinese(),
        lunar.day_to_chinese()
    );

    // 创建农历日期：2023年腊月初一（非闰月）
    let lunar = LunarDate {
        year: 2023,
        month: 12,
        day: 1,
        is_leap_month: false,
    };

    // 农历转公历
    let solar: SolarDate = lunar.into();
    // 使用SolarDate的Display实现和LunarDate的格式化方法
    println!(
        "农历 {}{}{}{} -> 公历 {}",
        lunar.year_to_chinese(),
        if lunar.is_leap_month { "闰" } else { "" },
        lunar.month_to_chinese(),
        lunar.day_to_chinese(),
        solar.format_ymd()
    );
}
