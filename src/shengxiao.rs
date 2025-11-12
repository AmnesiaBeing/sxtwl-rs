//! 生肖计算模块
//! 提供生肖相关的计算与转换功能

use crate::{DiZhi, ShengXiao, types::LunarDate};

impl ShengXiao {
    /// 获取生肖中文名称
    pub fn as_str(&self) -> &'static str {
        match self {
            ShengXiao::Shu => "鼠",
            ShengXiao::Niu => "牛",
            ShengXiao::Hu => "虎",
            ShengXiao::Tu => "兔",
            ShengXiao::Long => "龙",
            ShengXiao::She => "蛇",
            ShengXiao::Ma => "马",
            ShengXiao::Yang => "羊",
            ShengXiao::Hou => "猴",
            ShengXiao::Ji => "鸡",
            ShengXiao::Gou => "狗",
            ShengXiao::Zhu => "猪",
        }
    }

    /// 从索引获取生肖 (0-11)
    pub fn from_index(index: usize) -> Self {
        const SHENGXIAO: [ShengXiao; 12] = [
            ShengXiao::Shu,
            ShengXiao::Niu,
            ShengXiao::Hu,
            ShengXiao::Tu,
            ShengXiao::Long,
            ShengXiao::She,
            ShengXiao::Ma,
            ShengXiao::Yang,
            ShengXiao::Hou,
            ShengXiao::Ji,
            ShengXiao::Gou,
            ShengXiao::Zhu,
        ];
        SHENGXIAO[index % 12]
    }

    /// 获取生肖索引 (0-11)
    pub fn to_index(&self) -> usize {
        match self {
            ShengXiao::Shu => 0,
            ShengXiao::Niu => 1,
            ShengXiao::Hu => 2,
            ShengXiao::Tu => 3,
            ShengXiao::Long => 4,
            ShengXiao::She => 5,
            ShengXiao::Ma => 6,
            ShengXiao::Yang => 7,
            ShengXiao::Hou => 8,
            ShengXiao::Ji => 9,
            ShengXiao::Gou => 10,
            ShengXiao::Zhu => 11,
        }
    }

    /// 从农历年份获取生肖
    ///
    /// # 参数
    /// - `year`: 农历年份（支持BC，如公元前722年为-721）
    ///
    /// # 返回值
    /// 生肖枚举
    pub fn from_lunar_year(year: i32) -> Self {
        // 1984年是鼠年，作为参考点
        const REFERENCE_YEAR: i32 = 1984;
        let diff = year - REFERENCE_YEAR;

        // 计算生肖索引，确保结果为非负数
        let index = ((diff % 12 + 12) % 12) as usize;
        Self::from_index(index)
    }

    /// 从地支获取对应的生肖
    ///
    /// # 参数
    /// - `dizhi`: 地支
    ///
    /// # 返回值
    /// 对应的生肖
    pub fn from_dizhi(dizhi: DiZhi) -> Self {
        // 生肖与地支一一对应
        match dizhi {
            DiZhi::Zi => ShengXiao::Shu,
            DiZhi::Chou => ShengXiao::Niu,
            DiZhi::Yin => ShengXiao::Hu,
            DiZhi::Mao => ShengXiao::Tu,
            DiZhi::Chen => ShengXiao::Long,
            DiZhi::Si => ShengXiao::She,
            DiZhi::Wu => ShengXiao::Ma,
            DiZhi::Wei => ShengXiao::Yang,
            DiZhi::Shen => ShengXiao::Hou,
            DiZhi::You => ShengXiao::Ji,
            DiZhi::Xu => ShengXiao::Gou,
            DiZhi::Hai => ShengXiao::Zhu,
        }
    }

    /// 获取生肖对应的地支
    ///
    /// # 返回值
    /// 对应的地支
    pub fn to_dizhi(&self) -> DiZhi {
        match self {
            ShengXiao::Shu => DiZhi::Zi,
            ShengXiao::Niu => DiZhi::Chou,
            ShengXiao::Hu => DiZhi::Yin,
            ShengXiao::Tu => DiZhi::Mao,
            ShengXiao::Long => DiZhi::Chen,
            ShengXiao::She => DiZhi::Si,
            ShengXiao::Ma => DiZhi::Wu,
            ShengXiao::Yang => DiZhi::Wei,
            ShengXiao::Hou => DiZhi::Shen,
            ShengXiao::Ji => DiZhi::You,
            ShengXiao::Gou => DiZhi::Xu,
            ShengXiao::Zhu => DiZhi::Hai,
        }
    }
}

// 为方便使用，实现Display trait
impl core::fmt::Display for ShengXiao {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// 为LunarDate添加获取生肖的方法
impl LunarDate {
    /// 获取农历日期对应的生肖
    ///
    /// # 返回值
    /// 生肖枚举
    pub fn shengxiao(&self) -> ShengXiao {
        ShengXiao::from_lunar_year(self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_shengxiao_as_str() {
        // 测试生肖字符串表示
        assert_eq!(ShengXiao::Shu.as_str(), "鼠");
        assert_eq!(ShengXiao::Niu.as_str(), "牛");
        assert_eq!(ShengXiao::Hu.as_str(), "虎");
        assert_eq!(ShengXiao::Tu.as_str(), "兔");
        assert_eq!(ShengXiao::Long.as_str(), "龙");
        assert_eq!(ShengXiao::She.as_str(), "蛇");
        assert_eq!(ShengXiao::Ma.as_str(), "马");
        assert_eq!(ShengXiao::Yang.as_str(), "羊");
        assert_eq!(ShengXiao::Hou.as_str(), "猴");
        assert_eq!(ShengXiao::Ji.as_str(), "鸡");
        assert_eq!(ShengXiao::Gou.as_str(), "狗");
        assert_eq!(ShengXiao::Zhu.as_str(), "猪");
    }

    #[test]
    fn test_shengxiao_to_index() {
        // 测试生肖索引
        assert_eq!(ShengXiao::Shu.to_index(), 0);
        assert_eq!(ShengXiao::Niu.to_index(), 1);
        assert_eq!(ShengXiao::Hu.to_index(), 2);
        assert_eq!(ShengXiao::Long.to_index(), 4);
        assert_eq!(ShengXiao::Gou.to_index(), 10);
        assert_eq!(ShengXiao::Zhu.to_index(), 11);
    }

    #[test]
    fn test_shengxiao_from_index() {
        // 测试从索引创建生肖
        assert!(matches!(ShengXiao::from_index(0), ShengXiao::Shu));
        assert!(matches!(ShengXiao::from_index(5), ShengXiao::She));
        assert!(matches!(ShengXiao::from_index(11), ShengXiao::Zhu));
        assert!(matches!(ShengXiao::from_index(12), ShengXiao::Shu)); // 循环测试
        assert!(matches!(ShengXiao::from_index(23), ShengXiao::Zhu)); // 循环测试
    }

    #[test]
    fn test_shengxiao_from_lunar_year() {
        // 测试参考年1984（鼠年）
        assert!(matches!(ShengXiao::from_lunar_year(1984), ShengXiao::Shu));

        // 测试2023年（兔年）
        assert!(matches!(ShengXiao::from_lunar_year(2023), ShengXiao::Tu));

        // 测试2024年（龙年）
        assert!(matches!(ShengXiao::from_lunar_year(2024), ShengXiao::Long));

        // 测试2000年（龙年）
        assert!(matches!(ShengXiao::from_lunar_year(2000), ShengXiao::Long));

        // 测试公元前年份
        assert!(matches!(ShengXiao::from_lunar_year(-721), ShengXiao::She)); // 公元前722年

        // 测试未来年份
        assert!(matches!(ShengXiao::from_lunar_year(2050), ShengXiao::Ma));
    }

    #[test]
    fn test_shengxiao_dizhi_conversion() {
        // 测试生肖与地支的相互转换
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Zi), ShengXiao::Shu));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Chou), ShengXiao::Niu));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Yin), ShengXiao::Hu));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Mao), ShengXiao::Tu));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Chen), ShengXiao::Long));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Si), ShengXiao::She));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Wu), ShengXiao::Ma));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Wei), ShengXiao::Yang));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Shen), ShengXiao::Hou));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::You), ShengXiao::Ji));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Xu), ShengXiao::Gou));
        assert!(matches!(ShengXiao::from_dizhi(DiZhi::Hai), ShengXiao::Zhu));

        // 测试反向转换
        assert!(matches!(ShengXiao::Shu.to_dizhi(), DiZhi::Zi));
        assert!(matches!(ShengXiao::Niu.to_dizhi(), DiZhi::Chou));
        assert!(matches!(ShengXiao::Hu.to_dizhi(), DiZhi::Yin));
        assert!(matches!(ShengXiao::Zhu.to_dizhi(), DiZhi::Hai));
    }

    #[test]
    fn test_shengxiao_display() {
        // 测试Display trait
        assert_eq!(format!("{}", ShengXiao::Shu), "鼠");
        assert_eq!(format!("{}", ShengXiao::Niu), "牛");
        assert_eq!(format!("{}", ShengXiao::Hu), "虎");
        assert_eq!(format!("{}", ShengXiao::Long), "龙");
        assert_eq!(format!("{}", ShengXiao::Zhu), "猪");
    }

    #[test]
    fn test_lunar_date_shengxiao() {
        // 测试LunarDate的shengxiao方法
        let lunar = LunarDate {
            year: 2023,
            month: 1,
            day: 1,
            is_leap_month: false,
        };
        assert!(matches!(lunar.shengxiao(), ShengXiao::Tu));

        let lunar2 = LunarDate {
            year: 1984,
            month: 1,
            day: 1,
            is_leap_month: false,
        };
        assert!(matches!(lunar2.shengxiao(), ShengXiao::Shu));
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let max_index = ShengXiao::from_index(11);
        assert_eq!(max_index.to_index(), 11);

        let overflow_index = ShengXiao::from_index(100);
        assert_eq!(overflow_index.to_index(), 4); // 100 % 12 = 4

        // 测试负年份
        let ancient_year = ShengXiao::from_lunar_year(-1000);
        // 只是确保不会panic，不检查具体值
        assert!(!ancient_year.as_str().is_empty());

        // 测试一致性
        for i in 0..12 {
            let shengxiao = ShengXiao::from_index(i);
            assert_eq!(shengxiao.to_index(), i);
        }
    }
}