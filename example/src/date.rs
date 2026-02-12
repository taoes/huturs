use huturs_core::timestamp;
use huturs_core::datetime;

pub fn entry() {
    println!("Current timestamp: {}", timestamp::current_timestamp());
    println!("Current timestamp plus 4 hours: {}", timestamp::add_seconds(timestamp::current_timestamp(), 4 * 60 * 60));
    println!("Current timestamp format: {:?}", datetime::format_current("%F %T").unwrap());

    assert_eq!(datetime::reformat(&String::from("2023-04-01 12:00:00"), &String::from("%F %T"), &String::from("%F")), Some(String::from("2023-04-01")));


}
