
use huturs_core::datetime::*;

#[test]
pub fn test_format_current_timestamp(){
    assert_eq!(format_current("%F %T").is_some(), true);
    assert_eq!(format_current("%F %T").is_some(), true);
    assert_eq!(format_current("%F %T %z").is_some(), true);
}
