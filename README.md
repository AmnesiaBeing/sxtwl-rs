# sxtwl-rs

农历/公历转换与传统历法计算库（支持no_std环境）

## 功能特点

- 公历与农历的相互转换
- 天干地支计算（年、月、日、时）
- 24节气查询与精确时间计算
- 儒略日计算与转换
- 生肖查询
- 支持中文表示的日期和时间
- **no_std支持**，可在嵌入式系统中使用

## 安装

将以下内容添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
sxtwl-rs = "0.1.0"
```

## 使用示例

### 基本的公历与农历转换

```rust
use sxtwl_rs::{LunarDate, SolarDate};

// 创建公历日期：2024年1月1日
let solar = SolarDate {
    year: 2024,
    month: 1,
    day: 1,
    hour: 12,
    minute: 0,
    second: 0.0,
};
// 也可考虑使用new创建，但是校验会更严格（嵌入式下不建议使用）
// let solar = SolarDate::new(2024, 1, 1, 12, 0, 0.0).unwrap();

// 公历转农历
let lunar: LunarDate = solar.into();

// 创建农历日期：2023年腊月初一
let lunar = LunarDate {
    year: 2023,
    month: 12,
    day: 1,
    is_leap: false,
};

// 农历转公历
let solar: SolarDate = lunar.into();
```

### 天干地支与生肖

```rust
use sxtwl_rs::{GanZhi, LunarDate, ShengXiao, SolarDate};
use sxtwl_rs::consts::{GAN, ZHI, SHENGXIAO};

// 创建公历日期
let solar = SolarDate {
    year: 2024,
    month: 2,
    day: 10,
    hour: 12,
    minute: 0,
    second: 0.0,
};

// 获取农历日期
let lunar: LunarDate = solar.into();

// 计算生肖
let shengxiao = ShengXiao::from_lunar_year(lunar.year);

// 计算年干支
let year_ganzhi = GanZhi::from_lunar_year(lunar.year);
```

### 节气查询

```rust
use sxtwl_rs::{JieQi, SolarDate};

// 获取2024年的所有节气
let jieqis = JieQi::get_all_jieqi_by_solar_year(2024);

// 遍历节气信息
for jieqi in jieqis {
    println!("{}: {}", jieqi.jq_index.name(), jieqi.jd.0);
}
```

### 运行示例

克隆仓库后，您可以运行示例代码：

```bash
cargo run --example basic_usage
cargo run --example ganzhi_jieqi
cargo run --example julian_day
```

## 项目结构

- `src/types.rs`：定义核心数据类型（天干、地支、节气、公历日期、农历日期等）
- `src/lunar.rs`：农历计算模块，提供公历与农历的相互转换
- `src/julian.rs`：儒略日计算模块
- `src/ganzhi.rs`：天干地支计算模块
- `src/jieqi.rs`：节气计算模块
- `src/solordate.rs`：公历日期处理模块
- `src/shengxiao.rs`：生肖计算模块

## no_std支持

该库默认支持no_std环境，但依赖`alloc`和`libm`，可以在嵌入式系统中使用。

## 许可证

本项目使用BSD-3-Clause许可证。详情请查看[LICENSE](LICENSE)文件。