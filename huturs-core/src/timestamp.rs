//! 日期工具类模块
//! 提供日期时间处理相关的工具函数

use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳（秒）
///
/// # 返回值
/// 返回从 Unix 纪元（1970-01-01 00:00:00 UTC）到当前时间的秒数
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let ts = timestamp::current_timestamp();
/// assert!(ts > 0);
/// ```
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 获取当前时间戳（毫秒）
///
/// # 返回值
/// 返回从 Unix 纪元（1970-01-01 00:00:00 UTC）到当前时间的毫秒数
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let ts = timestamp::current_timestamp_millis();
/// assert!(ts > 0);
/// ```
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// 格式化时间戳为日期字符串
///
/// # 参数
/// * `timestamp` - 要格式化的时间戳（秒）
///
/// # 返回值
/// 返回时间戳的字符串表示
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let formatted = timestamp::format_timestamp(1234567890);
/// assert_eq!(formatted, "1234567890");
/// ```
pub fn format_timestamp(timestamp: u64) -> String {
    format!("{}", timestamp)
}

/// 获取当前日期字符串
///
/// # 返回值
/// 返回当前时间戳的字符串表示
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let date = timestamp::current_date();
/// assert!(!date.is_empty());
/// ```
pub fn current_date() -> String {
    format!("{}", current_timestamp())
}

/// 计算两个时间戳之间的秒数差
///
/// # 参数
/// * `timestamp1` - 第一个时间戳
/// * `timestamp2` - 第二个时间戳
///
/// # 返回值
/// 返回两个时间戳之间的绝对差值（秒）
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let diff = timestamp::diff_seconds(100, 50);
/// assert_eq!(diff, 50);
/// ```
pub fn diff_seconds(timestamp1: u64, timestamp2: u64) -> i64 {
    (timestamp1 as i64 - timestamp2 as i64).abs()
}

/// 检查时间戳是否在未来
///
/// # 参数
/// * `timestamp` - 要检查的时间戳
///
/// # 返回值
/// 如果时间戳表示的时间在未来，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let future_ts = timestamp::current_timestamp() + 1000;
/// assert!(timestamp::is_future(future_ts));
/// ```
pub fn is_future(timestamp: u64) -> bool {
    timestamp > current_timestamp()
}

/// 检查时间戳是否在过去
///
/// # 参数
/// * `timestamp` - 要检查的时间戳
///
/// # 返回值
/// 如果时间戳表示的时间在过去，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let past_ts = timestamp::current_timestamp() - 1000;
/// assert!(timestamp::is_past(past_ts));
/// ```
pub fn is_past(timestamp: u64) -> bool {
    timestamp < current_timestamp()
}

/// 添加秒数到时间戳
///
/// # 参数
/// * `timestamp` - 原始时间戳
/// * `seconds` - 要添加的秒数
///
/// # 返回值
/// 返回添加指定秒数后的新时间戳
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let ts = timestamp::add_seconds(100, 60);
/// assert_eq!(ts, 160);
/// ```
pub fn add_seconds(timestamp: u64, seconds: u64) -> u64 {
    timestamp + seconds
}

/// 从时间戳减去秒数
///
/// # 参数
/// * `timestamp` - 原始时间戳
/// * `seconds` - 要减去的秒数
///
/// # 返回值
/// 返回减去指定秒数后的时间戳，如果结果为负数则返回 0
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let ts = timestamp::subtract_seconds(100, 30);
/// assert_eq!(ts, 70);
/// ```
pub fn subtract_seconds(timestamp: u64, seconds: u64) -> u64 {
    timestamp.saturating_sub(seconds)
}

/// 获取时间戳对应的分钟数
///
/// # 参数
/// * `timestamp` - 时间戳（秒）
///
/// # 返回值
/// 返回时间戳对应的分钟数
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let minutes = timestamp::get_minutes(120);
/// assert_eq!(minutes, 2);
/// ```
pub fn get_minutes(timestamp: u64) -> u64 {
    timestamp / 60
}

/// 获取时间戳对应的小时数
///
/// # 参数
/// * `timestamp` - 时间戳（秒）
///
/// # 返回值
/// 返回时间戳对应的小时数
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let hours = timestamp::get_hours(7200);
/// assert_eq!(hours, 2);
/// ```
pub fn get_hours(timestamp: u64) -> u64 {
    timestamp / 3600
}

/// 获取时间戳对应的天数
///
/// # 参数
/// * `timestamp` - 时间戳（秒）
///
/// # 返回值
/// 返回时间戳对应的天数
///
/// # 示例
///
/// ```
/// use huturs_core::timestamp;
///
/// let days = timestamp::get_days(172800);
/// assert_eq!(days, 2);
/// ```
pub fn get_days(timestamp: u64) -> u64 {
    timestamp / 86400
}