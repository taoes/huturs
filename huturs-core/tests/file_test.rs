use huturs_core::file;

#[test]
pub fn test_read_file() {
    file::read_file("tests/data/test.txt").unwrap();
}
