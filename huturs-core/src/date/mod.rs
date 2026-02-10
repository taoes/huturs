//! 日期工具类模块
//! 提供日期时间处理相关的工具函数

use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒）
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 获取当前时间戳（毫秒）
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// 格式化时间戳为日期字符串
pub fn format_timestamp(timestamp: u64) -> String {
    format!("{}", timestamp)
}

/// 获取当前日期字符串
pub fn current_date() -> String {
    format!("{}", current_timestamp())
}

/// 计算两个时间戳之间的秒数差
pub fn diff_seconds(timestamp1: u64, timestamp2: u64) -> i64 {
    (timestamp1 as i64 - timestamp2 as i64).abs()
}

/// 检查时间戳是否在未来
pub fn is_future(timestamp: u64) -> bool {
    timestamp > current_timestamp()
}

/// 检查时间戳是否在过去
pub fn is_past(timestamp: u64) -> bool {
    timestamp < current_timestamp()
}

/// 添加秒数到时间戳
pub fn add_seconds(timestamp: u64, seconds: u64) -> u64 {
    timestamp + seconds
}

/// 从时间戳减去秒数
pub fn subtract_seconds(timestamp: u64, seconds: u64) -> u64 {
    timestamp.saturating_sub(seconds)
}

/// 获取时间戳的分钟数
pub fn get_minutes(timestamp: u64) -> u64 {
    timestamp / 60
}

/// 获取时间戳的小时数
pub fn get_hours(timestamp: u64) -> u64 {
    timestamp / 3600
}

/// 获取时间戳的天数
pub fn get_days(timestamp: u64) -> u64 {
    timestamp / 86400
}
