//! 日期相关功能，包括农历日期、节气、干支等计算

use crate::consts::J2000;
use crate::gz::GanZhi;
use crate::lunar_phase_calculator::LunarPhaseCalculator;
use crate::types::JulianDay;
use crate::types::{LunarDate, SolarDate};
use alloc::boxed::Box;
use alloc::rc::Rc;
use libm::{floor, fmod};

/// 日期计算的核心结构，提供公历/农历转换、节气、干支等功能
pub struct Day {
    d0: i32, // 从J2000起的天数（儒略日-2451545）

    // 公历信息
    y: i32, // 公历年
    m: u8,  // 公历月
    d: i32, // 公历日

    // 农历信息
    lmc: i32,    // 阴历月的月
    ldi: u8,     // 阴历月的日
    ldn: i32,    // 该阴历月的总天数
    lleap: bool, // 是不是阴历的闰月

    // 农历年信息
    lyear: i32,  // 以立春为界，定农历纪年(10进制,1984年起算)
    lyear0: i32, // 以正月初一为界，农历纪年(10进制,1984年起算)

    // 其他信息
    week: u8,  // 星期几
    qk: i8,    // 节令值
    xiz: u8,   // 星座
    jqjd: f64, // 节气具体的时间

    // 干支信息
    lyear2: Option<Box<GanZhi>>,  // 干支纪年(立春)
    lyear3: Option<Box<GanZhi>>,  // 干支纪年(正月 春节)
    lmonth2: Option<Box<GanZhi>>, // 月天干地支
    lday2: Option<Box<GanZhi>>,   // 日天干地支

    // 计算器
    lunar_calculator: LunarPhaseCalculator,
}

impl Day {
    /// 创建新的Day实例
    fn new(d0: i32) -> Self {
        Self {
            d0,
            y: 0,
            m: 0,
            d: 0,
            lmc: 0,
            ldi: 0,
            ldn: 0,
            lleap: false,
            lyear: 0,
            lyear0: 0,
            week: 0xFF,
            qk: -2,
            xiz: 0xFF,
            jqjd: 0.0,
            lyear2: None,
            lyear3: None,
            lmonth2: None,
            lday2: None,
            lunar_calculator: LunarPhaseCalculator::default(),
        }
    }

    /// 计算农历数据
    fn check_lunar_data(&mut self) {
        // 如果已经计算过了，直接返回
        if self.ldn != 0 {
            return;
        }

        let calculator = &self.lunar_calculator;

        // 查找当前日期所在的农历月
        let mut mk = 0;
        while mk < 13 && calculator.shuo[mk + 1] <= self.d0 as f64 {
            mk += 1;
        }

        // 设置农历月信息
        self.lmc = if let Some(month_idx) = calculator.month_indices.get(mk) {
            *month_idx
        } else {
            1
        };

        // 设置月的天数
        self.ldn = if let Some(length) = calculator.month_lengths.get(mk) {
            floor(*length) as i32
        } else {
            30 // 默认小月
        };

        // 检查是否为闰月
        self.lleap = calculator.leap_month == Some(mk as i32);

        // 计算农历日
        self.ldi = (self.d0 as f64 - calculator.shuo[mk]) as u8;
    }

    /// 计算公历数据
    fn check_solar_data(&mut self) {
        if self.m != 0 {
            return;
        }

        // 将J2000天数转换为儒略日，然后转换为SolarDate
        let jd = JulianDay::from_j2000_days(self.d0);
        let solar_date: SolarDate = JulianDay(jd).into();

        self.y = solar_date.year;
        self.m = solar_date.month;
        self.d = solar_date.day as i32;
    }

    /// 计算节气数据
    fn check_jq_data(&mut self) {
        if self.qk != -2 {
            return;
        }

        self.qk = -1;
        self.get_jie_qi_jd();
    }

    /// 获取几天后的日期
    pub fn after(&self, day: i32) -> Day {
        Day::new(self.d0 + day)
    }

    /// 获取几天前的日期
    pub fn before(&self, day: i32) -> Day {
        Day::new(self.d0 - day)
    }

    /// 获取阴历日期
    pub fn get_lunar_day(&mut self) -> i32 {
        self.check_lunar_data();
        self.ldi as i32 + 1
    }

    /// 获取阴历月
    pub fn get_lunar_month(&mut self) -> u8 {
        self.check_lunar_data();

        // 计算农历月
        let mut month = self.lmc;
        if month > 2 {
            month -= 2;
        } else {
            month += 10;
        }

        month as u8
    }

    /// 获取阴历年
    /// chinese_new_year_boundary: 是否以春节为界
    pub fn get_lunar_year(&mut self, chinese_new_year_boundary: bool) -> i32 {
        let calculator = &self.lunar_calculator;

        let jd = self.d0 as f64;

        // 以立春为界（第4个节气）
        if !chinese_new_year_boundary {
            if self.lyear == 0 {
                let 立春_jd = calculator.jieqi[3];
                let offset = if jd < 立春_jd { -365.0 } else { 0.0 };
                let d = 立春_jd + offset + 365.25 * 16.0 - 35.0;
                self.lyear = floor(d / 365.2422 + 0.5) as i32;
            }
            return self.lyear + 1984;
        }

        // 以春节为界
        if self.lyear0 == 0 {
            // 查找正月初一（第一个月）
            let mut 春节_jd = calculator.shuo[2];

            // 遍历查找正月初一
            for j in 0..14 {
                if let Some(month_idx) = calculator.month_indices.get(j) {
                    // 正月（寅月）
                    if *month_idx == 2
                        && (!calculator.leap_month.is_some()
                            || calculator.leap_month.unwrap() != j as i32)
                    {
                        春节_jd = calculator.shuo[j];
                        if jd < 春节_jd {
                            春节_jd -= 365.0;
                        }
                        break;
                    }
                }
            }

            // 计算农历年份
            let d = 春节_jd + 5810.0;
            self.lyear0 = floor(d / 365.2422 + 0.5) as i32;
        }

        self.lyear0 + 1984
    }

    // /// 获取阴历年干支
    // pub fn get_year_gz(&mut self, chinese_new_year_boundary: bool) -> GanZhi {
    //     // 以春节为界
    //     if chinese_new_year_boundary {
    //         if self.lyear3.is_none() {
    //             let year = self.get_lunar_year(chinese_new_year_boundary) - 1984;
    //             let d = year + 12000;
    //             self.lyear3 = Some(
    //                 GanZhi::new((d % 10) as u8, (d % 12) as u8).unwrap_or(GanZhi {
    //                     tian_gan: 0,
    //                     di_zhi: 0,
    //                 }),
    //             );
    //         }
    //         *self.lyear3.as_ref().unwrap()
    //     } else {
    //         // 以立春为界
    //         if self.lyear2.is_none() {
    //             let year = self.get_lunar_year(false) - 1984;
    //             let d = year + 12000;
    //             self.lyear2 = Some(Box::new(
    //                 GanZhi::new((d % 10) as u8, (d % 12) as u8).unwrap_or(GanZhi {
    //                     tian_gan: 0,
    //                     di_zhi: 0,
    //                 }),
    //             ));
    //         }
    //         *self.lyear2.as_ref().unwrap()
    //     }
    // }

    // /// 获取月天干地支
    // pub fn get_month_gz(&mut self) -> GanZhi {
    //     if self.lmonth2.is_none() {
    //         let calculator = self.get_lunar_calculator();

    //         // 计算相对于大雪的月数
    //         let mk = floor((self.d0 as f64 - calculator.jieqi[0]) / 30.43685) as usize;

    //         // 调整月数
    //         let adjusted_mk = if mk < 12 && self.d0 as f64 >= calculator.jieqi[2 * mk + 1] {
    //             mk + 1
    //         } else {
    //             mk
    //         };

    //         // 计算月干支
    //         let year_frac = floor((calculator.jieqi[12] + 390.0) / 365.2422) as i32;
    //         let d = adjusted_mk + year_frac * 12 + 900000;

    //         self.lmonth2 = Some(Box::new(
    //             GanZhi::new((d % 10) as u8, (d % 12) as u8).unwrap_or(GanZhi {
    //                 tian_gan: 0,
    //                 di_zhi: 0,
    //             }),
    //         ));
    //     }

    //     *self.lmonth2.as_ref().unwrap()
    // }

    // /// 获取日天干地支
    // pub fn get_day_gz(&mut self) -> GanZhi {
    //     if self.lday2.is_none() {
    //         // 正确的日天干地支计算方法
    //         let d = self.d0 - 6 + 9000000;

    //         // 计算天干地支
    //         let tian_gan = (d % 10) as u8;
    //         let di_zhi = (d % 12) as u8;

    //         self.lday2 = Some(Box::new(
    //             GanZhi::new(tian_gan, di_zhi).unwrap_or(GanZhi { tian_gan, di_zhi }),
    //         ));
    //     }

    //     *self.lday2.as_ref().unwrap()
    // }

    // /// 获取时天干地支
    // pub fn get_hour_gz(&mut self, hour: u8, is_zao_wan_zi_shi: bool) -> GanZhi {
    //     let day_gz = self.get_day_gz();

    //     // 计算时天干地支
    //     // 时天干 = (日天干 * 2 + 时地支) % 10
    //     // 时地支 = (hour / 2) % 12
    //     let mut shi_zhi = (hour / 2) % 12;

    //     // 特殊处理早晚子时
    //     if is_zao_wan_zi_shi {
    //         // 晚上23点到24点为晚子时，算作下一天的子时
    //         if hour == 23 {
    //             shi_zhi = 0; // 子
    //         }
    //     }

    //     // 计算时天干
    //     let shi_gan = (day_gz.tian_gan * 2 + shi_zhi) % 10;

    //     GanZhi::new(shi_gan, shi_zhi).unwrap_or(GanZhi {
    //         tian_gan: 0,
    //         di_zhi: 0,
    //     })
    // }

    /// 是否是闰月
    pub fn is_lunar_leap(&mut self) -> bool {
        self.check_lunar_data();
        self.lleap
    }

    /// 获取公历年
    pub fn get_solar_year(&mut self) -> i32 {
        self.check_solar_data();
        self.y
    }

    /// 获取公历月
    pub fn get_solar_month(&mut self) -> u8 {
        self.check_solar_data();
        self.m
    }

    /// 获取公历日
    pub fn get_solar_day(&mut self) -> i32 {
        self.check_solar_data();
        self.d
    }

    /// 获取星期几
    pub fn get_week(&mut self) -> u8 {
        if self.week == 0xFF {
            // 计算星期几：(儒略日 + 1) % 7
            self.week = ((self.d0 + J2000 as i32 + 1 + 7000000) % 7) as u8;
        }
        self.week
    }

    /// 获取处于该月的第几周
    pub fn get_week_index(&mut self) -> u8 {
        let day = self.get_solar_day() - 1;
        let i = day % 7;

        let week = self.get_week() as i32;
        let w0 = if week >= i { week - i } else { week + 7 - i };

        (((w0 + day) / 7) + 1) as u8
    }

    /// 是否有节气
    pub fn has_jie_qi(&mut self) -> bool {
        self.check_jq_data();
        self.qk != -1
    }

    /// 获取节气
    pub fn get_jie_qi(&mut self) -> u8 {
        self.check_jq_data();
        self.qk as u8
    }

    /// 获取节气的儒略日
    pub fn get_jie_qi_jd(&mut self) -> f64 {
        if self.jqjd != 0.0 {
            return self.jqjd;
        }

        let calculator = &self.lunar_calculator;

        // 查找当前日期对应的节气
        for i in 0..24 {
            if (calculator.jieqi[i] - self.d0 as f64).abs() < 0.5 {
                self.jqjd = calculator.jieqi[i];
                self.qk = i as i8;
                break;
            }
        }

        self.jqjd
    }

    /// 获取星座
    pub fn get_constellation(&mut self) -> u8 {
        if self.xiz == 0xFF {
            let calculator = &self.lunar_calculator;

            // 计算星座所在月的序数
            let mk = floor((self.d0 as f64 - calculator.jieqi[0] - 15.0) / 30.43685) as usize;

            // 调整星座序数
            let adjusted_mk = if mk < 11 && self.d0 as f64 >= calculator.jieqi[2 * mk + 2] {
                mk + 1
            } else {
                mk
            };

            // 计算星座索引
            self.xiz = ((adjusted_mk + 12) % 12) as u8;
        }
        self.xiz
    }

    /// 从公历日期创建Day实例
    pub fn from_solar(year: i32, month: u8, day: i32) -> Day {
        let solar_date = SolarDate {
            year,
            month,
            day: day as u8,
            hour: 12,
            minute: 0,
            second: 0.1,
        };

        let jd: JulianDay = solar_date.into();
        let d0 = JulianDay::to_j2000_days(jd.value());

        Day::new(d0)
    }

    /// 从SolarDate创建Day实例
    pub fn from_solar_date(solar_date: SolarDate) -> Day {
        // 将SolarDate转换为儒略日
        let jd: JulianDay = solar_date.into();
        // 转换为J2000天
        let d0 = JulianDay::to_j2000_days(jd.value()) as i32;

        Day::new(d0)
    }

    /// 从农历日期创建Day实例
    pub fn from_lunar(year: i32, month: u8, day: i32, is_run: bool) -> Day {
        // 创建临时SolarDate用于计算基准儒略日
        let solar_date = SolarDate {
            year: year - if month > 10 { 0 } else { 1 },
            month: if month > 10 { 1 } else { 12 },
            day: 1,
            hour: 12,
            minute: 0,
            second: 0.1,
        };

        let jd: JulianDay = solar_date.into();
        let bd0 = JulianDay::to_j2000_days(jd.value());

        // 创建计算器并计算该年的农历数据
        let mut calculator = LunarPhaseCalculator::default();
        calculator.calculate_lunar_year_months(bd0 as f64);

        // 月份映射数组
        const YUE_INDEX: [i32; 12] = [11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // 查找对应的月份索引
        let mut yue = 0;
        for i in 0..YUE_INDEX.len() {
            if YUE_INDEX[i] == month as i32 {
                yue = i;
                break;
            }
        }

        let yue = yue as i32;

        // 查找对应的农历月
        let mut mk = 0;
        let leap = calculator.leap_month.unwrap_or(-1) - 1;

        for i in 0..calculator.month_indices.len() {
            if let Some(it) = calculator.month_indices.get(i) {
                if leap < 0 {
                    if *it == yue as i32 {
                        break;
                    }
                } else {
                    if (yue < leap) && (*it == yue as i32) {
                        break;
                    }

                    if yue == leap && *it == yue as i32 && is_run {
                        mk += 1;
                        break;
                    }

                    if yue == leap && *it == yue as i32 && !is_run {
                        break;
                    }

                    if yue > leap && *it == yue as i32 {
                        break;
                    }
                }
                mk += 1;
            }
        }

        // 计算儒略日
        let bdi = if let Some(shuo) = calculator.shuo.get(mk) {
            *shuo
        } else {
            bd0 as f64
        };

        let jd = bdi + day as f64 - 1.0;

        Day::new(jd as i32)
    }

    /// 转换为农历日期
    pub fn to_lunar_date(&mut self) -> LunarDate {
        self.check_lunar_data();

        // 获取农历年（以春节为界）
        let year = self.get_lunar_year(true);

        // 计算农历月
        // 月份映射：11 -> 11(冬月), 12 -> 12(腊月), 1 -> 1(正月), ...
        let mut month = self.lmc;
        if month > 2 {
            month -= 2;
        } else {
            month += 10;
        }

        // 处理闰月
        let is_leap_month = self.lleap;

        // 农历日
        let day = self.ldi as i32 + 1; // 0-29/30 转换为 1-30/31

        LunarDate {
            year,
            month: month as u8,
            day: day as u8,
            is_leap_month,
        }
    }

    /// 转换为公历日期
    pub fn to_solar_date(&mut self) -> SolarDate {
        let jd = JulianDay::from_j2000_days(self.d0);
        JulianDay(jd).into()
    }

    /// 获取公历日期
    pub fn get_solar_date(&mut self) -> SolarDate {
        self.to_solar_date()
    }
}

// 这个实现是不完整的，因为缺少SSQ类的具体实现
// 在实际使用时，需要完整实现SSQ类和相关的天文计算函数

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_solar() {
        let mut day = Day::from_solar(2024, 1, 1);
        assert_eq!(day.get_solar_year(), 2024);
        assert_eq!(day.get_solar_month(), 1);
        assert_eq!(day.get_solar_day(), 1);
    }
}
