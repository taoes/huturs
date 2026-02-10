//! 日期时间工具类模块
//! 提供日期时间处理相关的工具函数，包括格式化、解析和偏移计算

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Timelike};
use std::fmt::Display;

/// 格式化当前时间为指定格式的字符串
///
/// # 参数
/// * `fmt` - 日期时间格式字符串，遵循 `chrono` 的格式规范
///
/// # 返回值
/// 返回格式化后的字符串，如果格式化失败则返回 `None`
///
/// # 示例
/// ```
/// use huturs_core::datetime;
/// let formatted = datetime::format_current_timestamp("%Y-%m-%d %H:%M:%S");
/// assert!(formatted.is_some());
/// ```
pub fn format_current_timestamp(fmt: &str) -> Option<String> {
    format_date(&Local::now(), fmt)
}

/// 格式化指定的日期时间为字符串
///
/// # 参数
/// * `date` - 要格式化的日期时间对象
/// * `fmt` - 日期时间格式字符串，遵循 `chrono` 的格式规范
///
/// # 返回值
/// 返回格式化后的字符串，如果格式化失败则返回 `None`
///
/// # 示例
/// ```
/// use chrono::Local;
/// use huturs_core::datetime;
/// let now = Local::now();
/// let formatted = datetime::format_date(&now, "%Y-%m-%d");
/// assert!(formatted.is_some());
/// ```
pub fn format_date<T>(date: &DateTime<T>, fmt: &str) -> Option<String>
where
    T: TimeZone,
    <T as TimeZone>::Offset: Display,
{
    Some(date.format(fmt).to_string())
}

/// 将日期时间字符串从一种格式重新格式化为另一种格式
///
/// # 参数
/// * `content` - 原始日期时间字符串
/// * `original_fmt` - 原始字符串的格式
/// * `new_fmt` - 目标格式
///
/// # 返回值
/// 返回重新格式化后的字符串，如果解析失败则返回 `None`
///
/// # 示例
/// ```
/// use huturs_core::datetime;
/// let result = datetime::reformat(
///     &"2024-01-01 12:00:00".to_string(),
///     &"%Y-%m-%d %H:%M:%S".to_string(),
///     &"%Y/%m/%d".to_string()
/// );
/// assert_eq!(result, Some("2024/01/01".to_string()));
/// ```
pub fn reformat(content: &String, original_fmt: &String, new_fmt: &String) -> Option<String> {
    match NaiveDateTime::parse_from_str(content, original_fmt) {
        Ok(date) => Some(date.format(new_fmt).to_string()),
        Err(_) => None,
    }
}

/// 日期时间偏移单位枚举
///
/// 用于指定在进行日期时间偏移计算时使用的时间单位
pub enum DateTimeOffsetUnit {
    /// 秒
    SECOND,
    /// 分钟
    MINUTES,
    /// 小时
    HOURS,
    /// 天
    DAYS,
}

/// 对日期时间进行指定单位的偏移计算
///
/// # 参数
/// * `date_time` - 原始日期时间
/// * `value` - 偏移量，正数表示向前，负数表示向后
/// * `unit` - 偏移的时间单位
///
/// # 返回值
/// 返回偏移后的新日期时间对象
///
/// # 示例
/// ```
/// use chrono::Local;
/// use huturs_core::datetime::{DateTimeOffsetUnit, offset};
/// let now = Local::now();
/// let future = offset(now, 1, DateTimeOffsetUnit::HOURS);
/// // future 是 now 之后 1 小时的时间
/// ```
pub fn offset<T: chrono::TimeZone>(
    date_time: DateTime<T>,
    value: i64,
    unit: DateTimeOffsetUnit,
) -> DateTime<T> {
    match unit {
        DateTimeOffsetUnit::SECOND => date_time + chrono::Duration::seconds(value),
        DateTimeOffsetUnit::MINUTES => date_time + chrono::Duration::minutes(value),
        DateTimeOffsetUnit::HOURS => date_time + chrono::Duration::hours(value),
        DateTimeOffsetUnit::DAYS => date_time + chrono::Duration::days(value),
    }
}

pub fn between<T: chrono::TimeZone>(date_time1: &DateTime<T>, date_time2: &DateTime<T>) -> i64 {
    (date_time2.naive_local() - date_time1.naive_local()).num_seconds()
}
