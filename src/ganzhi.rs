//! 天干地支计算模块
//! 包含年、月、日、时干支的计算逻辑

use crate::{DiZhi, TianGan, types::GanZhi};

impl TianGan {
    /// 获取天干字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            TianGan::Jia => "甲",
            TianGan::Yi => "乙",
            TianGan::Bing => "丙",
            TianGan::Ding => "丁",
            TianGan::Wu => "戊",
            TianGan::Ji => "己",
            TianGan::Geng => "庚",
            TianGan::Xin => "辛",
            TianGan::Ren => "壬",
            TianGan::Gui => "癸",
        }
    }

    /// 从索引获取天干
    pub fn from_index(index: usize) -> Self {
        match index % 10 {
            0 => TianGan::Jia,
            1 => TianGan::Yi,
            2 => TianGan::Bing,
            3 => TianGan::Ding,
            4 => TianGan::Wu,
            5 => TianGan::Ji,
            6 => TianGan::Geng,
            7 => TianGan::Xin,
            8 => TianGan::Ren,
            9 => TianGan::Gui,
            _ => unreachable!(),
        }
    }
}

impl DiZhi {
    /// 获取地支字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            DiZhi::Zi => "子",
            DiZhi::Chou => "丑",
            DiZhi::Yin => "寅",
            DiZhi::Mao => "卯",
            DiZhi::Chen => "辰",
            DiZhi::Si => "巳",
            DiZhi::Wu => "午",
            DiZhi::Wei => "未",
            DiZhi::Shen => "申",
            DiZhi::You => "酉",
            DiZhi::Xu => "戌",
            DiZhi::Hai => "亥",
        }
    }

    /// 从索引获取地支
    pub fn from_index(index: usize) -> Self {
        match index % 12 {
            0 => DiZhi::Zi,
            1 => DiZhi::Chou,
            2 => DiZhi::Yin,
            3 => DiZhi::Mao,
            4 => DiZhi::Chen,
            5 => DiZhi::Si,
            6 => DiZhi::Wu,
            7 => DiZhi::Wei,
            8 => DiZhi::Shen,
            9 => DiZhi::You,
            10 => DiZhi::Xu,
            11 => DiZhi::Hai,
            _ => unreachable!(),
        }
    }
}

impl GanZhi {
    /// 获取干支字符串
    // pub fn as_str(&self) -> &'static str {
    //     match self {
    //         GanZhi(TianGan::Jia, DiZhi::Zi) => "甲子",
    //         GanZhi(TianGan::Jia, DiZhi::Chou) => "甲丑",
    //         GanZhi(TianGan::Jia, DiZhi::Yin) => "甲寅",
    //         GanZhi(TianGan::Jia, DiZhi::Mao) => "甲卯",
    //         GanZhi(TianGan::Jia, DiZhi::Chen) => "甲辰",
    //         GanZhi(TianGan::Jia, DiZhi::Si) => "甲巳",
    //         GanZhi(TianGan::Jia, DiZhi::Wu) => "甲午",
    //         GanZhi(TianGan::Jia, DiZhi::Wei) => "甲未",
    //         GanZhi(TianGan::Jia, DiZhi::Shen) => "甲申",
    //         GanZhi(TianGan::Jia, DiZhi::You) => "甲酉",
    //         GanZhi(TianGan::Jia, DiZhi::Xu) => "甲戌",
    //         GanZhi(TianGan::Jia, DiZhi::Hai) => "甲亥",
    //         GanZhi(TianGan::Yi, DiZhi::Zi) => "乙子",
    //         GanZhi(TianGan::Yi, DiZhi::Chou) => "乙丑",
    //         GanZhi(TianGan::Yi, DiZhi::Yin) => "乙寅",
    //         GanZhi(TianGan::Yi, DiZhi::Mao) => "乙卯",
    //         GanZhi(TianGan::Yi, DiZhi::Chen) => "乙辰",
    //         GanZhi(TianGan::Yi, DiZhi::Si) => "乙巳",
    //         GanZhi(TianGan::Yi, DiZhi::Wu) => "乙午",
    //         GanZhi(TianGan::Yi, DiZhi::Wei) => "乙未",
    //         GanZhi(TianGan::Yi, DiZhi::Shen) => "乙申",
    //         GanZhi(TianGan::Yi, DiZhi::You) => "乙酉",
    //         GanZhi(TianGan::Yi, DiZhi::Xu) => "乙戌",
    //         GanZhi(TianGan::Yi, DiZhi::Hai) => "乙亥",
    //         GanZhi(TianGan::Bing, DiZhi::Zi) => "丙子",
    //         GanZhi(TianGan::Bing, DiZhi::Chou) => "丁丑",
    //         GanZhi(TianGan::Bing, DiZhi::Yin) => "丁寅",
    //         GanZhi(TianGan::Bing, DiZhi::Mao) => "丁卯",
    //         GanZhi(TianGan::Bing, DiZhi::Chen) => "丁辰",
    //         GanZhi(TianGan::Bing, DiZhi::Si) => "丁巳",
    //         GanZhi(TianGan::Bing, DiZhi::Wu) => "丁午",
    //         GanZhi(TianGan::Bing, DiZhi::Wei) => "丁未",
    //         GanZhi(TianGan::Bing, DiZhi::Shen) => "丁申",
    //         GanZhi(TianGan::Bing, DiZhi::You) => "丁酉",
    //         GanZhi(TianGan::Bing, DiZhi::Xu) => "丁戌",
    //         GanZhi(TianGan::Bing, DiZhi::Hai) => "丁亥",
    //         GanZhi(TianGan::Ding, DiZhi::Zi) => "丁子",
    //         GanZhi(TianGan::Ding, DiZhi::Chou) => "丁丑",
    //         GanZhi(TianGan::Ding, DiZhi::Yin) => "丁寅",
    //         GanZhi(TianGan::Ding, DiZhi::Mao) => "丁卯",
    //         GanZhi(TianGan::Ding, DiZhi::Chen) => "丁辰",
    //         GanZhi(TianGan::Ding, DiZhi::Si) => "丁巳",
    //         GanZhi(TianGan::Ding, DiZhi::Wu) => "丁午",
    //         GanZhi(TianGan::Ding, DiZhi::Wei) => "丁未",
    //         GanZhi(TianGan::Ding, DiZhi::Shen) => "丁申",
    //         GanZhi(TianGan::Ding, DiZhi::You) => "丁酉",
    //         GanZhi(TianGan::Ding, DiZhi::Xu) => "丁戌",
    //         GanZhi(TianGan::Ding, DiZhi::Hai) => "丁亥",
    //     }
    // }

    /// 从年份获取干支
    ///
    /// # 参数
    /// - `year`: 年份（支持BC，如公元前722年为-721）
    ///
    /// # 返回值
    /// 干支结构体
    pub fn from_year(year: i32) -> Self {
        let diff = year - 1984;
        let tg = ((diff % 10 + 10) % 10) as usize;
        let dz = ((diff % 12 + 12) % 12) as usize;
        Self(TianGan::from_index(tg), DiZhi::from_index(dz))
    }
}
