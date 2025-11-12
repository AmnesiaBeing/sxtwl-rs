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
        const GAN: [TianGan; 10] = [
            TianGan::Jia,
            TianGan::Yi,
            TianGan::Bing,
            TianGan::Ding,
            TianGan::Wu,
            TianGan::Ji,
            TianGan::Geng,
            TianGan::Xin,
            TianGan::Ren,
            TianGan::Gui,
        ];
        GAN[index % 10]
    }

    /// 获取天干索引 (0-9)
    pub fn to_index(&self) -> usize {
        match self {
            TianGan::Jia => 0,
            TianGan::Yi => 1,
            TianGan::Bing => 2,
            TianGan::Ding => 3,
            TianGan::Wu => 4,
            TianGan::Ji => 5,
            TianGan::Geng => 6,
            TianGan::Xin => 7,
            TianGan::Ren => 8,
            TianGan::Gui => 9,
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
        const ZHI: [DiZhi; 12] = [
            DiZhi::Zi,
            DiZhi::Chou,
            DiZhi::Yin,
            DiZhi::Mao,
            DiZhi::Chen,
            DiZhi::Si,
            DiZhi::Wu,
            DiZhi::Wei,
            DiZhi::Shen,
            DiZhi::You,
            DiZhi::Xu,
            DiZhi::Hai,
        ];
        ZHI[index % 12]
    }

    /// 获取地支索引 (0-11)
    pub fn to_index(&self) -> usize {
        match self {
            DiZhi::Zi => 0,
            DiZhi::Chou => 1,
            DiZhi::Yin => 2,
            DiZhi::Mao => 3,
            DiZhi::Chen => 4,
            DiZhi::Si => 5,
            DiZhi::Wu => 6,
            DiZhi::Wei => 7,
            DiZhi::Shen => 8,
            DiZhi::You => 9,
            DiZhi::Xu => 10,
            DiZhi::Hai => 11,
        }
    }
}

impl GanZhi {
    /// 预定义的干支字符串查找表
    const GANZHI_STR: [&'static str; 60] = [
        "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌",
        "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉",
        "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申",
        "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未",
        "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午",
        "己未", "庚申", "辛酉", "壬戌", "癸亥",
    ];

    /// 获取干支字符串
    pub fn as_str(&self) -> &'static str {
        let index = self.to_index();
        Self::GANZHI_STR[index]
    }

    /// 从索引获取干支 (0-59)
    pub fn from_index(index: usize) -> Self {
        let index = index % 60;
        let gan_index = index % 10;
        let zhi_index = index % 12;
        Self(TianGan::from_index(gan_index), DiZhi::from_index(zhi_index))
    }

    /// 获取干支索引 (0-59)
    pub fn to_index(&self) -> usize {
        let gan_index = self.0.to_index();
        let zhi_index = self.1.to_index();

        // 干支索引计算：找到天干地支匹配的位置
        // 由于干支是60个一循环，我们需要找到满足条件的索引
        for i in 0..60 {
            if (i % 10 == gan_index) && (i % 12 == zhi_index) {
                return i;
            }
        }

        // 理论上不会执行到这里，因为干支组合总是有效的
        0
    }

    /// 从年份获取干支
    ///
    /// # 参数
    /// - `year`: 农历年份（支持BC，如公元前722年为-721）
    ///
    /// # 返回值
    /// 干支结构体
    pub fn from_lunar_year(year: i32) -> Self {
        // 1984年是甲子年，作为参考点
        const REFERENCE_YEAR: i32 = 1984;
        let diff = year - REFERENCE_YEAR;

        // 计算天干地支索引，确保结果为非负数
        let tg_index = ((diff % 10 + 10) % 10) as usize;
        let dz_index = ((diff % 12 + 12) % 12) as usize;

        Self(TianGan::from_index(tg_index), DiZhi::from_index(dz_index))
    }
}

// 为方便使用，实现Display trait
impl core::fmt::Display for TianGan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::fmt::Display for DiZhi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::fmt::Display for GanZhi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_tiangan_basics() {
        // 测试天干字符串
        assert_eq!(TianGan::Jia.as_str(), "甲");
        assert_eq!(TianGan::Yi.as_str(), "乙");
        assert_eq!(TianGan::Gui.as_str(), "癸");

        // 测试天干索引
        assert_eq!(TianGan::Jia.to_index(), 0);
        assert_eq!(TianGan::Geng.to_index(), 6);
        assert_eq!(TianGan::Gui.to_index(), 9);

        // 测试天干从索引创建
        assert!(matches!(TianGan::from_index(0), TianGan::Jia));
        assert!(matches!(TianGan::from_index(5), TianGan::Ji));
        assert!(matches!(TianGan::from_index(15), TianGan::Ji)); // 循环测试

        // 测试Display trait
        assert_eq!(format!("{}", TianGan::Bing), "丙");
    }

    #[test]
    fn test_dizhi_basics() {
        // 测试地支字符串
        assert_eq!(DiZhi::Zi.as_str(), "子");
        assert_eq!(DiZhi::Chou.as_str(), "丑");
        assert_eq!(DiZhi::Hai.as_str(), "亥");

        // 测试地支索引
        assert_eq!(DiZhi::Zi.to_index(), 0);
        assert_eq!(DiZhi::Wu.to_index(), 6);
        assert_eq!(DiZhi::Hai.to_index(), 11);

        // 测试地支从索引创建
        assert!(matches!(DiZhi::from_index(0), DiZhi::Zi));
        assert!(matches!(DiZhi::from_index(6), DiZhi::Wu));
        assert!(matches!(DiZhi::from_index(18), DiZhi::Wu)); // 循环测试

        // 测试Display trait
        assert_eq!(format!("{}", DiZhi::Mao), "卯");
    }

    #[test]
    fn test_ganzhi_from_index() {
        // 测试一些关键干支
        let gz1 = GanZhi::from_index(0);
        assert!(matches!(gz1.0, TianGan::Jia));
        assert!(matches!(gz1.1, DiZhi::Zi));
        assert_eq!(gz1.as_str(), "甲子");

        let gz2 = GanZhi::from_index(59);
        assert!(matches!(gz2.0, TianGan::Gui));
        assert!(matches!(gz2.1, DiZhi::Hai));
        assert_eq!(gz2.as_str(), "癸亥");

        let gz3 = GanZhi::from_index(35);
        assert!(matches!(gz3.0, TianGan::Ji));
        assert!(matches!(gz3.1, DiZhi::Hai));
        assert_eq!(gz3.as_str(), "己亥");

        // 测试循环
        let gz4 = GanZhi::from_index(60); // 应该等同于0
        assert!(matches!(gz4.0, TianGan::Jia));
        assert!(matches!(gz4.1, DiZhi::Zi));
    }

    #[test]
    fn test_ganzhi_to_index() {
        // 测试甲子
        let gz1 = GanZhi(TianGan::Jia, DiZhi::Zi);
        assert_eq!(gz1.to_index(), 0);
        assert_eq!(gz1.as_str(), "甲子");

        // 测试乙丑
        let gz2 = GanZhi(TianGan::Yi, DiZhi::Chou);
        assert_eq!(gz2.to_index(), 1);
        assert_eq!(gz2.as_str(), "乙丑");

        // 测试癸亥
        let gz3 = GanZhi(TianGan::Gui, DiZhi::Hai);
        assert_eq!(gz3.to_index(), 59);
        assert_eq!(gz3.as_str(), "癸亥");

        // 测试己亥
        let gz4 = GanZhi(TianGan::Ji, DiZhi::Hai);
        assert_eq!(gz4.to_index(), 35);
        assert_eq!(gz4.as_str(), "己亥");
    }

    #[test]
    fn test_ganzhi_consistency() {
        // 测试所有60个干支的索引一致性
        for i in 0..60 {
            let gz = GanZhi::from_index(i);
            let calculated_index = gz.to_index();
            assert_eq!(
                i, calculated_index,
                "干支索引不一致: 期望 {}, 实际 {}",
                i, calculated_index
            );

            // 测试字符串格式
            let expected_gan = TianGan::from_index(i % 10);
            let expected_zhi = DiZhi::from_index(i % 12);
            assert!(matches!(gz.0, _ if gz.0.to_index() == expected_gan.to_index()));
            assert!(matches!(gz.1, _ if gz.1.to_index() == expected_zhi.to_index()));
        }
    }

    #[test]
    fn test_ganzhi_from_year() {
        // 测试参考年1984（甲子年）
        let gz1 = GanZhi::from_lunar_year(1984);
        assert!(matches!(gz1.0, TianGan::Jia));
        assert!(matches!(gz1.1, DiZhi::Zi));
        assert_eq!(gz1.as_str(), "甲子");

        // 测试2023年（癸卯年）
        let gz2 = GanZhi::from_lunar_year(2023);
        assert!(matches!(gz2.0, TianGan::Gui));
        assert!(matches!(gz2.1, DiZhi::Mao));
        assert_eq!(gz2.as_str(), "癸卯");

        // 测试2000年（庚辰年）
        let gz3 = GanZhi::from_lunar_year(2000);
        assert!(matches!(gz3.0, TianGan::Geng));
        assert!(matches!(gz3.1, DiZhi::Chen));
        assert_eq!(gz3.as_str(), "庚辰");

        // 测试公元前722年
        let gz4 = GanZhi::from_lunar_year(-721); // 公元前722年
        assert_eq!(gz4.as_str(), "己未");

        // 测试未来年份
        let gz5 = GanZhi::from_lunar_year(2050);
        assert_eq!(gz5.as_str(), "庚午");
    }

    #[test]
    fn test_ganzhi_display() {
        let gz = GanZhi(TianGan::Bing, DiZhi::Yin);
        assert_eq!(format!("{}", gz), "丙寅");

        let gz2 = GanZhi(TianGan::Xin, DiZhi::You);
        assert_eq!(format!("{}", gz2), "辛酉");
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let max_gz = GanZhi::from_index(59);
        assert_eq!(max_gz.to_index(), 59);

        let overflow_gz = GanZhi::from_index(100);
        assert_eq!(overflow_gz.to_index(), 40); // 100 % 60 = 40

        // 测试负年份
        let ancient_gz = GanZhi::from_lunar_year(-1000);
        // 只是确保不会panic，不检查具体值
        assert!(!ancient_gz.as_str().is_empty());
    }
}
