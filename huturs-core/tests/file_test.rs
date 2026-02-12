use huturs_core::{current_timestamp, file, read_dirs};

#[test]
pub fn test_write_and_read_file() {
    let content = format!("{}", current_timestamp());
    let file_path = format!(
        "{}/{}.rs",
        std::env::temp_dir().display(),
        current_timestamp()
    );
    match file::write_file(&file_path, &content) {
        Ok(()) => (),
        Err(e) => assert!(false, "Failed to write file: {}", e),
    };

    let content_from_file = file::read_file(&file_path).unwrap_or_else(|_| "".to_string());
    assert_ne!(content_from_file, "");
    assert_eq!(content_from_file, content);

    match file::append_file(&file_path, "123456") {
        Ok(size) => assert_eq!(size, 6),
        Err(e) => assert!(false, "Failed to delete file: {}", e),
    };

    match file::delete_file(&file_path) {
        Ok(()) => (),
        Err(e) => assert!(false, "Failed to delete file: {}", e),
    };
}

#[test]
pub fn test_write_and_read_dir() {
    let dir_path = std::env::temp_dir().display().to_string();
    match read_dirs(dir_path.as_str()) {
        Ok(vec) => {
            for x in vec.iter() {
                println!("{}", x.display());
            }
        }
        Err(e) => assert!(false, "Failed to read dir: {}", e),
    }
}
