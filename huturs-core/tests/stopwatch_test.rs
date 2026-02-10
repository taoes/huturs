use chrono::Local;
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
    assert_ne!(
        reformat(&content, &empty_original_fmt, &new_fmt),
        None
    );
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
