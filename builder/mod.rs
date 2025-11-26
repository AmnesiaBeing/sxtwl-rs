//! 资源构建主模块

mod modules;
mod progress;

use anyhow::Result;
use progress::ProgressTracker;

/// 运行完整的资源构建流程
pub fn run() -> Result<()> {
    let mut progress = ProgressTracker::new();

    progress.start_stage("处理 气朔 数据");
    modules::qishuo::generate_qishuo_data()?;
    progress.complete_stage();

    progress.start_stage("处理 闰月 数据");
    modules::leap_month::generate_leap_year_data()?;
    progress.complete_stage();

    progress.start_stage("处理 法定节假日 数据");
    modules::holiday::generate_holidays_data()?;
    progress.complete_stage();

    progress.finish_build();
    Ok(())
}
