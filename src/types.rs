//! 定义日历计算的核心枚举与数据结构

/// 天干（10个）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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

/// 地支（12个）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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

/// 节气（24个）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum JieQi {
    LiChun = 0,       // 立春
    YuShui = 1,       // 雨水
    JingZhe = 2,      // 惊蛰
    ChunFen = 3,      // 春分
    QingMing = 4,     // 清明
    GuYu = 5,         // 谷雨
    LiXia = 6,        // 立夏
    XiaoMan = 7,      // 小满
    MangZhong = 8,    // 芒种
    XiaZhi = 9,       // 夏至
    XiaoShu = 10,     // 小暑
    DaShu = 11,       // 大暑
    LiQiu = 12,       // 立秋
    ChuShu = 13,      // 处暑
    BaiLu = 14,       // 白露
    QiuFen = 15,      // 秋分
    HanLu = 16,       // 寒露
    ShuangJiang = 17, // 霜降
    LiDong = 18,      // 立冬
    XiaoXue = 19,     // 小雪
    DaXue = 20,       // 大雪
    DongZhi = 21,     // 冬至
    XiaoHan = 22,     // 小寒
    DaHan = 23,       // 大寒
}

/// 节气信息结构体
#[derive(Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub struct JieQiInfo {
    pub jd: JulianDay,   // 节气对应的儒略日
    pub jq_index: JieQi, // 节气索引
}

/// 生肖（12个，与地支一一对应）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
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

/// 公历日期（年、月、日、时、分、秒）
#[derive(Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub struct SolarDate {
    pub year: i32,   // 年份（支持BC，如公元前722年为-721）
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31（根据月份调整）
    pub hour: u8,    // 0-23
    pub minute: u8,  // 0-59
    pub second: f64, // 0.0-60.0（含闰秒）
}

/// 农历日期（年、月、日、是否闰月）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct LunarDate {
    pub year: i32,           // 农历年
    pub month: u8,           // 1-12
    pub day: u8,             // 1-30（农历月最多30天）
    pub is_leap_month: bool, // 是否为闰月
}

/// 儒略日（天文计算基础，高精度浮点数）
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct JulianDay(pub f64);

/// 干支组合（天干+地支）
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct GanZhi(pub TianGan, pub DiZhi);
