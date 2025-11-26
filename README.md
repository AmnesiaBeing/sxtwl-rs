# sxtwl-rs

农历/公历转换与传统历法计算库（支持no_std环境）

## 项目简介

`sxtwl-rs` 是一个功能全面的农历/公历转换与传统历法计算库，使用Rust语言开发，支持`no_std`环境。该库提供了丰富的传统历法功能，包括农历公历互转、节气计算、天干地支、星座、六曜、二十八宿等多种传统历法元素的计算。

## 主要功能

- 农历/公历日期互转
- 节气计算
- 天干地支计算（年、月、日、时）
- 六十甲子循环
- 星期计算
- 儒略日计算
- 星座查询

## 可选特性

通过features可以启用以下额外功能：

### 八字相关
- `eight-char`: 启用八字计算功能
  - `eight-char-default-provider`: 八字默认提供器
  - `eight-char-lunar-sect2-provider`: 八字农历分节提供器

### 童限相关
- `child-limit`: 启用童限计算
  - `child-limit-default-provider`: 童限默认提供器
  - `child-limit-china95-provider`: 童限中国95提供器
  - `child-limit-lunar-sect1-provider`: 童限农历分节1提供器
  - `child-limit-lunar-sect2-provider`: 童限农历分节2提供器

### 节假日相关
- `festival`: 节假日（固定的，单一的公历、农历节日）
- `holiday`: 节假日（法定假日、调休等）

### 其他传统历法元素
- `rabbyung`: 藏历
- `dog`: 三伏
- `god`: 神煞
- `peng_zu`: 彭祖百忌
- `phenology`: 物候
- `fetus`: 胎日
- `nine`: 数九
- `plumrain`: 梅雨
- `miniren`: 小六壬

### 星曜相关
- `star-nine`: 北斗九星
- `star-seven`: 七曜
- `star-six`: 六曜
- `star-ten`: 十神
- `star-twelve`: 黄道黑道十二神
- `star-twenty-eight`: 二十八宿

## 使用

在您的`Cargo.toml`文件中添加依赖：

```toml
[dependencies]
sxtwl-rs = {
    version = "0.1.0",
    features = [
        # 根据需要选择启用的特性
        "eight-char",
        "festival",
        "holiday",
        "dog",
        "star-six"
    ]
}
```

## 基本使用

### 公历转农历

```rust
use sxtwl_rs::solar::SolarDay;

// 创建公历日期
let solar_day = SolarDay::from_ymd(2024, 1, 1).unwrap();

// 转换为农历
let lunar_day = solar_day.get_lunar_day();

// 获取农历信息
println!("农历: {}-{}-{}", 
         lunar_day.get_year(), 
         lunar_day.get_month(), 
         lunar_day.get_day());
println!("是否闰月: {}", lunar_day.get_month().is_leap());
```

### 农历转公历

```rust
use sxtwl_rs::lunar::LunarDay;

// 创建农历日期（2023年腊月初一，非闰月）
let lunar_day = LunarDay::from_ymd(2023, 12, 1, false).unwrap();

// 转换为公历
let solar_day = lunar_day.get_solar_day();

// 获取公历信息
println!("公历: {}-{}-{}", 
         solar_day.get_year(), 
         solar_day.get_month(), 
         solar_day.get_day());
```

### 获取节气

```rust
use sxtwl_rs::solar::SolarYear;

// 获取2024年的所有节气
let solar_year = SolarYear::from_year(2024);
for term in solar_year.get_terms() {
    println!("节气: {}, 日期: {}-{}-{}", 
             term.get_name(), 
             term.get_solar_time().get_solar_day().get_year(), 
             term.get_solar_time().get_solar_day().get_month(), 
             term.get_solar_time().get_solar_day().get_day());
}
```

### 获取天干地支

```rust
use sxtwl_rs::solar::SolarDay;

// 创建公历日期
let solar_day = SolarDay::from_ymd(2024, 1, 1).unwrap();

// 获取六十甲子日
let sixty_cycle_day = solar_day.get_sixty_cycle_day();
println!("六十甲子日: {}", sixty_cycle_day.get_name());

// 获取天干
let heaven_stem = sixty_cycle_day.get_heaven_stem();
println!("天干: {}", heaven_stem.get_name());

// 获取地支
let earth_branch = sixty_cycle_day.get_earth_branch();
println!("地支: {}", earth_branch.get_name());
```

## 支持的环境

- **标准环境**：支持标准Rust环境
- **no_std环境**：通过设置`#![no_std]`，支持嵌入式设备等资源受限环境

## 许可证

本项目采用MIT许可证。详见LICENSE文件。

## 致谢

- 感谢许剑伟老师分享的寿星天文历，本项目节气算法引自 https://github.com/sxwnl/sxwnl

- 感谢 6tail/tyme4rs: https://github.com/6tail/tyme4rs
  - 本项目主要基于上述Rust源码，进行了no_std环境的适配