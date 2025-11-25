use core::fmt::{Display, Formatter};

use alloc::string::{String, ToString};

use crate::enums::FestivalType;
use crate::lunar::LunarDay;
use crate::solar::{SolarDay, SolarTerm};
use crate::types::{AbstractCulture, Culture, Tyme};

#[rustfmt::skip]
pub static SOLAR_FESTIVAL_NAMES: [&str; 10] = ["元旦", "三八妇女节", "植树节", "五一劳动节", "五四青年节", "六一儿童节", "建党节", "八一建军节", "教师节", "国庆节"];

#[derive(Debug, Clone, Copy)]
pub struct SolarFestivalEntry {
    pub index: u8,
    pub festival_type: FestivalType,
    pub month: u8,
    pub day: u8,
    pub start_year: i16,
}

#[rustfmt::skip]
pub const SOLAR_FESTIVAL_TABLE: [SolarFestivalEntry; 10] = [
    SolarFestivalEntry { index: 0, festival_type: FestivalType::DAY, month: 1, day: 1, start_year: 1950 },
    SolarFestivalEntry { index: 1, festival_type: FestivalType::DAY, month: 3, day: 8, start_year: 1950 },
    SolarFestivalEntry { index: 2, festival_type: FestivalType::DAY, month: 3, day: 12, start_year: 1979 },
    SolarFestivalEntry { index: 3, festival_type: FestivalType::DAY, month: 5, day: 1, start_year: 1950 },
    SolarFestivalEntry { index: 4, festival_type: FestivalType::DAY, month: 5, day: 4, start_year: 1950 },
    SolarFestivalEntry { index: 5, festival_type: FestivalType::DAY, month: 6, day: 1, start_year: 1950 },
    SolarFestivalEntry { index: 6, festival_type: FestivalType::DAY, month: 7, day: 1, start_year: 1941 },
    SolarFestivalEntry { index: 7, festival_type: FestivalType::DAY, month: 8, day: 1, start_year: 1933 },
    SolarFestivalEntry { index: 8, festival_type: FestivalType::DAY, month: 9, day: 10, start_year: 1985 },
    SolarFestivalEntry { index: 9, festival_type: FestivalType::DAY, month: 10, day: 1, start_year: 1950 },
];

/// 公历现代节日
#[derive(Debug, Copy, Clone)]
pub struct SolarFestival {
    /// 类型
    festival_type: FestivalType,
    /// 公历日
    day: SolarDay,
    /// 索引
    index: usize,
    /// 起始年
    start_year: isize,
}

impl Culture for SolarFestival {
    fn get_name(&self) -> String {
        SOLAR_FESTIVAL_NAMES[self.index].to_string()
    }
}

impl SolarFestival {
    pub fn from_ymd(year: isize, month: usize, day: usize) -> Option<Self> {
        SOLAR_FESTIVAL_TABLE
            .iter()
            .find(|entry| {
                entry.month == month as u8
                    && entry.day == day as u8
                    && year >= entry.start_year as isize
            })
            .map(|entry| Self {
                festival_type: entry.festival_type.clone(),
                day: SolarDay::from_ymd(year, month, day),
                index: entry.index as usize,
                start_year: entry.start_year as isize,
            })
    }

    pub fn from_index(year: isize, index: usize) -> Option<Self> {
        SOLAR_FESTIVAL_TABLE
            .get(index)
            .filter(|entry| year >= entry.start_year as isize)
            .map(|entry| {
                let day = SolarDay::from_ymd(year, entry.month as usize, entry.day as usize);
                Self {
                    festival_type: entry.festival_type.clone(),
                    day,
                    index: entry.index as usize,
                    start_year: entry.start_year as isize,
                }
            })
    }

    pub fn get_type(&self) -> FestivalType {
        self.festival_type.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_day(&self) -> SolarDay {
        self.day
    }

    pub fn get_start_year(&self) -> isize {
        self.start_year
    }

    pub fn next(&self, n: isize) -> Option<Self> {
        let size: isize = SOLAR_FESTIVAL_NAMES.len() as isize;
        let i: isize = self.get_index() as isize + n;
        Self::from_index(
            (self.day.get_year() * size + i) / size,
            AbstractCulture::new().index_of(i, size as usize),
        )
    }
}

impl Display for SolarFestival {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.get_day(), self.get_name())
    }
}

impl PartialEq for SolarFestival {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for SolarFestival {}

#[rustfmt::skip]
pub static LUNAR_FESTIVAL_NAMES: [&str; 13] = ["春节", "元宵节", "龙头节", "上巳节", "清明节", "端午节", "七夕节", "中元节", "中秋节", "重阳节", "冬至节", "腊八节", "除夕"];

#[derive(Debug, Clone, Copy)]
pub enum LunarFestivalVariant {
    Fixed { month: i8, day: u8 }, // 固定日期
    SolarTerm { term_index: u8 }, // 节气相关
    NewYearEve,                   // 除夕
}

#[derive(Debug, Clone, Copy)]
pub struct LunarFestivalEntry {
    pub index: u8,
    pub variant: LunarFestivalVariant,
}

#[rustfmt::skip]
pub const LUNAR_FESTIVAL_TABLE: [LunarFestivalEntry; 13] = [
    LunarFestivalEntry { index: 0, variant: LunarFestivalVariant::Fixed { month: 1, day: 1 } },
    LunarFestivalEntry { index: 1, variant: LunarFestivalVariant::Fixed { month: 1, day: 15 } },
    LunarFestivalEntry { index: 2, variant: LunarFestivalVariant::Fixed { month: 2, day: 2 } },
    LunarFestivalEntry { index: 3, variant: LunarFestivalVariant::Fixed { month: 3, day: 3 } },
    LunarFestivalEntry { index: 4, variant: LunarFestivalVariant::SolarTerm { term_index: 7 } },  // 清明
    LunarFestivalEntry { index: 5, variant: LunarFestivalVariant::Fixed { month: 5, day: 5 } },
    LunarFestivalEntry { index: 6, variant: LunarFestivalVariant::Fixed { month: 7, day: 7 } },
    LunarFestivalEntry { index: 7, variant: LunarFestivalVariant::Fixed { month: 7, day: 15 } },
    LunarFestivalEntry { index: 8, variant: LunarFestivalVariant::Fixed { month: 8, day: 15 } },
    LunarFestivalEntry { index: 9, variant: LunarFestivalVariant::Fixed { month: 9, day: 9 } },
    LunarFestivalEntry { index: 10, variant: LunarFestivalVariant::SolarTerm { term_index: 24 } }, // 冬至
    LunarFestivalEntry { index: 11, variant: LunarFestivalVariant::Fixed { month: 12, day: 8 } },
    LunarFestivalEntry { index: 12, variant: LunarFestivalVariant::NewYearEve },
];

/// 农历传统节日（依据国家标准《农历的编算和颁行》GB/T 33661-2017）
#[derive(Debug, Clone)]
pub struct LunarFestival {
    /// 类型
    festival_type: FestivalType,
    /// 农历日
    day: LunarDay,
    /// 索引
    index: usize,
    /// 节气
    solar_term: Option<SolarTerm>,
}

impl Culture for LunarFestival {
    fn get_name(&self) -> String {
        LUNAR_FESTIVAL_NAMES[self.index].to_string()
    }
}

impl LunarFestival {
    pub fn from_ymd(year: isize, month: isize, day: usize) -> Option<Self> {
        for entry in &LUNAR_FESTIVAL_TABLE {
            match entry.variant {
                LunarFestivalVariant::Fixed { month: m, day: d } => {
                    if m == month as i8 && d == day as u8 {
                        let lunar_day = LunarDay::from_ymd(year, month, day);
                        return Some(Self {
                            festival_type: FestivalType::DAY,
                            day: lunar_day,
                            index: entry.index as usize,
                            solar_term: None,
                        });
                    }
                }
                LunarFestivalVariant::SolarTerm { term_index } => {
                    let solar_term = SolarTerm::from_index(year, term_index as isize);
                    let lunar_day = solar_term.get_solar_day().get_lunar_day();
                    if lunar_day.get_year() == year
                        && lunar_day.get_month() == month
                        && lunar_day.get_day() == day
                    {
                        return Some(Self {
                            festival_type: FestivalType::TERM,
                            day: lunar_day,
                            index: entry.index as usize,
                            solar_term: Some(solar_term),
                        });
                    }
                }
                LunarFestivalVariant::NewYearEve => {
                    let lunar_day = LunarDay::from_ymd(year, month, day);
                    let next_day = lunar_day.next(1);
                    if next_day.get_month() == 1 && next_day.get_day() == 1 {
                        return Some(Self {
                            festival_type: FestivalType::EVE,
                            day: lunar_day,
                            index: entry.index as usize,
                            solar_term: None,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn from_index(year: isize, index: usize) -> Option<Self> {
        LUNAR_FESTIVAL_TABLE
            .get(index)
            .and_then(|entry| match entry.variant {
                LunarFestivalVariant::Fixed { month, day } => {
                    let lunar_day = LunarDay::from_ymd(year, month as isize, day as usize);
                    Some(Self {
                        festival_type: FestivalType::DAY,
                        day: lunar_day,
                        index: entry.index as usize,
                        solar_term: None,
                    })
                }
                LunarFestivalVariant::SolarTerm { term_index } => {
                    let solar_term = SolarTerm::from_index(year, term_index as isize);
                    let lunar_day = solar_term.get_solar_day().get_lunar_day();
                    Some(Self {
                        festival_type: FestivalType::TERM,
                        day: lunar_day,
                        index: entry.index as usize,
                        solar_term: Some(solar_term),
                    })
                }
                LunarFestivalVariant::NewYearEve => {
                    // 除夕是农历年的最后一天
                    let lunar_day = LunarDay::from_ymd(year + 1, 1, 1).next(-1);
                    Some(Self {
                        festival_type: FestivalType::EVE,
                        day: lunar_day,
                        index: entry.index as usize,
                        solar_term: None,
                    })
                }
            })
    }

    pub fn get_type(&self) -> FestivalType {
        self.festival_type.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_day(&self) -> LunarDay {
        self.day.clone()
    }

    pub fn get_solar_term(&self) -> Option<SolarTerm> {
        self.solar_term.clone()
    }

    pub fn next(&self, n: isize) -> Option<Self> {
        let size: isize = LUNAR_FESTIVAL_NAMES.len() as isize;
        let i: isize = self.get_index() as isize + n;
        Self::from_index(
            (self.get_day().get_year() * size + i) / size,
            AbstractCulture::new().index_of(i, size as usize),
        )
    }
}

impl Display for LunarFestival {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.get_day(), self.get_name())
    }
}

impl PartialEq for LunarFestival {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for LunarFestival {}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use crate::festival::{LunarFestival, SolarFestival};
    use crate::lunar::LunarDay;
    use crate::solar::SolarDay;

    #[test]
    fn test1() {
        let f: LunarFestival = LunarFestival::from_index(2023, 0).unwrap();
        assert_eq!("农历甲辰年正月初一 春节", f.next(13).unwrap().to_string());
        assert_eq!(
            "农历壬寅年十一月廿九 冬至节",
            f.next(-3).unwrap().to_string()
        );
    }

    #[test]
    fn test2() {
        let f: LunarFestival = LunarFestival::from_index(2023, 0).unwrap();
        assert_eq!("农历壬寅年三月初五 清明节", f.next(-9).unwrap().to_string());
    }

    #[test]
    fn test3() {
        let f: LunarFestival = LunarDay::from_ymd(2010, 1, 15).get_festival().unwrap();
        assert_eq!("农历庚寅年正月十五 元宵节", f.to_string());
    }

    #[test]
    fn test4() {
        let f: LunarFestival = LunarDay::from_ymd(2021, 12, 29).get_festival().unwrap();
        assert_eq!("农历辛丑年十二月廿九 除夕", f.to_string());
    }

    #[test]
    fn test5() {
        let f: Option<SolarFestival> = SolarFestival::from_index(2023, 0);
        assert_eq!(false, f.is_none());
        assert_eq!(
            "2024年5月1日 五一劳动节",
            f.unwrap().next(13).unwrap().to_string()
        );
        assert_eq!(
            "2022年8月1日 八一建军节",
            f.unwrap().next(-3).unwrap().to_string()
        );
    }

    #[test]
    fn test6() {
        let f: Option<SolarFestival> = SolarFestival::from_index(2023, 0);
        assert_eq!(false, f.is_none());
        assert_eq!(
            "2022年3月8日 三八妇女节",
            f.unwrap().next(-9).unwrap().to_string()
        );
    }

    #[test]
    fn test7() {
        let f: Option<SolarFestival> = SolarDay::from_ymd(2010, 1, 1).get_festival();
        assert_eq!(false, f.is_none());
        assert_eq!("2010年1月1日 元旦", f.unwrap().to_string());
    }

    #[test]
    fn test8() {
        let f: Option<SolarFestival> = SolarDay::from_ymd(2021, 5, 4).get_festival();
        assert_eq!(false, f.is_none());
        assert_eq!("2021年5月4日 五四青年节", f.unwrap().to_string());
    }

    #[test]
    fn test9() {
        let f: Option<SolarFestival> = SolarDay::from_ymd(1939, 5, 4).get_festival();
        assert_eq!(true, f.is_none());
    }
}
