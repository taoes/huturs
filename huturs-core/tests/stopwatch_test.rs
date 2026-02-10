use chrono::Local;
use huturs_core::*;

#[test]
fn test_stopwatch() {
    let mut sw = StopWatch::new();
    sw.start();
    std::thread::sleep(std::time::Duration::from_secs(1));
    sw.stop();
    assert!(sw.elapsed().as_secs() >= 1);
    println!("{}", sw.elapsed().as_secs());
}
