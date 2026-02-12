use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike};
use huturs_core::*;
#[test]
pub fn test_reformat() {
    let content = String::from("2023-04-01 12:00:00");
    let original_fmt = String::from("%F %T");
    let new_fmt = String::from("%F");

    assert_eq!(
        reformat(&content, &original_fmt, &new_fmt),
        Some(String::from("2023-04-01"))
    );
    assert_ne!(
        reformat(&content, &original_fmt, &new_fmt),
        Some(String::from("2023-04-02"))
    );

    let empty_original_fmt = String::from("%F %T");
    assert_ne!(reformat(&content, &empty_original_fmt, &new_fmt), None);
}

#[test]
pub fn test_datetime_offset() {
    let date_time = Local::now();
    let value = 1;
    let unit = DateTimeOffsetUnit::MINUTES;
    let result = offset(date_time, value, unit);
    assert_ne!(result, date_time);
}

#[test]

pub fn test_between() {
    let date_time1 = Local::now();
    let date_time2 = date_time1 + chrono::Duration::minutes(1);
    assert_ne!(between(&date_time1, &date_time2), 59);
    assert_eq!(between(&date_time1, &date_time2), 60);

    let date_time3 = date_time1 + chrono::Duration::minutes(10);
    assert_eq!(between(&date_time1, &date_time3), 600);
}

#[test]
pub fn test_parse() {
    let content = String::from("2023-04-01 12:34:56");
    let fmt = String::from("%F %T");
    let result = parse(&content, &fmt, Local);
    assert_eq!(result.is_ok(), true);
    let result = result.unwrap();
    assert_eq!(result.year(), 2023);
    assert_eq!(result.month(), 4);
    assert_eq!(result.day(), 1);
    assert_eq!(result.hour(), 12);
    assert_eq!(result.minute(), 34);
    assert_eq!(result.second(), 56);

    let content = String::from("2023-02-29 12:34:56");
    let fmt = String::from("%F %T");
    let result = parse(&content, &fmt, Local);
    assert_eq!(result.is_err(), true);

    let content = String::from("2024-02-29 12:34:56");
    let fmt = String::from("%F %T");
    let result = parse(&content, &fmt, Local);
    assert_eq!(result.is_err(), false);
    assert_eq!(result.is_ok(), true);

    let content = String::from("2024-02-29 00:00:00");
    let fmt = String::from("%Y-%m-%d %H:%M:%S");
    let result = parse(&content, &fmt, Local);
    assert_eq!(result.is_err(), false);
    assert_eq!(result.is_ok(), true);
}

#[test]
pub fn test_end_time_of_day() {
    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_day(&date_time).unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 6);
    assert_eq!(end.day(), 15);
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);
    assert_eq!(end.nanosecond(), 999_999_999);
}

#[test]
pub fn test_start_time_of_day() {
    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_day(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 15);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);
    assert_eq!(start.nanosecond(), 0);
}

#[test]
pub fn test_end_time_of_month() {
    let naive = NaiveDateTime::parse_from_str("2024-02-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_month(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 2);
    assert_eq!(end.day(), 29); // 2024年是闰年，2月有29天
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);
    assert_eq!(end.nanosecond(), 999_999_999);

    // 测试非闰年的2月
    let naive = NaiveDateTime::parse_from_str("2023-02-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_month(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.day(), 28); // 2023年不是闰年，2月有28天
}

#[test]
pub fn test_start_time_of_month() {
    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_month(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 1);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);
    assert_eq!(start.nanosecond(), 0);
}

#[test]
pub fn test_end_time_of_year() {
    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_year(&date_time);
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 12);
    assert_eq!(end.day(), 31);
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);
    assert_eq!(end.nanosecond(), 999_999_999);
}

#[test]
pub fn test_start_time_of_year() {
    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_year(&date_time);
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 1);
    assert_eq!(start.day(), 1);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);
    assert_eq!(start.nanosecond(), 0);
}

#[test]
pub fn test_start_time_of_week() {
    // 测试普通情况：周三
    let naive = NaiveDateTime::parse_from_str("2024-06-12 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 10); // 6月12日是周三，周一是6月10日
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);
    assert_eq!(start.nanosecond(), 0);

    // 测试周一：应该是同一天的开始时间
    let naive = NaiveDateTime::parse_from_str("2024-06-10 15:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 10);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);

    // 测试周日：应该回到周一
    let naive = NaiveDateTime::parse_from_str("2024-06-16 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 10);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
    assert_eq!(start.second(), 0);

    // 测试跨月情况：6月30日是周日
    let naive = NaiveDateTime::parse_from_str("2024-06-30 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 6);
    assert_eq!(start.day(), 24); // 6月24日是周一
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);

    // 测试跨年情况：2024年1月1日是周一
    let naive = NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 1);
    assert_eq!(start.day(), 1);
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);

    // 测试跨年情况：2023年12月31日是周日
    let naive = NaiveDateTime::parse_from_str("2023-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2023);
    assert_eq!(start.month(), 12);
    assert_eq!(start.day(), 25); // 12月25日是周一
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);

    // 测试2月29日（闰年）
    let naive = NaiveDateTime::parse_from_str("2024-02-29 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let start = start_time_of_week(&date_time);
    assert!(start.is_some());
    let start = start.unwrap();
    assert_eq!(start.year(), 2024);
    assert_eq!(start.month(), 2);
    assert_eq!(start.day(), 26); // 2月26日是周一
    assert_eq!(start.hour(), 0);
    assert_eq!(start.minute(), 0);
}

#[test]
pub fn test_end_time_of_week() {
    // 测试普通情况：周三
    let naive = NaiveDateTime::parse_from_str("2024-06-12 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 6);
    assert_eq!(end.day(), 16); // 6月12日是周三，周日是6月16日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);
    assert_eq!(end.nanosecond(), 999_999_999);

    // 测试周一：周日应该是同周的最后一天
    let naive = NaiveDateTime::parse_from_str("2024-06-10 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 6);
    assert_eq!(end.day(), 16);
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);

    // 测试周日：应该就是当天的结束时间
    let naive = NaiveDateTime::parse_from_str("2024-06-16 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 6);
    assert_eq!(end.day(), 16);
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
    assert_eq!(end.second(), 59);

    // 测试跨月情况：5月31日是周五
    let naive = NaiveDateTime::parse_from_str("2024-05-31 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 6);
    assert_eq!(end.day(), 2); // 6月2日是周日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);

    // 测试跨年情况：2024年1月1日是周一
    let naive = NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2024);
    assert_eq!(end.month(), 1);
    assert_eq!(end.day(), 7); // 1月7日是周日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);

    // 测试跨年情况：2023年12月30日是周六
    let naive = NaiveDateTime::parse_from_str("2023-12-30 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2023);
    assert_eq!(end.month(), 12);
    assert_eq!(end.day(), 31); // 12月31日是周日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);

    // 测试跨年情况到下一年：2023年12月31日是周日
    let naive = NaiveDateTime::parse_from_str("2023-12-29 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2023);
    assert_eq!(end.month(), 12);
    assert_eq!(end.day(), 31); // 12月31日是周日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);

    // 测试2月末（非闰年）
    let naive = NaiveDateTime::parse_from_str("2023-02-27 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    let end = end_time_of_week(&date_time);
    assert!(end.is_some());
    let end = end.unwrap();
    assert_eq!(end.year(), 2023);
    assert_eq!(end.month(), 3);
    assert_eq!(end.day(), 5); // 3月5日是周日
    assert_eq!(end.hour(), 23);
    assert_eq!(end.minute(), 59);
}

#[test]
pub fn test_is_am() {
    // 测试上午时间（0:00 - 11:59）
    let naive = NaiveDateTime::parse_from_str("2024-06-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), true);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), true);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 11:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), true);

    // 测试下午时间（12:00 - 23:59）
    let naive = NaiveDateTime::parse_from_str("2024-06-15 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), false);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), false);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_am(&date_time), false);
}

#[test]
pub fn test_is_pm() {
    // 测试上午时间（0:00 - 11:59）
    let naive = NaiveDateTime::parse_from_str("2024-06-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), false);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), false);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 11:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), false);

    // 测试下午时间（12:00 - 23:59）
    let naive = NaiveDateTime::parse_from_str("2024-06-15 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), true);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), true);

    let naive = NaiveDateTime::parse_from_str("2024-06-15 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_time: DateTime<Local> = Local.from_local_datetime(&naive).unwrap();
    assert_eq!(is_pm(&date_time), true);
}
