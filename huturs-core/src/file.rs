//! 文件操作工具类模块
//! 提供文件读写相关的工具函数

use crate::is_blank;
use std::fs;
use std::io::{Error, Write};
use std::path::PathBuf;

/// 读取文件内容
///
/// # 参数
/// * `path` - 文件路径
///
/// # 返回值
/// 返回 `Result<String, String>`，成功时包含文件内容，失败时包含错误信息
///
/// # 示例
///
/// ```
/// use huturs_core::file;
///
/// match file::read_file("test.txt") {
///     Ok(content) => println!("文件内容: {}", content),
///     Err(e) => eprintln!("读取失败: {}", e),
/// }
/// ```
pub fn read_file(path: &str) -> Result<String, String> {
    if is_blank(path) {
        return Err(format!("File {} is blank", path));
    }
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// 写入内容到文件
///
/// # 参数
/// * `path` - 文件路径
/// * `contents` - 要写入的内容
///
/// # 返回值
/// 返回 `Result<(), String>`，成功时返回 `Ok(())`，失败时包含错误信息
///
/// # 示例
///
/// ```
/// use huturs_core::file;
///
/// match file::write_file("test.txt", "Hello, world!") {
///     Ok(()) => println!("写入成功"),
///     Err(e) => eprintln!("写入失败: {}", e),
/// }
/// ```
pub fn write_file(path: &str, contents: &str) -> Result<(), Error> {
    if is_blank(path) {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("File {} is blank", path),
        ));
    }
    fs::write(&path, contents)
}

/// 追加内容到文件末尾
///
/// # 参数
/// * `path` - 文件路径
/// * `contents` - 要追加的内容
///
/// # 返回值
/// 返回 `Result<usize, Error>`，成功时返回写入的字节数，失败时包含错误信息
///
/// # 注意
/// 如果文件不存在，将创建新文件。如果文件已存在，内容将追加到文件末尾
///
/// # 示例
///
/// ```
/// use huturs_core::file;
///
/// match file::append_file("test.txt", "\nAdditional content") {
///     Ok(bytes) => println!("写入了 {} 字节", bytes),
///     Err(e) => eprintln!("追加失败: {}", e),
/// }
/// ```
pub fn append_file(path: &str, contents: &str) -> Result<usize, Error> {
    if is_blank(path) {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("File {} is blank", path),
        ));
    }
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .and_then(|mut file| file.write(contents.as_bytes()))
}

/// 删除指定文件
///
/// # 参数
/// * `path` - 要删除的文件路径
///
/// # 返回值
/// 返回 `Result<(), Error>`，成功时返回 `Ok(())`，失败时包含错误信息
///
/// # 注意
/// 此函数只能删除文件，不能删除目录。如果文件不存在，操作将失败
///
/// # 示例
///
/// ```
/// use huturs_core::file;
///
/// match file::delete_file("test.txt") {
///     Ok(()) => println!("删除成功"),
///     Err(e) => eprintln!("删除失败: {}", e),
/// }
/// ```
pub fn delete_file(path: &str) -> Result<(), Error> {
    if is_blank(path) {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("File {} is blank", path),
        ));
    }
    fs::remove_file(&path)
}

/// 读取目录下的所有文件和子目录
///
/// # 参数
/// * `path` - 目录路径
///
/// # 返回值
/// 返回 `Result<Vec<PathBuf>, Error>`，成功时包含目录下所有条目的路径向量，失败时包含错误信息
///
/// # 注意
/// 此函数只返回直接子项，不会递归遍历子目录
///
/// # 示例
///
/// ```
/// use huturs_core::file;
///
/// match file::read_dirs("./src") {
///     Ok(entries) => {
///         for entry in entries {
///             println!("发现: {:?}", entry);
///         }
///     }
///     Err(e) => eprintln!("读取目录失败: {}", e),
/// }
/// ```
pub fn read_dirs(path: &str) -> Result<Vec<PathBuf>, Error> {
    if is_blank(path) {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("File {} is blank", path),
        ));
    }
    fs::read_dir(path)?
        .map(|entry| entry.map(|e| e.path()))
        .collect()
}