#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
解压SSQ.cpp中的suoS和qiS字符串，并生成Rust代码片段
"""

def jieya(s):
    """解压函数，实现与C++版本相同的逻辑"""
    o = "0000000000"  # 10个0
    o2 = o + o  # 20个0
    
    # 替换字符映射表
    replacements = [
        ("J", "00"),
        ("I", "000"),
        ("H", "0000"),
        ("G", "00000"),
        ("t", "02"),
        ("s", "002"),
        ("r", "0002"),
        ("q", "00002"),
        ("p", "000002"),
        ("o", "0000002"),
        ("n", "00000002"),
        ("m", "000000002"),
        ("l", "0000000002"),
        ("k", "01"),
        ("j", "0101"),
        ("i", "001"),
        ("h", "001001"),
        ("g", "0001"),
        ("f", "00001"),
        ("e", "000001"),
        ("d", "0000001"),
        ("c", "00000001"),
        ("b", "000000001"),
        ("a", "0000000001"),
        ("A", o2 + o2 + o2),  # 60个0
        ("B", o2 + o2 + o),   # 50个0
        ("C", o2 + o2),        # 40个0
        ("D", o2 + o),         # 30个0
        ("E", o2),             # 20个0
        ("F", o)               # 10个0
    ]
    
    # 按顺序替换字符
    result = s
    for old, new in replacements:
        result = result.replace(old, new)
    
    return result

# 原始压缩字符串
suoS = """
EqoFscDcrFpmEsF2DfFideFelFpFfFfFiaipqti1ksttikptikqckstekqttgkqttgkqteksttikptikq2fjstgjqttjkqttgkqt
ekstfkptikq2tijstgjiFkirFsAeACoFsiDaDiADc1AFbBfgdfikijFifegF1FhaikgFag1E2btaieeibggiffdeigFfqDfaiBkF
1kEaikhkigeidhhdiegcFfakF1ggkidbiaedksaFffckekidhhdhdikcikiakicjF1deedFhFccgicdekgiFbiaikcfi1kbFibef
gEgFdcFkFeFkdcfkF1kfkcickEiFkDacFiEfbiaejcFfffkhkdgkaiei1ehigikhdFikfckF1dhhdikcfgjikhfjicjicgiehdik
cikggcifgiejF1jkieFhegikggcikFegiegkfjebhigikggcikdgkaFkijcfkcikfkcifikiggkaeeigefkcdfcfkhkdgkegieid
hijcFfakhfgeidieidiegikhfkfckfcjbdehdikggikgkfkicjicjF1dbidikFiggcifgiejkiegkigcdiegfggcikdbgfgefjF1
kfegikggcikdgFkeeijcfkcikfkekcikdgkabhkFikaffcfkhkdgkegbiaekfkiakicjhfgqdq2fkiakgkfkhfkfcjiekgFebicg
gbedF1jikejbbbiakgbgkacgiejkijjgigfiakggfggcibFifjefjF1kfekdgjcibFeFkijcfkfhkfkeaieigekgbhkfikidfcje
aibgekgdkiffiffkiakF1jhbakgdki1dj1ikfkicjicjieeFkgdkicggkighdF1jfgkgfgbdkicggfggkidFkiekgijkeigfiski
ggfaidheigF1jekijcikickiggkidhhdbgcfkFikikhkigeidieFikggikhkffaffijhidhhakgdkhkijF1kiakF1kfheakgdkif
iggkigicjiejkieedikgdfcggkigieeiejfgkgkigbgikicggkiaideeijkefjeijikhkiggkiaidheigcikaikffikijgkiahi1
hhdikgjfifaakekighie1hiaikggikhkffakicjhiahaikggikhkijF1kfejfeFhidikggiffiggkigicjiekgieeigikggiffig
gkidheigkgfjkeigiegikifiggkidhedeijcfkFikikhkiggkidhh1ehigcikaffkhkiggkidhh1hhigikekfiFkFikcidhh1hit
cikggikhkfkicjicghiediaikggikhkijbjfejfeFhaikggifikiggkigiejkikgkgieeigikggiffiggkigieeigekijcijikgg
ifikiggkideedeijkefkfckikhkiggkidhh1ehijcikaffkhkiggkidhh1hhigikhkikFikfckcidhh1hiaikgjikhfjicjicgie
hdikcikggifikigiejfejkieFhegikggifikiggfghigkfjeijkhigikggifikiggkigieeijcijcikfksikifikiggkidehdeij
cfdckikhkiggkhghh1ehijikifffffkhsFngErD1pAfBoDd1BlEtFqA2AqoEpDqElAEsEeB2BmADlDkqBtC1FnEpDqnEmFsFsAFn
llBbFmDsDiCtDmAB2BmtCgpEplCpAEiBiEoFqFtEqsDcCnFtADnFlEgdkEgmEtEsCtDmADqFtAFrAtEcCqAE1BoFqC1F1DrFtBmF
tAC2ACnFaoCgADcADcCcFfoFtDlAFgmFqBq2bpEoAEmkqnEeCtAE1bAEqgDfFfCrgEcBrACfAAABqAAB1AAClEnFeCtCgAADqDoB
mtAAACbFiAAADsEtBqAB2FsDqpFqEmFsCeDtFlCeDtoEpClEqAAFrAFoCgFmFsFqEnAEcCqFeCtFtEnAEeFtAAEkFnErAABbFkAD
nAAeCtFeAfBoAEpFtAABtFqAApDcCGJ
"""

qiS = """
FrcFs22AFsckF2tsDtFqEtF1posFdFgiFseFtmelpsEfhkF2anmelpFlF1ikrotcnEqEq2FfqmcDsrFor22FgFrcgDscFs22FgEe
FtE2sfFs22sCoEsaF2tsD1FpeE2eFsssEciFsFnmelpFcFhkF2tcnEqEpFgkrotcnEqrEtFermcDsrE222FgBmcmr22DaEfnaF22
2sD1FpeForeF2tssEfiFpEoeFssD1iFstEqFppDgFstcnEqEpFg11FscnEqrAoAF2ClAEsDmDtCtBaDlAFbAEpAAAAAD2FgBiBqo
BbnBaBoAAAAAAAEgDqAdBqAFrBaBoACdAAf1AACgAAAeBbCamDgEifAE2AABa1C1BgFdiAAACoCeE1ADiEifDaAEqAAFe1AcFbcA
AAAAF1iFaAAACpACmFmAAAAAAAACrDaAAADG0
"""

# 清理字符串中的换行符
suoS = suoS.replace('\n', '')
qiS = qiS.replace('\n', '')

# 解压字符串
sb_unpacked = jieya(suoS)
qb_unpacked = jieya(qiS)

print(f"解压后的suoS长度: {len(sb_unpacked)}")
print(f"解压后的qiS长度: {len(qb_unpacked)}")

# 生成Rust代码片段
with open("rust_ssq_fragment.txt", "w") as f:
    f.write("    /// 预计算的定朔修正表\n")
    f.write("    fn precomputed_sb() -> String {\n")
    f.write(f"        String::from(\"{sb_unpacked}\")\n")
    f.write("    }\n\n")
    
    f.write("    /// 预计算的定气修正表\n")
    f.write("    fn precomputed_qb() -> String {\n")
    f.write(f"        String::from(\"{qb_unpacked}\")\n")
    f.write("    }\n")

print("Rust代码片段已生成到 rust_ssq_fragment.txt")

# 生成calc方法代码片段
calc_fragment = f'''
    /// 计算函数
    pub fn calc(&self, jd: f64, qs: QSType) -> i32 {{
        let jd_adj = jd + 2451545.0;
        let mut b = &self.suo_kb;
        let mut pc = 14.0;
        
        // 如果查的是气朔
        if qs == QSType::QiType {{
            b = &self.qi_kb;
            pc = 7.0;
        }}
        
        let f1 = b[0].start_jd - pc;
        let f2 = b.last().unwrap().start_jd - pc;
        let f3 = 2436935.0;
        
        if jd_adj < f1 || jd_adj >= f3 {{
            // 平气朔表中首个之前，使用现代天文算法
            if qs == QSType::QiType {{
                return self.qi_high((jd_adj + pc - 2451259.0) / 365.2422 * 24.0 * PI / 12.0).floor() as i32 + 1;
            }} else {{
                return self.so_high((jd_adj + pc - 2451551.0) / 29.5306 * 2.0 * PI).floor() as i32 + 1;
            }}
        }}
        
        if jd_adj >= f1 && jd_adj < f2 {{
            // 平气或平朔
            let mut i = 0;
            while i + 1 < b.len() && jd_adj + pc >= b[i + 1].start_jd {{
                i += 1;
            }}
            
            let d = b[i].start_jd + b[i].period * ((jd_adj + pc - b[i].start_jd) / b[i].period).floor();
            let mut result = d.floor() + 0.5;
            
            // 特殊修正
            if result == 1683460.0 {{
                result += 1.0;
            }}
            
            return (result - 2451545.0) as i32;
        }}
        
        if jd_adj >= f2 && jd_adj < f3 {{
            // 定气或定朔
            let mut d = 0.0;
            
            if qs == QSType::QiType {{
                d = self.qi_low((jd_adj + pc - 2451259.0) / 365.2422 * 24.0 * PI / 12.0).floor() + 0.5;
                // 找定气修正值
                let pos = ((jd_adj - f2) / 365.2422 * 24.0).floor() as usize;
                if pos < self.qb.len() {{
                    let n_char = self.qb.chars().nth(pos).unwrap_or('0');
                    match n_char {{
                        '1' => return (d + 1.0) as i32,
                        '2' => return (d - 1.0) as i32,
                        _ => (),
                    }}
                }}
            }} else {{
                d = self.so_low((jd_adj + pc - 2451551.0) / 29.5306 * 2.0 * PI).floor() + 0.5;
                // 找定朔修正值
                let pos = ((jd_adj - f2) / 29.5306).floor() as usize;
                if pos < self.sb.len() {{
                    let n_char = self.sb.chars().nth(pos).unwrap_or('0');
                    match n_char {{
                        '1' => return (d + 1.0) as i32,
                        '2' => return (d - 1.0) as i32,
                        _ => (),
                    }}
                }}
            }}
            
            return d as i32;
        }}
        
        0
    }}
'''

with open("calc_fragment.txt", "w") as f:
    f.write(calc_fragment)

print("calc方法代码片段已生成到 calc_fragment.txt")