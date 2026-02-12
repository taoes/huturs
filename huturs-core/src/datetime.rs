//! 日期时间工具类模块
//! 提供日期时间处理相关的工具函数，包括格式化、解析和偏移计算

use chrono::{DateTime, Datelike, Local, NaiveDateTime, NaiveTime, TimeZone, Timelike, Weekday};
use std::fmt::Display;
use std::io::Error;

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
/// let formatted = datetime::format_current("%Y-%m-%d %H:%M:%S");
/// assert!(formatted.is_some());
/// ```
pub fn format_current(fmt: &str) -> Option<String> {
    format(&Local::now(), fmt)
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
/// let formatted = datetime::format(&now, "%Y-%m-%d");
/// assert!(formatted.is_some());
/// ```
pub fn format<T>(date: &DateTime<T>, fmt: &str) -> Option<String>
where
    T: TimeZone,
    <T as TimeZone>::Offset: Display,
{
    Some(date.format(fmt).to_string())
}

/// 将日期时间字符串解析为 DateTime 对象
///
/// # 参数
/// * `content` - 日期时间字符串
/// * `fmt` - 日期时间格式字符串，遵循 `chrono` 的格式规范
/// * `timezone` - 时区实例，如 `Local` 或 `Utc`
///
/// # 返回值
/// 返回解析后的 DateTime 对象，如果解析失败则返回错误
///
/// # 示例
/// ```
/// use chrono::Local;
/// use huturs_core::datetime;
/// let result = datetime::parse(
///     &"2024-01-01 12:00:00".to_string(),
///     &"%Y-%m-%d %H:%M:%S".to_string(),
///     Local
/// );
/// assert!(result.is_ok());
/// ```
pub fn parse<T>(content: &String, fmt: &String, timezone: T) -> Result<DateTime<T>, Error>
where
    T: TimeZone,
    <T as TimeZone>::Offset: Display,
{
    NaiveDateTime::parse_from_str(content, fmt)
        .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Parse error"))
        .and_then(|naive_date| {
            naive_date
                .and_local_timezone(timezone)
                .single()
                .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "Invalid date time"))
        })
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

pub fn is_am<T>(date_time: &DateTime<T>) -> bool
where
    T: TimeZone,
{
    date_time.hour() < 12
}

pub fn is_pm<T>(date_time: &DateTime<T>) -> bool
where
    T: TimeZone,
{
    date_time.hour() >= 12
}

pub fn end_time_of_day<T>(date_time: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    date_time
        .clone()
        .with_hour(23)
        .and_then(|dt| dt.with_minute(59))
        .and_then(|dt| dt.with_second(59))
        .and_then(|dt| dt.with_nanosecond(999_999_999))
}
pub fn start_time_of_day<T>(date_time: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    date_time
        .clone()
        .with_hour(0)
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
}

pub fn end_time_of_week<T>(date_time: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    let days_until_sunday = 6 - date_time.weekday().num_days_from_monday();
    let end_of_week: DateTime<T> = date_time.clone() + chrono::Duration::days(days_until_sunday as i64);
    end_of_week
        .with_hour(23)
        .and_then(|dt: DateTime<T>| dt.with_minute(59))
        .and_then(|dt: DateTime<T>| dt.with_second(59))
        .and_then(|dt: DateTime<T>| dt.with_nanosecond(999_999_999))
}
pub fn start_time_of_week<T>(date_time: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    let days_since_monday = date_time.weekday().num_days_from_monday();
    let start_of_week = date_time.clone() - chrono::Duration::days(days_since_monday as i64);
    start_of_week
        .with_hour(0)
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
}

pub fn end_time_of_month<T>(date: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    date.clone()
        .with_day(date.num_days_in_month() as u32)
        .and_then(|dt| dt.with_hour(23))
        .and_then(|dt| dt.with_minute(59))
        .and_then(|dt| dt.with_second(59))
        .and_then(|dt| dt.with_nanosecond(999_999_999))
}
pub fn start_time_of_month<T>(date: &DateTime<T>) -> Option<DateTime<T>>
where
    T: TimeZone,
{
    date.clone()
        .with_day(1)
        .and_then(|dt| dt.with_hour(0))
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
}

/// 获取给定日期时间所在年份的结束时间
///
/// # 参数
/// * `date_time` - 日期时间对象
///
/// # 返回值
/// 返回该年份的最后一天（12月31日）的最后一刻（23:59:59.999999999）
///
/// # 示例
/// ```
/// use chrono::{Datelike, Local, NaiveDateTime, TimeZone};
/// use huturs_core::datetime;
/// let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let date = Local.from_local_datetime(&naive).unwrap();
/// let end = datetime::end_time_of_year(&date);
/// assert_eq!(end.year(), 2024);
/// assert_eq!(end.month(), 12);
/// assert_eq!(end.day(), 31);
/// ```
pub fn end_time_of_year<T>(date_time: &DateTime<T>) -> DateTime<T>
where
    T: TimeZone,
{
    date_time
        .with_month(12)
        .and_then(|dt| dt.with_day(31))
        .and_then(|dt| dt.with_hour(23))
        .and_then(|dt| dt.with_minute(59))
        .and_then(|dt| dt.with_second(59))
        .and_then(|dt| dt.with_nanosecond(999_999_999))
        .expect("Failed to calculate end of year")
}
pub fn start_time_of_year<T>(date_time: &DateTime<T>) -> DateTime<T>
where
    T: TimeZone,
{
    date_time
        .clone()
        .with_month(1)
        .and_then(|dt| dt.with_day(1))
        .and_then(|dt| dt.with_hour(0))
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
        .expect("Failed to calculate end of year")
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
pub fn offset<T: TimeZone>(
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

pub fn between<T: TimeZone>(date_time1: &DateTime<T>, date_time2: &DateTime<T>) -> i64 {
    (date_time2.naive_local() - date_time1.naive_local()).num_seconds()
}