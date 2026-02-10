//! 字符串工具类模块
//! 提供各种字符串操作的工具函数

/// 检查字符串是否为空
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// 检查字符串是否只包含空白字符
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// 将字符串转换为大写
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// 将字符串转换为小写
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// 去除字符串首尾空白字符
pub fn trim(s: &str) -> &str {
    s.trim()
}

/// 去除字符串开头空白字符
pub fn trim_start(s: &str) -> &str {
    s.trim_start()
}

/// 去除字符串结尾空白字符
pub fn trim_end(s: &str) -> &str {
    s.trim_end()
}

/// 反转字符串
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// 检查字符串是否包含指定子串
pub fn contains(s: &str, pattern: &str) -> bool {
    s.contains(pattern)
}

/// 检查字符串是否以指定前缀开头
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// 检查字符串是否以指定后缀结尾
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// 获取字符串长度
pub fn length(s: &str) -> usize {
    s.len()
}

/// 检查字符串是否为空字符串
pub fn is_empty_str(s: &str) -> bool {
    s.is_empty()
}

/// 检查字符串是否不为空
pub fn is_not_empty(s: &str) -> bool {
    !s.is_empty()
}

/// 替换字符串中的子串
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// 分割字符串
pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

/// 连接字符串数组
pub fn join(strings: &[&str], delimiter: &str) -> String {
    strings.join(delimiter)
}

/// 重复字符串
pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}


