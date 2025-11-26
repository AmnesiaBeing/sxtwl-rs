use core::marker::PhantomData;

use crate::eightchar::{ChildLimitInfo, EightChar};
use crate::lunar::LunarHour;
#[cfg(feature = "eight-char-lunar-sect2-provider")]
use crate::sixtycycle::SixtyCycleHour;
use crate::solar::{SolarMonth, SolarTerm, SolarTime};
use crate::types::Tyme;

/// 童限计算接口
pub trait ChildLimitProvider {
    fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo;
}

/// 八字计算接口
pub trait EightCharProvider {
    fn get_eight_char(&self, hour: LunarHour) -> EightChar;
}

/// 默认的八字计算（晚子时算第二天）
#[cfg(feature = "eight-char-default-provider")]
#[derive(Debug, Copy, Clone)]
pub struct DefaultEightCharProvider {}

#[cfg(feature = "eight-char-default-provider")]
impl DefaultEightCharProvider {
    pub const fn new() -> Self {
        Self {}
    }
}

#[cfg(feature = "eight-char-default-provider")]
impl EightCharProvider for DefaultEightCharProvider {
    fn get_eight_char(&self, hour: LunarHour) -> EightChar {
        hour.get_sixty_cycle_hour().get_eight_char()
    }
}

/// Lunar流派2的八字计算（晚子时日柱算当天）
#[cfg(feature = "eight-char-lunar-sect2-provider")]
#[derive(Debug, Copy, Clone)]
pub struct LunarSect2EightCharProvider {}

#[cfg(feature = "eight-char-lunar-sect2-provider")]
impl LunarSect2EightCharProvider {
    pub const fn new() -> Self {
        Self {}
    }
}

#[cfg(feature = "eight-char-lunar-sect2-provider")]
impl EightCharProvider for LunarSect2EightCharProvider {
    fn get_eight_char(&self, hour: LunarHour) -> EightChar {
        let h: SixtyCycleHour = hour.get_sixty_cycle_hour();
        EightChar::from_sixty_cycle(
            h.get_year(),
            h.get_month(),
            hour.get_lunar_day().get_sixty_cycle(),
            h.get_sixty_cycle(),
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AbstractChildLimitProvider {}

impl ChildLimitProvider for AbstractChildLimitProvider {
    fn get_info(&self, _birth_time: SolarTime, _term: SolarTerm) -> ChildLimitInfo {
        unimplemented!()
    }
}

impl AbstractChildLimitProvider {
    pub const fn new() -> Self {
        Self {}
    }

    fn next(
        &self,
        birth_time: SolarTime,
        add_year: usize,
        add_month: usize,
        add_day: usize,
        add_hour: usize,
        add_minute: usize,
        add_second: usize,
    ) -> ChildLimitInfo {
        let mut d: usize = birth_time.get_day() + add_day;
        let mut h: usize = birth_time.get_hour() + add_hour;
        let mut mi: usize = birth_time.get_minute() + add_minute;
        let mut s: usize = birth_time.get_second() + add_second;
        mi += s / 60;
        s %= 60;
        h += mi / 60;
        mi %= 60;
        d += h / 24;
        h %= 24;

        let mut sm: SolarMonth = SolarMonth::from_ym(
            birth_time.get_year() + add_year as isize,
            birth_time.get_month(),
        )
        .next(add_month as isize);

        let mut dc: usize = sm.get_day_count();
        while d > dc {
            d -= dc;
            sm = sm.next(1);
            dc = sm.get_day_count();
        }

        ChildLimitInfo {
            start_time: birth_time,
            end_time: SolarTime::from_ymd_hms(sm.get_year(), sm.get_month(), d, h, mi, s),
            year_count: add_year,
            month_count: add_month,
            day_count: add_day,
            hour_count: add_hour,
            minute_count: add_minute,
        }
    }
}

/// 默认的童限计算（3年4月5天6时7分）
#[cfg(feature = "child-limit-default-provider")]
#[derive(Debug, Copy, Clone)]
pub struct DefaultChildLimitProvider {
    parent: AbstractChildLimitProvider,
}

#[cfg(feature = "child-limit-default-provider")]
impl DefaultChildLimitProvider {
    pub const fn new() -> Self {
        Self {
            parent: AbstractChildLimitProvider::new(),
        }
    }
}

#[cfg(feature = "child-limit-default-provider")]
impl ChildLimitProvider for DefaultChildLimitProvider {
    fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
        // 出生时刻和节令时刻相差的秒数
        let mut seconds: usize = term
            .get_julian_day()
            .get_solar_time()
            .subtract(birth_time)
            .abs() as usize;
        // 3天 = 1年，3天=60*60*24*3秒=259200秒 = 1年
        let year: usize = seconds / 259200;
        seconds %= 259200;
        // 1天 = 4月，1天=60*60*24秒=86400秒 = 4月，85400秒/4=21600秒 = 1月
        let month: usize = seconds / 21600;
        seconds %= 21600;
        // 1时 = 5天，1时=60*60秒=3600秒 = 5天，3600秒/5=720秒 = 1天
        let day: usize = seconds / 720;
        seconds %= 720;
        // 1分 = 2时，60秒 = 2时，60秒/2=30秒 = 1时
        let hour: usize = seconds / 30;
        seconds %= 30;
        // 1秒 = 2分，1秒/2=0.5秒 = 1分
        let minute: usize = seconds * 2;

        self.parent
            .next(birth_time, year, month, day, hour, minute, 0)
    }
}

/// 元亨利贞的童限计算（3年4月5天6时7分）
#[cfg(feature = "child-limit-china95-provider")]
#[derive(Debug, Copy, Clone)]
pub struct China95ChildLimitProvider {
    parent: AbstractChildLimitProvider,
}

#[cfg(feature = "child-limit-china95-provider")]
impl China95ChildLimitProvider {
    pub const fn new() -> Self {
        Self {
            parent: AbstractChildLimitProvider::new(),
        }
    }
}

#[cfg(feature = "child-limit-china95-provider")]
impl ChildLimitProvider for China95ChildLimitProvider {
    fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
        // 出生时刻和节令时刻相差的分钟数
        let mut minutes: usize = term
            .get_julian_day()
            .get_solar_time()
            .subtract(birth_time)
            .abs() as usize
            / 60;
        let year: usize = minutes / 4320;
        minutes %= 4320;
        let month: usize = minutes / 360;
        minutes %= 360;
        let day: usize = minutes / 12;

        self.parent.next(birth_time, year, month, day, 0, 0, 0)
    }
}

/// Lunar的流派1童限计算（按天数和时辰数计算，3天1年，1天4个月，1时辰10天）
#[cfg(feature = "child-limit-lunar-sect1-provider")]
#[derive(Debug, Copy, Clone)]
pub struct LunarSect1ChildLimitProvider {
    parent: AbstractChildLimitProvider,
}

#[cfg(feature = "child-limit-lunar-sect1-provider")]
impl LunarSect1ChildLimitProvider {
    pub const fn new() -> Self {
        Self {
            parent: AbstractChildLimitProvider::new(),
        }
    }
}

#[cfg(feature = "child-limit-lunar-sect1-provider")]
impl ChildLimitProvider for LunarSect1ChildLimitProvider {
    fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
        let term_time: SolarTime = term.get_julian_day().get_solar_time();
        let mut end: SolarTime = term_time;
        let mut start: SolarTime = birth_time;
        if birth_time.is_after(term_time) {
            end = birth_time;
            start = term_time;
        }
        let end_time_zhi_index: usize = if end.get_hour() == 23 {
            11
        } else {
            end.get_lunar_hour().get_index_in_day()
        };
        let start_time_zhi_index: usize = if start.get_hour() == 23 {
            11
        } else {
            start.get_lunar_hour().get_index_in_day()
        };
        // 时辰差
        let mut hour_diff: isize = end_time_zhi_index as isize - start_time_zhi_index as isize;
        // 天数差
        let mut day_diff: isize = end.get_solar_day().subtract(start.get_solar_day());
        if hour_diff < 0 {
            hour_diff += 12;
            day_diff -= 1;
        }
        let month_diff: isize = hour_diff * 10 / 30;
        let mut month: isize = day_diff * 4 + month_diff;
        let day: isize = hour_diff * 10 - month_diff * 30;
        let year: isize = month / 12;
        month = month - year * 12;

        self.parent.next(
            birth_time,
            year as usize,
            month as usize,
            day as usize,
            0,
            0,
            0,
        )
    }
}

/// Lunar的流派2童限计算（按分钟数计算，3年4月5天6时7分）
#[cfg(feature = "child-limit-lunar-sect2-provider")]
#[derive(Debug, Copy, Clone)]
pub struct LunarSect2ChildLimitProvider {
    parent: AbstractChildLimitProvider,
}

#[cfg(feature = "child-limit-lunar-sect2-provider")]
impl LunarSect2ChildLimitProvider {
    pub const fn new() -> Self {
        Self {
            parent: AbstractChildLimitProvider::new(),
        }
    }
}

#[cfg(feature = "child-limit-lunar-sect2-provider")]
impl ChildLimitProvider for LunarSect2ChildLimitProvider {
    fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
        // 出生时刻和节令时刻相差的分钟数
        let mut minutes: usize = term
            .get_julian_day()
            .get_solar_time()
            .subtract(birth_time)
            .abs() as usize
            / 60;
        let year: usize = minutes / 4320;
        minutes %= 4320;
        let month: usize = minutes / 360;
        minutes %= 360;
        let day: usize = minutes / 12;
        minutes %= 12;
        let hour: usize = minutes * 2;

        self.parent.next(birth_time, year, month, day, hour, 0, 0)
    }
}

// 八字服务
pub struct EightCharService<P: EightCharProvider> {
    provider: P,
    _marker: PhantomData<P>,
}

impl<P: EightCharProvider> EightCharService<P> {
    pub const fn new(provider: P) -> Self {
        Self {
            provider,
            _marker: PhantomData,
        }
    }

    /// 获取八字
    pub fn get_eight_char(&self, lunar: LunarHour) -> EightChar {
        self.provider.get_eight_char(lunar)
    }
}

// 八字全局静态实例
#[cfg(feature = "eight-char-default-provider")]
pub static EIGHT_CHAR_PROVIDER: EightCharService<DefaultEightCharProvider> =
    EightCharService::new(DefaultEightCharProvider::new());

#[cfg(feature = "eight-char-lunar-sect2-provider")]
pub static EIGHT_CHAR_PROVIDER: EightCharService<LunarSect2EightCharProvider> =
    EightCharService::new(LunarSect2EightCharProvider::new());

// 童限服务
pub struct ChildLimitService<P: ChildLimitProvider> {
    provider: P,
    _marker: PhantomData<P>,
}

impl<P: ChildLimitProvider> ChildLimitService<P> {
    pub const fn new(provider: P) -> Self {
        Self {
            provider,
            _marker: PhantomData,
        }
    }

    /// 获取童限
    pub fn get_info(&self, birth_time: SolarTime, term: SolarTerm) -> ChildLimitInfo {
        self.provider.get_info(birth_time, term)
    }
}

// 童限全局静态实例
#[cfg(feature = "child-limit-default-provider")]
pub static CHILD_LIMIT_PROVIDER: ChildLimitService<DefaultChildLimitProvider> =
    ChildLimitService::new(DefaultChildLimitProvider::new());

#[cfg(feature = "child-limit-china95-provider")]
pub static CHILD_LIMIT_PROVIDER: ChildLimitService<China95ChildLimitProvider> =
    ChildLimitService::new(China95ChildLimitProvider::new());

#[cfg(feature = "child-limit-lunar-sect1-provider")]
pub static CHILD_LIMIT_PROVIDER: ChildLimitService<LunarSect1ChildLimitProvider> =
    ChildLimitService::new(LunarSect1ChildLimitProvider::new());

#[cfg(feature = "child-limit-lunar-sect2-provider")]
pub static CHILD_LIMIT_PROVIDER: ChildLimitService<LunarSect2ChildLimitProvider> =
    ChildLimitService::new(LunarSect2ChildLimitProvider::new());
