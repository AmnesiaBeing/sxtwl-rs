//! 定义日历计算的核心枚举与数据结构

// 天干（10个）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TianGan {
    Jia,  // 甲
    Yi,   // 乙
    Bing, // 丙
    Ding, // 丁
    Wu,   // 戊
    Ji,   // 己
    Geng, // 庚
    Xin,  // 辛
    Ren,  // 壬
    Gui,  // 癸
}

// 地支（12个）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiZhi {
    Zi,   // 子
    Chou, // 丑
    Yin,  // 寅
    Mao,  // 卯
    Chen, // 辰
    Si,   // 巳
    Wu,   // 午
    Wei,  // 未
    Shen, // 申
    You,  // 酉
    Xu,   // 戌
    Hai,  // 亥
}

// 节气（24个，与原库Jqmc顺序一致）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JieQi {
    DongZhi,     // 冬至
    XiaoHan,     // 小寒
    DaHan,       // 大寒
    LiChun,      // 立春
    YuShui,      // 雨水
    JingZhe,     // 惊蛰
    ChunFen,     // 春分
    QingMing,    // 清明
    GuYu,        // 谷雨
    LiXia,       // 立夏
    XiaoMan,     // 小满
    MangZhong,   // 芒种
    XiaZhi,      // 夏至
    XiaoShu,     // 小暑
    DaShu,       // 大暑
    LiQiu,       // 立秋
    ChuShu,      // 处暑
    BaiLu,       // 白露
    QiuFen,      // 秋分
    HanLu,       // 寒露
    ShuangJiang, // 霜降
    LiDong,      // 立冬
    XiaoXue,     // 小雪
    DaXue,       // 大雪
}

// 生肖（12个，与地支一一对应）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShengXiao {
    Shu,  // 鼠（子）
    Niu,  // 牛（丑）
    Hu,   // 虎（寅）
    Tu,   // 兔（卯）
    Long, // 龙（辰）
    She,  // 蛇（巳）
    Ma,   // 马（午）
    Yang, // 羊（未）
    Hou,  // 猴（申）
    Ji,   // 鸡（酉）
    Gou,  // 狗（戌）
    Zhu,  // 猪（亥）
}

// 星座（12个，与原库XiZ顺序一致）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XingZuo {
    MoJie,     // 摩羯
    ShuiPing,  // 水瓶
    ShuangYu,  // 双鱼
    BaiYang,   // 白羊
    JinNiu,    // 金牛
    ShuangZi,  // 双子
    JuXie,     // 巨蟹
    ShiZi,     // 狮子
    ChuNv,     // 处女
    TianCheng, // 天秤
    TianXie,   // 天蝎
    SheShou,   // 射手
}

/// 公历日期（年、月、日、时、分、秒）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarDate {
    pub year: i32,   // 年份（支持BC，如公元前722年为-721）
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31（根据月份调整）
    pub hour: u8,    // 0-23
    pub minute: u8,  // 0-59
    pub second: f64, // 0.0-60.0（含闰秒）
}

/// 农历日期（年、月、日、是否闰月）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarDate {
    pub year: i32,     // 农历年
    pub month: u8,     // 1-12
    pub day: u8,       // 1-30（农历月最多30天）
    pub is_leap: bool, // 是否为闰月
}

/// 儒略日（天文计算基础，高精度浮点数）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JulianDay(pub f64);

/// 干支组合（天干+地支）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhi {
    pub tg: TianGan,
    pub dz: DiZhi,
}

/// 时间片段（用于节气精确时间等）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: f64,
}
