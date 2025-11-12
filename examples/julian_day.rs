//! 儒略日计算示例

use sxtwl_rs::{JulianDay, SolarDate};

fn main() {
    // 创建公历日期
    let solar = SolarDate {
        year: 2024,
        month: 1,
        day: 1,
        hour: 12,
        minute: 0,
        second: 0.0,
    };

    // 公历转儒略日
    let jd: JulianDay = solar.into();
    // 使用SolarDate的Display实现
    println!("{}对应的儒略日：{}",
             solar,
             jd.0);

    // 儒略日转公历
    let solar2: SolarDate = jd.into();
    // 使用SolarDate的Display实现
    println!("儒略日 {} 对应的公历日期：{}",
             jd.0,
             solar2);

    // 儒略日的加减法
    let jd_tomorrow = jd + 1.0; // 加一天
    let solar_tomorrow: SolarDate = jd_tomorrow.into();
    // 使用SolarDate的Display实现
    println!("明天的日期：{}",
             solar_tomorrow);
}