//! 资源构建主模块

mod modules;
mod progress;

use anyhow::Result;
use progress::ProgressTracker;

/// 运行完整的资源构建流程
pub fn run() -> Result<()> {
    // 配置增量编译触发
    configure_incremental_build();

    let mut progress = ProgressTracker::new();

    progress.start_stage("处理 气朔 数据");
    modules::qishuo::generate_qishuo_data()?;
    progress.complete_stage();

    progress.start_stage("处理 闰月 数据");
    modules::leap_month::generate_leap_year_data()?;
    progress.complete_stage();

    #[cfg(feature = "holiday")]
    {
        progress.start_stage("处理 法定节假日 数据");
        modules::holiday::generate_holidays_data()?;
        progress.complete_stage();
    }

    #[cfg(feature = "rabbyung")]
    {
        progress.start_stage("处理 RabByung 数据");
        modules::rab_byung_month_days::generate_rab_byung_data()?;
        progress.complete_stage();
    }

    #[cfg(feature = "god")]
    {
        progress.start_stage("处理 神煞 数据");
        modules::day_god::generate_day_god_data()?;
        progress.complete_stage();
    }

    progress.finish_build();
    Ok(())
}

/// 配置增量编译触发
fn configure_incremental_build() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=builder/");
}
