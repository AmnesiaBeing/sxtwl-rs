//! 朔望节气计算模块
//! 负责计算农历中的朔日、望日和二十四节气

use crate::consts::{J2000};
use crate::types::{QSType};
use std::f64::consts::PI;

// 定义拟合参数结构体
#[derive(Debug, Clone)]
pub struct FitParameter {
    pub start_jd: f64,    // 起始儒略日
    pub period: f64,      // 周期天数
}

// 定义SSQ结构体
pub struct SSQ {
    // 使用预计算的修正表替代解压字符串，提高性能
    // 这里直接使用解压后的数据作为常量
    pub sb: String,       // 定朔修正表
    pub qb: String,       // 定气修正表
    
    // 使用结构体数组替代原始的long double数组
    pub suo_kb: Vec<FitParameter>,  // 朔直线拟合参数
    pub qi_kb: Vec<FitParameter>,   // 气直线拟合参数
    
    // 其他成员变量
    pub zq: Vec<f64>,
    pub zq_pe1: f64,
    pub zq_pe2: f64,
    pub hs: Vec<i32>,
    pub ym: Vec<i32>,
    pub dx: Vec<i32>,
    pub leap: i32,
}

impl SSQ {
    /// 创建新的SSQ实例
    pub fn new() -> Self {
        // 直接预计算解压后的数据，避免运行时解压
        // 在实际项目中，可以考虑使用build.rs在编译时解压并存为常量
        let sb = Self::precomputed_sb();
        let qb = Self::precomputed_qb();
        
        // 初始化拟合参数
        let suo_kb = Self::init_suo_kb();
        let qi_kb = Self::init_qi_kb();
        
        Self {
            sb,
            qb,
            suo_kb,
            qi_kb,
            zq: Vec::new(),
            zq_pe1: 0.0,
            zq_pe2: 0.0,
            hs: Vec::new(),
            ym: Vec::new(),
            dx: Vec::new(),
            leap: 0,
        }
    }
    
    /// 预计算的定朔修正表
    fn precomputed_sb() -> String {
        // 在实际项目中，这里应该是解压后的完整字符串
        // 为了演示，这里只返回一个简化版本
        // 实际应用中建议在编译时使用build.rs解压并生成常量
        String::new()
    }
    
    /// 预计算的定气修正表
    fn precomputed_qb() -> String {
        // 同样，在实际项目中，这里应该是解压后的完整字符串
        String::new()
    }
    
    /// 初始化朔直线拟合参数
    fn init_suo_kb() -> Vec<FitParameter> {
        vec![
            FitParameter { start_jd: 1457698.231017, period: 29.53067166 },  // -721-12-17
            FitParameter { start_jd: 1546082.512234, period: 29.53085106 },  // -479-12-11
            FitParameter { start_jd: 1640640.735300, period: 29.53060000 },  // -221-10-31
            FitParameter { start_jd: 1642472.151543, period: 29.53085439 },  // -216-11-04
            FitParameter { start_jd: 1683430.509300, period: 29.53086148 },  // -104-12-25
            FitParameter { start_jd: 1752148.041079, period: 29.53085097 },  //   85-02-13
            FitParameter { start_jd: 1807665.420323, period: 29.53059851 },  //  237-02-12
            FitParameter { start_jd: 1883618.114100, period: 29.53060000 },  //  445-01-24
            FitParameter { start_jd: 1907360.704700, period: 29.53060000 },  //  510-01-26
            FitParameter { start_jd: 1936596.224900, period: 29.53060000 },  //  590-02-10
            FitParameter { start_jd: 1939135.675300, period: 29.53060000 },  //  597-01-24
        ]
    }
    
    /// 初始化气直线拟合参数
    fn init_qi_kb() -> Vec<FitParameter> {
        vec![
            FitParameter { start_jd: 1640650.479938, period: 15.21842500 },   // -221-11-09
            FitParameter { start_jd: 1642476.703182, period: 15.21874996 },   // -216-11-09
            FitParameter { start_jd: 1683430.515601, period: 15.218750011 },  // -104-12-25
            FitParameter { start_jd: 1752157.640664, period: 15.218749978 },  //   85-02-23
            FitParameter { start_jd: 1807675.003759, period: 15.218620279 },  //  237-02-22
            FitParameter { start_jd: 1883627.765182, period: 15.218612292 },  //  445-02-03
            FitParameter { start_jd: 1907369.128100, period: 15.218449176 },  //  510-02-03
            FitParameter { start_jd: 1936603.140413, period: 15.218425000 },  //  590-02-17
            FitParameter { start_jd: 1939145.524180, period: 15.218466998 },  //  597-02-03
            FitParameter { start_jd: 1947180.798300, period: 15.218524844 },  //  619-02-03
            FitParameter { start_jd: 1964362.041824, period: 15.218533526 },  //  666-02-17
            FitParameter { start_jd: 1987372.340971, period: 15.218513908 },  //  729-02-16
            FitParameter { start_jd: 1999653.819126, period: 15.218530782 },  //  762-10-03
            FitParameter { start_jd: 2007445.469786, period: 15.218535181 },  //  784-02-01
            FitParameter { start_jd: 2021324.917146, period: 15.218526248 },  //  822-02-01
            FitParameter { start_jd: 2047257.232342, period: 15.218519654 },  //  893-01-31
            FitParameter { start_jd: 2070282.898213, period: 15.218425000 },  //  956-02-16
            FitParameter { start_jd: 2073204.872850, period: 15.218515221 },  //  964-02-16
            FitParameter { start_jd: 2080144.500926, period: 15.218530782 },  //  983-02-16
            FitParameter { start_jd: 2086703.688963, period: 15.218523776 },  // 1001-01-31
            FitParameter { start_jd: 2110033.182763, period: 15.218425000 },  // 1064-12-15
            FitParameter { start_jd: 2111190.300888, period: 15.218425000 },  // 1068-02-15
            FitParameter { start_jd: 2113731.271005, period: 15.218515671 },  // 1075-01-30
            FitParameter { start_jd: 2120670.840263, period: 15.218425000 },  // 1094-01-30
            FitParameter { start_jd: 2123973.309063, period: 15.218425000 },  // 1103-02-14
            FitParameter { start_jd: 2125068.997336, period: 15.218477932 },  // 1106-02-14
            FitParameter { start_jd: 2136026.312633, period: 15.218472436 },  // 1136-02-14
            FitParameter { start_jd: 2156099.495538, period: 15.218425000 },  // 1191-01-29
            FitParameter { start_jd: 2159021.324663, period: 15.218425000 },  // 1199-01-29
            FitParameter { start_jd: 2162308.575254, period: 15.218461742 },  // 1208-01-30
            FitParameter { start_jd: 2178485.706538, period: 15.218425000 },  // 1252-05-15
            FitParameter { start_jd: 2178759.662849, period: 15.218445786 },  // 1253-02-13
            FitParameter { start_jd: 2185334.020800, period: 15.218425000 },  // 1271-02-13
            FitParameter { start_jd: 2187525.481425, period: 15.218425000 },  // 1277-02-12
            FitParameter { start_jd: 2188621.191481, period: 15.218437494 },  // 1280-02-13
        ]
    }
    
    /// 解压函数 - 在实际项目中，建议在编译时处理，而不是运行时
    pub fn jieya(&self, s: &str) -> String {
        // 注意：在Rust中，我们可以使用编译时计算或外部资源文件替代运行时解压
        // 这里保留原有的解压逻辑，但仅作为参考
        let o = "0000000000";  // 10个0
        let o2 = format!("{}{}", o, o); // 20个0
        
        let mut result = s.to_string();
        
        // 替换字符
        result = result.replace("J", "00");
        result = result.replace("I", "000");
        result = result.replace("H", "0000");
        result = result.replace("G", "00000");
        result = result.replace("t", "02");
        result = result.replace("s", "002");
        result = result.replace("r", "0002");
        result = result.replace("q", "00002");
        result = result.replace("p", "000002");
        result = result.replace("o", "0000002");
        result = result.replace("n", "00000002");
        result = result.replace("m", "000000002");
        result = result.replace("l", "0000000002");
        result = result.replace("k", "01");
        result = result.replace("j", "0101");
        result = result.replace("i", "001");
        result = result.replace("h", "001001");
        result = result.replace("g", "0001");
        result = result.replace("f", "00001");
        result = result.replace("e", "000001");
        result = result.replace("d", "0000001");
        result = result.replace("c", "00000001");
        result = result.replace("b", "000000001");
        result = result.replace("a", "0000000001");
        result = result.replace("A", &format!("{}{}{}", o2, o2, o2)); // 60个0
        result = result.replace("B", &format!("{}{}{}", o2, o2, o));  // 50个0
        result = result.replace("C", &format!("{}{}", o2, o2));       // 40个0
        result = result.replace("D", &format!("{}{}", o2, o));        // 30个0
        result = result.replace("E", o2);                             // 20个0
        result = result.replace("F", o);                              // 10个0
        
        result
    }
    
    /// 计算函数
    pub fn calc(&self, jd: f64, qs: QSType) -> i32 {
        let jd_adj = jd + 2451545.0;
        let mut b = &self.suo_kb;
        let mut pc = 14.0;
        
        // 如果查的是气朔
        if qs == QSType::QiType {
            b = &self.qi_kb;
            pc = 7.0;
        }
        
        let f1 = b[0].start_jd - pc;
        let f2 = b.last().unwrap().start_jd - pc;
        let f3 = 2436935.0;
        
        if jd_adj < f1 || jd_adj >= f3 {
            // 平气朔表中首个之前，使用现代天文算法
            if qs == QSType::QiType {
                return self.qi_high((jd_adj + pc - 2451259.0) / 365.2422 * 24.0 * PI / 12.0).floor() as i32 + 1;
            } else {
                return self.so_high((jd_adj + pc - 2451551.0) / 29.5306 * 2.0 * PI).floor() as i32 + 1;
            }
        }
        
        if jd_adj >= f1 && jd_adj < f2 {
            // 平气或平朔
            let mut i = 0;
            while i + 1 < b.len() && jd_adj + pc >= b[i + 1].start_jd {
                i += 1;
            }
            
            let d = b[i].start_jd + b[i].period * ((jd_adj + pc - b[i].start_jd) / b[i].period).floor();
            let mut result = d.floor() + 0.5;
            
            // 特殊修正
            if result == 1683460.0 {
                result += 1.0;
            }
            
            return (result - 2451545.0) as i32;
        }
        
        if jd_adj >= f2 && jd_adj < f3 {
            // 定气或定朔
            let mut d = 0.0;
            let n = "";
            
            if qs == QSType::QiType {
                d = self.qi_low((jd_adj + pc - 2451259.0) / 365.2422 * 24.0 * PI / 12.0).floor() + 0.5;
                // 找定气修正值，这里暂时使用空字符串
            } else {
                d = self.so_low((jd_adj + pc - 2451551.0) / 29.5306 * 2.0 * PI).floor() + 0.5;
                // 找定朔修正值，这里暂时使用空字符串
            }
            
            // 根据修正值调整结果
            match n {
                "1" => return (d + 1.0) as i32,
                "2" => return (d - 1.0) as i32,
                _ => return d as i32,
            }
        }
        
        0
    }
    
    /// 较高精度气计算
    pub fn qi_high(&self, w: f64) -> f64 {
        // 注意：这里需要调用XL::S_aLon_t2等函数，暂时保留接口
        // 这些函数需要从eph.cpp中转换
        0.0
    }
    
    /// 较高精度朔计算
    pub fn so_high(&self, w: f64) -> f64 {
        // 注意：这里需要调用XL::MS_aLon_t2等函数，暂时保留接口
        // 这些函数需要从eph.cpp中转换
        0.0
    }
    
    /// 低精度定朔计算
    pub fn so_low(&self, w: f64) -> f64 {
        let v = 7771.37714500204;
        let mut t = (w + 1.08472) / v;
        
        t -= (-0.0000331 * t * t
            + 0.10976 * (0.785 + 8328.6914 * t).cos()
            + 0.02224 * (0.187 + 7214.0629 * t).cos()
            - 0.03342 * (4.669 + 628.3076 * t).cos()) / v
            + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / 86400.0 / 36525.0;
        
        t * 36525.0 + 8.0 / 24.0
    }
    
    /// 低精度定气计算
    pub fn qi_low(&self, w: f64) -> f64 {
        let v = 628.3319653318;
        let mut t = (w - 4.895062166) / v; // 第一次估算
        
        // 第二次估算
        t -= (53.0 * t * t + 334116.0 * (4.67 + 628.307585 * t).cos() + 2061.0 * (2.678 + 628.3076 * t).cos() * t) / v / 10000000.0;
        
        // 计算平黄经
        let l = 48950621.66 + 6283319653.318 * t + 53.0 * t * t
            + 334166.0 * (4.669257 + 628.307585 * t).cos()
            + 3489.0 * (4.6261 + 1256.61517 * t).cos()
            + 2060.6 * (2.67823 + 628.307585 * t).cos() * t
            - 994.0 - 834.0 * (2.1824 - 33.75705 * t).sin();
        
        t -= (l / 10000000.0 - w) / 628.332 + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / 86400.0 / 36525.0;
        
        t * 36525.0 + 8.0 / 24.0
    }
    
    /// 农历排月序计算
    pub fn calc_y(&mut self, jd: i32) {
        // 清空相关向量
        self.zq.clear();
        self.hs.clear();
        self.dx.clear();
        self.ym.clear();
        self.leap = 0;
        
        // 该年的气计算
        let mut w = ((jd - 355 + 183) as f64 / 365.2422).floor() * 365.2422 + 355.0;
        
        if self.calc(w, QSType::QiType) > jd {
            w -= 365.2422;
        }
        
        // 计算25个节气时刻
        for i in 0..25 {
            let t = self.calc(w + 15.2184 * i as f64, QSType::QiType);
            self.zq.push(t as f64);
        }
        
        // 补算二气
        self.zq_pe1 = self.calc(w - 15.2, QSType::QiType) as f64;
        self.zq_pe2 = self.calc(w - 30.4, QSType::QiType) as f64;
        
        // 求较靠近冬至的朔日
        let mut w_shuo = self.calc(self.zq[0], QSType::SuoType) as f64;
        if w_shuo > self.zq[0] {
            w_shuo -= 29.53;
        }
        
        // 计算该年所有朔
        for i in 0..15 {
            self.hs.push(self.calc(w_shuo + 29.5306 * i as f64, QSType::SuoType));
        }
        
        // 月大小
        for i in 0..14 {
            self.dx.push(self.hs[i + 1] - self.hs[i]);
            self.ym.push(i as i32);
        }
        
        // 确定年份
        let yy = ((self.zq[0] + 10.0 + 180.0) / 365.2422).floor() as i32 + 2000;
        
        // -721年至-104年的后九月及月建问题处理
        if yy >= -721 && yy <= -104 {
            // 这里简化处理，实际需要实现完整的逻辑
            return;
        }
        
        // 无中气置闰法确定闰月
        if self.hs[13] <= self.zq[24] as i32 {
            let mut i = 1;
            while i < 13 && self.hs[i + 1] > self.zq[2 * i] as i32 {
                i += 1;
            }
            
            self.leap = i as i32;
            for j in i..14 {
                self.ym[j] -= 1;
            }
        }
        
        // 名称转换(月建别名)
        for i in 0..14 {
            let dm = self.hs[i] + J2000 as i32;
            let mut mc = self.ym[i] % 12;
            
            // 特殊情况处理
            if dm >= 1724360 && dm <= 1729794 {
                mc = (self.ym[i] + 1) % 12;
            } else if dm >= 1807724 && dm <= 1808699 {
                mc = (self.ym[i] + 1) % 12;
            } else if dm >= 1999349 && dm <= 1999467 {
                mc = (self.ym[i] + 2) % 12;
            } else if dm >= 1973067 && dm <= 1977052 {
                if self.ym[i] % 12 == 0 {
                    mc = 2;
                }
                if self.ym[i] == 2 {
                    mc = 2;
                }
            }
            
            if dm == 1729794 || dm == 1808699 {
                mc = 12;
            }
            
            self.ym[i] = mc;
        }
    }
}

/// 提供一个简单的API封装，供外部调用
pub fn calculate_jie_qi(jd: f64) -> i32 {
    let ssq = SSQ::new();
    ssq.calc(jd, QSType::QiType)
}

pub fn calculate_new_moon(jd: f64) -> i32 {
    let ssq = SSQ::new();
    ssq.calc(jd, QSType::SuoType)
}