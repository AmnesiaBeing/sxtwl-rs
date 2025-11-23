//! 干支（天干地支）结构和相关功能

/// 干支结构（天干地支）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhi {
    /// 天干（0-9）
    pub tian_gan: u8, // 天干：甲(0), 乙(1), 丙(2), 丁(3), 戊(4), 己(5), 庚(6), 辛(7), 壬(8), 癸(9)
    /// 地支（0-11）
    pub di_zhi: u8,  // 地支：子(0), 丑(1), 寅(2), 卯(3), 辰(4), 巳(5), 午(6), 未(7), 申(8), 酉(9), 戌(10), 亥(11)
}

impl GanZhi {
    /// 创建新的干支实例
    pub fn new(tian_gan: u8, di_zhi: u8) -> Result<Self, &'static str> {
        if tian_gan > 9 || di_zhi > 11 {
            return Err("天干必须在0-9范围内，地支必须在0-11范围内");
        }
        Ok(Self {
            tian_gan,
            di_zhi,
        })
    }
    
    /// 获取干支索引（0-59）
    pub fn get_index(&self) -> Result<u8, &'static str> {
        // 验证输入
        if self.tian_gan > 9 || self.di_zhi > 11 {
            return Err("无效的天干地支值");
        }
        
        // 计算索引
        for i in 0..6 {
            if (self.tian_gan + i * 10) % 12 == self.di_zhi as u8 {
                return Ok(self.tian_gan + i * 10);
            }
        }
        Err("无法找到对应的干支索引")
    }
    
    /// 获取天干的中文字符
    pub fn get_tian_gan_str(&self) -> &'static str {
        static TIANGAN_CHARS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        TIANGAN_CHARS.get(self.tian_gan as usize).unwrap_or(&"未知")
    }
    
    /// 获取地支的中文字符
    pub fn get_di_zhi_str(&self) -> &'static str {
        static DIZHI_CHARS: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        DIZHI_CHARS.get(self.di_zhi as usize).unwrap_or(&"未知")
    }
    
    /// 获取完整的干支字符串
    pub fn to_string(&self) -> String {
        format!("{}{}", self.get_tian_gan_str(), self.get_di_zhi_str())
    }
}