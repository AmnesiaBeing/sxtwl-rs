//! 农历朔望节气计算模块
//!
//! 本模块负责计算农历中的朔日、望日和二十四节气，支持从公元前721年至今的计算。
//! 采用分段拟合算法，针对不同历史时期使用不同的计算参数。
//!
//! 参考信息：https://www.astro.ink/sm1.htm
//! 参考源码：sxtwl_cpp/src/SSQ.cpp

use libm::{cos, floor, sin};

use crate::{
    compressed_qishuo_correction_data::{get_qi_correction, get_shuo_correction},
    consts::{
        J2000, JIEQI_INTERVAL, JULIAN_CENTURY_DAYS, LUNAR_MONTH_DAYS, SECONDS_PER_DAY,
        TROPICAL_YEAR_DAYS,
    },
    qishuo_fit_parameter::{FitParameter, QI_FIT_PARAMETERS, SHUO_FIT_PARAMETERS},
};
use core::f64::consts::PI;

const JD_MINUS_103_1_23_12_00_00: f64 = 1683460.0; // 特殊修正日期，用于修正-103年1月23日的朔日计算
const JD_1960_1_1_12_00_00: f64 = 2436935.0; // 1960年1月1日12:00:00的儒略日
const JD_1999_3_21_12_00_00: f64 = 2451259.0; // 1999年3月21日12:00:00的儒略日
const JD_2000_1_7_12_00_00: f64 = 2451551.0; // 2000年1月7日12:00:00的儒略日

// 计算类型枚举 - 内部使用
#[derive(PartialEq, Eq)]
pub(crate) enum CalculationType {
    Qi,   // 节气
    Shuo, // 朔日
}

// 计算方法枚举 - 内部使用
enum CalculationMethod {
    HighPrecision, // 高精度计算，用于1999年3月21日12:00:00之前的日期，及修正表记录之前的日期
    FlatPhase,     // 平气朔表计算，用于1645年以前的日期
    FixedPhase,    // 定朔相位计算，用于1645年以后至1999年3月21日之间的日期
}

/// 农历相位计算器
#[derive(Default)]
pub struct LunarPhaseCalculator {
    /// 计算后的25个节气的儒略日，从冬至开始到下一个冬至以后
    pub jieqi: [f64; 25],
    /// 当前计算农历年的上一年的前一个节气和前前一个节气的儒略日
    pub pre_jieqi: [f64; 2],
    /// 计算后的14个朔日，每个朔日的儒略日
    pub shuo: [f64; 14],
    /// 计算后的12个月的月序，每个月的月序从1开始
    pub month_indices: [i32; 14],
    /// 计算后的12个月的天数，每个月的天数从1开始
    pub month_lengths: [f64; 14],
    /// 计算后的闰月，None表示没有闰月
    pub leap_month: Option<i32>,
}

impl LunarPhaseCalculator {
    fn determine_calculation_method(
        &self,
        jd: f64,
        calc_type: &CalculationType,
    ) -> CalculationMethod {
        let params = self.get_fit_parameters(calc_type);
        let pc = self.get_period_constant(calc_type);

        let f1 = params[0].start_julian_day - pc;
        let f2 = params.last().unwrap().start_julian_day - pc;

        if jd < f1 || jd >= JD_1960_1_1_12_00_00 {
            CalculationMethod::HighPrecision
        } else if jd >= f1 && jd < f2 {
            CalculationMethod::FlatPhase
        } else {
            CalculationMethod::FixedPhase
        }
    }

    #[inline]
    fn get_fit_parameters(&self, calc_type: &CalculationType) -> &[FitParameter] {
        match calc_type {
            CalculationType::Qi => QI_FIT_PARAMETERS,
            CalculationType::Shuo => SHUO_FIT_PARAMETERS,
        }
    }

    #[inline]
    fn get_period_constant(&self, calc_type: &CalculationType) -> f64 {
        match calc_type {
            CalculationType::Qi => 7.0,
            CalculationType::Shuo => 14.0,
        }
    }

    // 现代天文算法
    fn calculate_high_precision(&self, adjusted_jd: f64, calc_type: &CalculationType) -> f64 {
        let pc = self.get_period_constant(calc_type);
        match calc_type {
            CalculationType::Qi => {
                floor(self.calculate_qi_high_precision(
                    (adjusted_jd + pc - JD_1999_3_21_12_00_00) / TROPICAL_YEAR_DAYS * 24.0 * PI / 12.0,
                )) + 1.0
            }
            CalculationType::Shuo => {
                floor(self.calculate_shuo_high_precision(
                    (adjusted_jd + pc - JD_2000_1_7_12_00_00) / LUNAR_MONTH_DAYS * 2.0 * PI,
                )) + 1.0
            }
        }
    }

    // 平气或平朔
    fn calculate_flat_phase(&self, adjusted_jd: f64, calc_type: &CalculationType) -> f64 {
        let mut i = 0;
        let params = self.get_fit_parameters(calc_type);
        let pc = self.get_period_constant(calc_type);

        while i + 1 < params.len() && adjusted_jd + pc >= params[i + 1].start_julian_day {
            i += 1;
        }

        let d = params[i].start_julian_day
            + params[i].period_days
                * floor((adjusted_jd + pc - params[i].start_julian_day) / params[i].period_days);
        let mut result = floor(d) + 0.5;

        // 特殊修正，如果使用太初历计算-103年1月24日的朔日，结果得到的是23日，这里修正为24日(实历)
        // 修正后仍不影响-103年的无中置闰
        // 如果使用秦汉历，得到的是24日，该修正不会被执行
        if result == JD_MINUS_103_1_23_12_00_00 {
            result += 1.0;
        }

        result - J2000
    }

    // 定气或定朔
    fn calculate_fixed_phase(&self, adjusted_jd: f64, calc_type: &CalculationType) -> f64 {
        let params = self.get_fit_parameters(calc_type);
        let pc = self.get_period_constant(calc_type);
        let f2 = params.last().unwrap().start_julian_day - pc;
        let pc = self.get_period_constant(calc_type);

        let (ret, n): (f64, u8) = match calc_type {
            CalculationType::Qi => (
                floor(self.calculate_qi_high_precision(
                    (adjusted_jd + pc - JD_1999_3_21_12_00_00) / TROPICAL_YEAR_DAYS * 24.0 * PI / 12.0,
                )) + 1.0,
                get_qi_correction(adjusted_jd, f2),
            ),
            CalculationType::Shuo => (
                floor(self.calculate_shuo_high_precision(
                    (adjusted_jd + pc - JD_2000_1_7_12_00_00) / LUNAR_MONTH_DAYS * 2.0 * PI,
                )) + 1.0,
                get_shuo_correction(adjusted_jd, f2),
            ),
        };

        // 根据修正值调整结果
        ret + match n {
            1 => 1.0,
            2 => -1.0,
            _ => 0.0,
        }
    }

    /// 计算指定儒略日的节气或朔日
    ///
    /// # 参数
    /// - `julian_day`: 儒略日数值
    /// - `calculation_type`: 计算类型（节气或朔日）
    ///
    /// # 返回
    /// 计算得到的节气或朔日对应的儒略日整数部分
    ///
    /// # 算法说明
    /// 根据日期范围自动选择计算方法：
    /// - 高精度算法：1999年之前及修正表记录之前
    /// - 平气朔表：1645年以前  
    /// - 定朔相位：1645年至1999年之间
    pub(crate) fn calculate_phase(&self, jd: f64, calc_type: CalculationType) -> f64 {
        let adjusted_jd = jd + J2000;

        match self.determine_calculation_method(adjusted_jd, &calc_type) {
            CalculationMethod::HighPrecision => {
                self.calculate_high_precision(adjusted_jd, &calc_type)
            }
            CalculationMethod::FlatPhase => self.calculate_flat_phase(adjusted_jd, &calc_type),
            CalculationMethod::FixedPhase => self.calculate_fixed_phase(adjusted_jd, &calc_type),
        }
    }

    /// 较高精度气计算
    fn calculate_qi_high_precision(&self, _angle: f64) -> f64 {
        // 注意：这里需要调用XL::S_aLon_t2等函数，暂时保留接口
        // 这些函数需要从eph.cpp中转换
        0.0
    }

    /// 较高精度朔计算
    fn calculate_shuo_high_precision(&self, _angle: f64) -> f64 {
        // 注意：这里需要调用XL::MS_aLon_t2等函数，暂时保留接口
        // 这些函数需要从eph.cpp中转换
        0.0
    }

    /// 低精度定朔计算
    fn calculate_shuo_low_precision(&self, angle: f64) -> f64 {
        const VELOCITY: f64 = 7771.37714500204;

        let mut time_param = (angle + 1.08472) / VELOCITY;

        // 天文校正项
        let correction = -0.0000331 * time_param * time_param
            + 0.10976 * cos(0.785 + 8328.6914 * time_param)
            + 0.02224 * cos(0.187 + 7214.0629 * time_param)
            - 0.03342 * cos(4.669 + 628.3076 * time_param);

        let t_plus_1_8 = time_param + 1.8;

        time_param -= correction / VELOCITY
            + (32.0 * t_plus_1_8 * t_plus_1_8 - 20.0) / SECONDS_PER_DAY / JULIAN_CENTURY_DAYS;

        time_param * JULIAN_CENTURY_DAYS + 8.0 / 24.0
    }

    /// 低精度定气计算
    fn calculate_qi_low_precision(&self, angle: f64) -> f64 {
        const VELOCITY: f64 = 628.3319653318;

        let mut time_param = (angle - 4.895062166) / VELOCITY; // 第一次估算

        // 第二次估算
        time_param -= (53.0 * time_param * time_param
            + 334116.0 * cos(4.669257 + 628.307585 * time_param)
            + 2061.0 * cos(2.67823 + 628.3076 * time_param) * time_param)
            / VELOCITY
            / 10000000.0;

        // 计算平黄经
        let l = 48950621.66
            + 6283319653.318 * time_param
            + 53.0 * time_param * time_param
            + 334166.0 * cos(4.669257 + 628.307585 * time_param)
            + 3489.0 * cos(4.6261 + 1256.61517 * time_param)
            + 2060.6 * cos(2.67823 + 628.307585 * time_param) * time_param
            - 994.0
            - 834.0 * sin(2.1824 - 33.75705 * time_param);

        let t_plus_1_8 = time_param + 1.8;

        time_param -= (l / 10000000.0 - angle) / VELOCITY
            + (32.0 * t_plus_1_8 * t_plus_1_8 - 20.0) / SECONDS_PER_DAY / JULIAN_CENTURY_DAYS;

        time_param * JULIAN_CENTURY_DAYS + 8.0 / 24.0
    }

    fn calculate_base_date(&self, julian_day: f64) -> f64 {
        let mut base =
            floor((julian_day - 355.0 + 183.0) / TROPICAL_YEAR_DAYS) * TROPICAL_YEAR_DAYS + 355.0;

        // 调整基准日期
        if self.calculate_phase(base, CalculationType::Qi) > julian_day {
            base -= TROPICAL_YEAR_DAYS;
        }

        base
    }

    // 计算25个节气时刻
    fn calculate_jie(&mut self, base_date: f64) {
        for i in 0..25 {
            self.jieqi[i] =
                self.calculate_phase(base_date + JIEQI_INTERVAL * i as f64, CalculationType::Qi);
        }

        // 补算二气
        self.pre_jieqi = [
            self.calculate_phase(base_date - JIEQI_INTERVAL, CalculationType::Qi),
            self.calculate_phase(base_date - JIEQI_INTERVAL * 2.0, CalculationType::Qi),
        ];
    }

    // 计算24个朔日时刻
    fn calculate_shuo(&mut self) {
        // 求较靠近冬至的朔日
        let mut nearest_dongzhi_shuo = self.calculate_phase(self.jieqi[0], CalculationType::Shuo);
        // 确保朔日不晚于冬至
        if nearest_dongzhi_shuo > self.jieqi[0] {
            nearest_dongzhi_shuo -= LUNAR_MONTH_DAYS;
        }

        // 计算该年所有朔
        for i in 0..14 {
            self.shuo[i] = self.calculate_phase(
                nearest_dongzhi_shuo + LUNAR_MONTH_DAYS * i as f64,
                CalculationType::Shuo,
            );
        }
    }

    // 计算月大小
    fn calculate_month_properties(&mut self) {
        for i in 0..14 {
            self.month_lengths[i] = self.shuo[i + 1] - self.shuo[i];
        }
    }

    // 使用无中气置闰法确定闰月
    fn determine_leap_month(&mut self) {
        if self.shuo[13] <= self.jieqi[24] {
            let mut i = 1;
            while i < 13 && self.shuo[i + 1] > self.jieqi[2 * i] {
                i += 1;
            }

            self.leap_month = Some(i as i32);
            for j in i..14 {
                self.month_indices[j] -= 1;
            }
        }
    }

    /// 农历排月序计算
    pub fn calculate_lunar_year_months(&mut self, julian_day: f64) {
        let base_date = self.calculate_base_date(julian_day);
        self.calculate_jie(base_date);
        self.calculate_shuo();
        self.calculate_month_properties();
        self.determine_leap_month();
    }
}
