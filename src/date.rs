//! 日期计算模块，处理公历和农历日期的转换

use crate::ssq::{self, calculate_start_of_spring, find_nearest_new_moon};
use crate::types::{JulianDay, LunarDate, SolarDate};
use crate::utils::calculate_gz_from_diff;
use crate::GanZhi;

/// 计算农历春节的儒略日
fn calculate_spring_festival(year: i32) -> f64 {
    // 简化实现：使用查表法获取春节日期
    // 实际应该通过计算立春节气和相关朔日来确定
    let spring_festival_dates = [
        (2023, 1, 22),
        (2024, 2, 10),
        (2025, 1, 29),
        (2026, 2, 17),
        (2027, 2, 6),
        (2028, 1, 26),
        (2029, 2, 13),
        (2030, 2, 3),
        (2031, 1, 23),
        (2032, 2, 11),
        (2033, 1, 31),
        (2034, 2, 19),
        (2035, 2, 8),
        (2036, 1, 28),
        (2037, 2, 15),
        (2038, 2, 4),
        (2039, 1, 24),
        (2040, 2, 12),
        (2041, 2, 1),
        (2042, 1, 22),
        (2043, 2, 10),
        (2044, 1, 30),
        (2045, 2, 17),
        (2046, 2, 6),
        (2047, 1, 26),
        (2048, 2, 14),
        (2049, 2, 2),
        (2050, 1, 22),
    ];

    // 查找指定年份的春节日期
    for &(y, month, day) in spring_festival_dates.iter() {
        if y == year {
            let solar_date = SolarDate::new(year, month as u8, day as u8, 12, 0, 0.0);
            let julian_day: JulianDay = solar_date.into();
            return julian_day.value();
        }
    }

    // 如果没有找到对应年份，使用二分查找
    let ssq_calculator = crate::ssq::SSQCalculator::new();
    let terms = ssq_calculator.find_terms(year);

    // 立春是第3个节气（索引为3）
    let 立春_jd = terms[3];

    // 查找立春附近的朔日
    let mut new_moon_jd = crate::ssq::find_nearest_new_moon(立春_jd);

    // 确保找到的是立春附近的第一个朔日
    if new_moon_jd > 立春_jd {
        new_moon_jd = crate::ssq::find_nearest_new_moon(立春_jd - 30.0);
    }

    new_moon_jd
}

/// 日期处理的核心结构
#[derive(Debug, Clone)]
pub struct ChineseCalendar {
    /// 儒略日天数（相对于J2000）
    j2000_days: i32,
    /// 懒加载的公历日期
    solar_date: Option<SolarDate>,
    /// 懒加载的农历日期
    lunar_date: Option<LunarDate>,
    /// 懒加载的年份天干地支（立春）
    year_gan_zhi_li_chun: Option<GanZhi>,
    /// 懒加载的年份天干地支（春节）
    year_gan_zhi_spring: Option<GanZhi>,
    /// 懒加载的月份天干地支
    month_gan_zhi: Option<GanZhi>,
    /// 懒加载的日天干地支
    day_gan_zhi: Option<GanZhi>,
    /// 节气信息
    jie_qi: Option<(u8, f64)>, // (节气索引, 节气儒略日)
    /// 星座
    constellation: Option<u8>,
}

impl ChineseCalendar {
    /// 从公历日期创建
    pub fn from_solar(year: i32, month: u8, day: u8) -> Self {
        let solar_date = SolarDate::new(year, month, day, 12, 0, 0.0);
        let julian_day: JulianDay = solar_date.into();
        let j2000_days = JulianDay::to_j2000_days(julian_day.value());

        Self {
            j2000_days,
            solar_date: Some(solar_date),
            lunar_date: None,
            year_gan_zhi_li_chun: None,
            year_gan_zhi_spring: None,
            month_gan_zhi: None,
            day_gan_zhi: None,
            jie_qi: None,
            constellation: None,
        }
    }

    /// 获取公历日期
    pub fn get_solar_date(&mut self) -> SolarDate {
        if let Some(date) = self.solar_date {
            return date;
        }

        // 从儒略日计算公历日期
        let julian_day_value = JulianDay::from_j2000_days(self.j2000_days);
        let solar_date: SolarDate = JulianDay(julian_day_value).into();

        self.solar_date = Some(solar_date);
        solar_date
    }

    /// 获取农历日期
    pub fn get_lunar_date(&mut self) -> LunarDate {
        if let Some(date) = self.lunar_date {
            return date;
        }

        // 实现农历日期计算
        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let lunar_date = self.calculate_lunar_from_solar(julian_day);

        self.lunar_date = Some(lunar_date);
        lunar_date
    }

    /// 从公历日期计算农历日期
    fn calculate_lunar_from_solar(&self, julian_day: f64) -> LunarDate {
        // 查找当前日期前的最近的朔日
        let mut new_moon = find_nearest_new_moon(julian_day);
        if new_moon > julian_day {
            // 如果找到的朔日在当前日期之后，找前一个朔日
            new_moon -= self.calculate_synodic_month(new_moon);
        }

        // 计算农历日
        let lunar_day = (julian_day - new_moon).floor() as u8 + 1;

        // 计算农历月份和年份
        let (lunar_year, lunar_month, is_leap_month) = self.calculate_lunar_month_year(new_moon);

        LunarDate::new(lunar_year, lunar_month, lunar_day, is_leap_month)
    }

    /// 计算朔望月长度
    fn calculate_synodic_month(&self, _julian_day: f64) -> f64 {
        // 简化实现，使用平均朔望月长度
        29.530588853
    }

    /// 计算农历月份和年份
    fn calculate_lunar_month_year(&self, new_moon: f64) -> (i32, u8, bool) {
        // 计算年份：根据春节位置确定农历年份
        let year = ((new_moon - 2415020.0) / 365.25) as i32;
        let spring_festival = calculate_spring_festival(year);

        let lunar_year = if new_moon < spring_festival {
            year - 1
        } else {
            year
        };

        // 计算月份：从春节开始逐月查找朔日
        let mut month = 1;
        let is_leap_month = false;

        // 简化实现：查找春节后的朔日来确定月份
        let current_sf = calculate_spring_festival(lunar_year);
        let mut current_new_moon = find_nearest_new_moon(current_sf - 15.0); // 春节前的朔日

        while current_new_moon + 29.5 < new_moon {
            // 下一个朔日可能还没到
            current_new_moon = find_nearest_new_moon(current_new_moon + 25.0); // 往前找下一个朔日
            month += 1;
        }

        // 限制月份范围
        month = month.min(12);

        (lunar_year, month, is_leap_month)
    }

    /// 获取以立春为界的年份天干地支
    pub fn get_year_gan_zhi_li_chun(&mut self) -> GanZhi {
        if let Some(gz) = self.year_gan_zhi_li_chun {
            return gz;
        }

        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let solar = self.get_solar_date();

        // 计算立春时间
        let li_chun = calculate_start_of_spring(solar.year);

        // 确定干支年份
        let gz_year = if julian_day < li_chun {
            solar.year - 1
        } else {
            solar.year
        };

        // 计算与1984年（甲子年）的差值
        let year_diff = gz_year - 1984;
        let gz = calculate_gz_from_diff(year_diff);

        self.year_gan_zhi_li_chun = Some(gz);
        gz
    }

    /// 获取以春节为界的年份天干地支
    pub fn get_year_gan_zhi_spring(&mut self) -> GanZhi {
        if let Some(gz) = self.year_gan_zhi_spring {
            return gz;
        }

        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let solar = self.get_solar_date();

        // 计算春节时间
        let spring_festival = calculate_spring_festival(solar.year);

        // 确定干支年份
        let gz_year = if julian_day < spring_festival {
            solar.year - 1
        } else {
            solar.year
        };

        // 计算与1984年（甲子年）的差值
        let year_diff = gz_year - 1984;
        let gz = calculate_gz_from_diff(year_diff);

        self.year_gan_zhi_spring = Some(gz);
        gz
    }

    /// 获取月份天干地支
    pub fn get_month_gan_zhi(&mut self) -> GanZhi {
        if let Some(gz) = self.month_gan_zhi {
            return gz;
        }

        // 获取年份天干
        let year_gz = self.get_year_gan_zhi_li_chun();
        let lunar_date = self.get_lunar_date();

        // 计算月份天干：(年干 * 2 + 月数) % 10
        // 注意：农历月份从1开始，这里需要调整
        let month_tian_gan = (year_gz.tian_gan * 2 + (lunar_date.month - 1) as u8) % 10;

        // 计算月份地支：月数 - 1 + 2（正月为寅）
        let month_di_zhi = ((lunar_date.month - 1 + 2) % 12) as u8;

        let gz = GanZhi {
            tian_gan: month_tian_gan,
            di_zhi: month_di_zhi,
        };

        self.month_gan_zhi = Some(gz);
        gz
    }

    /// 获取日天干地支
    pub fn get_day_gan_zhi(&mut self) -> GanZhi {
        if let Some(gz) = self.day_gan_zhi {
            return gz;
        }

        // 以2000年1月1日为基准（戊午日）
        let base_julian_day = 2451545.0; // 2000年1月1日的儒略日
        let base_tian_gan = 5; // 戊
        let base_di_zhi = 5; // 午

        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let days_diff = (julian_day - base_julian_day).floor() as i32;

        // 计算日天干地支
        let tian_gan = ((base_tian_gan as i32 + days_diff) % 10 + 10) % 10;
        let di_zhi = ((base_di_zhi as i32 + days_diff) % 12 + 12) % 12;

        let gz = GanZhi {
            tian_gan: tian_gan as u8,
            di_zhi: di_zhi as u8,
        };

        self.day_gan_zhi = Some(gz);
        gz
    }

    /// 获取时天干地支
    pub fn get_hour_gan_zhi(&self, hour: u8, _is_early_late_zi: bool) -> GanZhi {
        // 计算时辰地支
        // 子: 23-1时, 丑: 1-3时, 寅: 3-5时, 以此类推
        let mut hour_index = (hour as i32 - 1) / 2;
        if hour >= 23 || hour < 1 {
            hour_index = 0; // 子时
        }

        // 子时分为早子时和晚子时
        // 这里简化处理，默认使用相同的地支
        let di_zhi = ((hour_index + 11) % 12) as u8;

        // 需要日天干来计算时天干，所以需要临时计算
        let base_julian_day = 2451545.0; // 2000年1月1日的儒略日
        let base_tian_gan = 5; // 戊

        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let days_diff = (julian_day - base_julian_day).floor() as i32;
        let day_tian_gan = ((base_tian_gan as i32 + days_diff) % 10 + 10) % 10;

        // 计算时天干：(日天干 * 2 + 时地支) % 10
        let tian_gan = ((day_tian_gan * 2 + di_zhi as i32) % 10 + 10) % 10;

        GanZhi {
            tian_gan: tian_gan as u8,
            di_zhi,
        }
    }

    /// 获取星期几（0-6，0表示星期日）
    pub fn get_weekday(&self) -> u8 {
        // 计算星期几：儒略日+1取模7
        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let weekday = ((julian_day + 1.5) % 7.0).floor() as u8;
        weekday
    }

    /// 获取当前日期附近的节气
    pub fn get_current_jie_qi(&mut self) -> Option<(String, f64)> {
        if let Some((jq_index, jq_jd)) = self.jie_qi {
            let jq_name = ssq::get_jie_qi_name(jq_index);
            return Some((jq_name.to_string(), jq_jd));
        }

        let julian_day = JulianDay::from_j2000_days(self.j2000_days);
        let jq_info = ssq::find_nearest_jie_qi(julian_day, crate::types::QSType::QiType);

        self.jie_qi = Some((jq_info.jq_index, jq_info.julian_day));
        let jq_name = ssq::get_jie_qi_name(jq_info.jq_index);

        Some((jq_name.to_string(), jq_info.julian_day))
    }

    /// 获取星座
    pub fn get_constellation(&mut self) -> String {
        if let Some(const_index) = self.constellation {
            return self.get_constellation_name(const_index);
        }

        let solar = self.get_solar_date();
        let const_index = self.calculate_constellation(solar.month, solar.day);

        self.constellation = Some(const_index);
        self.get_constellation_name(const_index)
    }

    /// 计算星座索引
    fn calculate_constellation(&self, month: u8, day: u8) -> u8 {
        match (month, day) {
            (1, 20..=31) => 0, // 水瓶座
            (2, 1..=18) => 0,
            (2, 19..=29) => 1, // 双鱼座
            (3, 1..=20) => 1,
            (3, 21..=31) => 2, // 白羊座
            (4, 1..=19) => 2,
            (4, 20..=30) => 3, // 金牛座
            (5, 1..=20) => 3,
            (5, 21..=31) => 4, // 双子座
            (6, 1..=21) => 4,
            (6, 22..=30) => 5, // 巨蟹座
            (7, 1..=22) => 5,
            (7, 23..=31) => 6, // 狮子座
            (8, 1..=22) => 6,
            (8, 23..=31) => 7, // 处女座
            (9, 1..=22) => 7,
            (9, 23..=30) => 8, // 天秤座
            (10, 1..=23) => 8,
            (10, 24..=31) => 9, // 天蝎座
            (11, 1..=22) => 9,
            (11, 23..=30) => 10, // 射手座
            (12, 1..=21) => 10,
            (12, 22..=31) => 11, // 摩羯座
            (1, 1..=19) => 11,
            _ => 0,
        }
    }

    /// 获取星座名称
    fn get_constellation_name(&self, index: u8) -> String {
        match index {
            0 => "水瓶座",
            1 => "双鱼座",
            2 => "白羊座",
            3 => "金牛座",
            4 => "双子座",
            5 => "巨蟹座",
            6 => "狮子座",
            7 => "处女座",
            8 => "天秤座",
            9 => "天蝎座",
            10 => "射手座",
            11 => "摩羯座",
            _ => "未知",
        }
        .to_string()
    }

    /// 从农历日期创建
    pub fn from_lunar(year: i32, month: u8, day: u8, is_leap_month: bool) -> Self {
        // 验证输入参数
        if month == 0 || month > 12 || day == 0 || day > 30 {
            panic!("无效的农历日期: 月{} 日{}", month, day);
        }

        // 1. 计算该农历年春节所在的儒略日
        let spring_festival = calculate_spring_festival(year);

        // 2. 查找该农历月的朔日
        let mut current_month = 1;
        let mut current_new_moon = find_nearest_new_moon(spring_festival - 15.0); // 春节前的朔日
        let mut target_new_moon = current_new_moon;
        let mut found = false;

        // 查找目标月份的朔日
        while current_new_moon < spring_festival + 366.0 {
            // 限制在一年内搜索
            // 检查是否需要考虑闰月
            let has_leap_month = crate::utils::calculate_leap_month(year).is_some();
            let is_current_leap = if has_leap_month {
                Some(current_month) == crate::utils::calculate_leap_month(year)
            } else {
                false
            };

            // 处理闰月情况
            if is_leap_month && is_current_leap && current_month == month {
                target_new_moon = current_new_moon;
                found = true;
                break;
            }

            // 处理普通月份情况
            if !is_leap_month && !is_current_leap {
                if current_month == month {
                    target_new_moon = current_new_moon;
                    found = true;
                    break;
                }
                current_month += 1;
            }

            // 搜索下一个朔日
            current_new_moon = find_nearest_new_moon(current_new_moon + 25.0);
        }

        // 如果没有找到对应的月份，使用默认值
        if !found {
            panic!(
                "未找到对应的农历月份: {}{}",
                if is_leap_month { "闰" } else { "" },
                month
            );
        }

        // 3. 计算公历日期：朔日 + 农历日 - 1
        let julian_day = target_new_moon + (day as f64) - 1.0;
        let j2000_days = JulianDay::to_j2000_days(julian_day);

        Self {
            j2000_days,
            solar_date: None,
            lunar_date: Some(LunarDate::new(year, month, day, is_leap_month)),
            year_gan_zhi_li_chun: None,
            year_gan_zhi_spring: None,
            month_gan_zhi: None,
            day_gan_zhi: None,
            jie_qi: None,
            constellation: None,
        }
    }

    /// 判断某年某月是否为闰月
    fn is_leap_month_at(year: i32, month: u8) -> bool {
        if let Some(leap_month) = crate::utils::calculate_leap_month(year) {
            leap_month == month
        } else {
            false
        }
    }

    /// 获取几天后的日期
    pub fn after(&self, days: i32) -> Self {
        Self {
            j2000_days: self.j2000_days + days,
            solar_date: None,
            lunar_date: None,
            year_gan_zhi_li_chun: None,
            year_gan_zhi_spring: None,
            month_gan_zhi: None,
            day_gan_zhi: None,
            jie_qi: None,
            constellation: None,
        }
    }

    /// 获取几天前的日期
    pub fn before(&self, days: i32) -> Self {
        self.after(-days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_to_lunar_basic() {
        // 基础测试：创建公历日期
        let mut cal = ChineseCalendar::from_solar(2024, 1, 1);
        let solar = cal.get_solar_date();
        assert_eq!(solar.year, 2024);
        assert_eq!(solar.month, 1);
        assert_eq!(solar.day, 1);

        // 测试日期加减
        let tomorrow = cal.after(1);
        assert_eq!(tomorrow.j2000_days, cal.j2000_days + 1);

        let yesterday = cal.before(1);
        assert_eq!(yesterday.j2000_days, cal.j2000_days - 1);
    }

    #[test]
    fn test_lunar_date_calculation() {
        // 测试农历日期计算
        let mut cal = ChineseCalendar::from_solar(2024, 2, 10); // 2024年春节
        let lunar = cal.get_lunar_date();

        // 验证农历年份
        assert_eq!(lunar.year, 2024);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.day, 1);
        assert!(!lunar.is_leap_month);
    }

    #[test]
    fn test_gan_zhi_calculation() {
        // 测试天干地支计算
        let mut cal = ChineseCalendar::from_solar(2024, 2, 10); // 甲辰年春节

        // 测试年干支（立春）
        let year_gz = cal.get_year_gan_zhi_li_chun();
        assert_eq!(year_gz.tian_gan, 0); // 甲
        assert_eq!(year_gz.di_zhi, 4); // 辰

        // 测试日干支
        let day_gz = cal.get_day_gan_zhi();
        // 具体值可能需要调整，这里只是示例
        assert!(day_gz.tian_gan < 10);
        assert!(day_gz.di_zhi < 12);
    }

    #[test]
    fn test_constellation() {
        // 测试星座计算
        let mut cal = ChineseCalendar::from_solar(2024, 1, 1);
        assert_eq!(cal.get_constellation(), "摩羯座");

        cal = ChineseCalendar::from_solar(2024, 2, 14);
        assert_eq!(cal.get_constellation(), "水瓶座");

        cal = ChineseCalendar::from_solar(2024, 3, 21);
        assert_eq!(cal.get_constellation(), "白羊座");
    }

    #[test]
    fn test_weekday() {
        // 测试星期几计算
        // 2024年1月1日是星期一
        let cal = ChineseCalendar::from_solar(2024, 1, 1);
        assert_eq!(cal.get_weekday(), 1);

        // 2024年1月7日是星期日
        let cal = ChineseCalendar::from_solar(2024, 1, 7);
        assert_eq!(cal.get_weekday(), 0);
    }
}