use chrono::Local;
use huturs_core::*;

#[test]
pub fn test_format_current_timestamp(){
    assert_eq!(format_current("%F %T").is_some(), true);
    assert_eq!(format_current("%F %T").is_some(), true);
    assert_eq!(format_current("%F %T %z").is_some(), true);
}
