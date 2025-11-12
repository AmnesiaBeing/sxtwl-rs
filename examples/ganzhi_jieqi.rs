//! 天干地支与节气示例

use sxtwl_rs::{GanZhi, JieQi, LunarDate, ShengXiao, SolarDate};

fn main() {
    // 创建公历日期
    let solar = SolarDate {
        year: 2024,
        month: 2,
        day: 10,
        hour: 12,
        minute: 0,
        second: 0.0,
    };

    // 获取农历日期
    let lunar: LunarDate = solar.into();

    // 计算生肖
    let shengxiao = ShengXiao::from_lunar_year(lunar.year);
    println!("{}年生肖：{}", lunar.year, shengxiao.as_str());

    // 计算年干支 - 使用GanZhi的Display实现
    let year_ganzhi = GanZhi::from_lunar_year(lunar.year);
    println!("{}年干支：{}", lunar.year, year_ganzhi);

    // 获取农历月份和日期的中文表示
    println!(
        "农历日期：{}{}{}{}",
        lunar.year_to_chinese(),
        if lunar.is_leap_month { "闰" } else { "" },
        lunar.month_to_chinese(),
        lunar.day_to_chinese()
    );

    // 获取2024年的所有节气
    println!("\n2024年节气列表：");
    let jieqis = JieQi::get_all_jieqi_by_solar_year(2024);
    for jieqi in jieqis {
        let jieqi_date: SolarDate = jieqi.jd.into();
        println!("{}: {}", jieqi.jq_index.name(), jieqi_date.format_ymd());
    }
}
