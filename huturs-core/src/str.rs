//! 字符串工具类模块
//! 提供各种字符串操作的工具函数

/// 检查字符串是否为空
///
/// # 参数
/// * `s` - 要检查的字符串
///
/// # 返回值
/// 如果字符串长度为 0，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::is_empty(""));
/// assert!(!str::is_empty("hello"));
/// ```
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// 检查字符串是否只包含空白字符
///
/// # 参数
/// * `s` - 要检查的字符串
///
/// # 返回值
/// 如果字符串只包含空白字符（空格、制表符、换行符等）或为空，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::is_blank(""));
/// assert!(str::is_blank("   "));
/// assert!(!str::is_blank("  hello  "));
/// ```
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

/// 将字符串转换为大写
///
/// # 参数
/// * `s` - 要转换的字符串
///
/// # 返回值
/// 返回转换为大写的字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::to_uppercase("hello"), "HELLO");
/// assert_eq!(str::to_uppercase("Hello"), "HELLO");
/// ```
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

/// 将字符串转换为小写
///
/// # 参数
/// * `s` - 要转换的字符串
///
/// # 返回值
/// 返回转换为小写的字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::to_lowercase("HELLO"), "hello");
/// assert_eq!(str::to_lowercase("Hello"), "hello");
/// ```
pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

/// 去除字符串首尾空白字符
///
/// # 参数
/// * `s` - 要处理的字符串
///
/// # 返回值
/// 返回去除首尾空白字符后的字符串切片
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::trim("  hello  "), "hello");
/// assert_eq!(str::trim("\t\nhello\n\t"), "hello");
/// ```
pub fn trim(s: &str) -> &str {
    s.trim()
}

/// 去除字符串开头空白字符
///
/// # 参数
/// * `s` - 要处理的字符串
///
/// # 返回值
/// 返回去除开头空白字符后的字符串切片
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::trim_start("  hello  "), "hello  ");
/// assert_eq!(str::trim_start("\thello"), "hello");
/// ```
pub fn trim_start(s: &str) -> &str {
    s.trim_start()
}

/// 去除字符串结尾空白字符
///
/// # 参数
/// * `s` - 要处理的字符串
///
/// # 返回值
/// 返回去除结尾空白字符后的字符串切片
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::trim_end("  hello  "), "  hello");
/// assert_eq!(str::trim_end("hello\t"), "hello");
/// ```
pub fn trim_end(s: &str) -> &str {
    s.trim_end()
}

/// 反转字符串
///
/// # 参数
/// * `s` - 要反转的字符串
///
/// # 返回值
/// 返回反转后的新字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::reverse("hello"), "olleh");
/// assert_eq!(str::reverse("rust"), "tsur");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// 检查字符串是否包含指定子串
///
/// # 参数
/// * `s` - 要搜索的字符串
/// * `pattern` - 要查找的子串
///
/// # 返回值
/// 如果字符串包含指定的子串，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::contains("hello world", "world"));
/// assert!(!str::contains("hello", "xyz"));
/// ```
pub fn contains(s: &str, pattern: &str) -> bool {
    s.contains(pattern)
}

/// 检查字符串是否以指定前缀开头
///
/// # 参数
/// * `s` - 要检查的字符串
/// * `prefix` - 要检查的前缀
///
/// # 返回值
/// 如果字符串以指定的前缀开头，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::starts_with("hello", "he"));
/// assert!(!str::starts_with("hello", "wo"));
/// ```
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// 检查字符串是否以指定后缀结尾
///
/// # 参数
/// * `s` - 要检查的字符串
/// * `suffix` - 要检查的后缀
///
/// # 返回值
/// 如果字符串以指定的后缀结尾，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::ends_with("hello", "lo"));
/// assert!(!str::ends_with("hello", "he"));
/// ```
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// 获取字符串长度（字节数）
///
/// # 参数
/// * `s` - 要测量的字符串
///
/// # 返回值
/// 返回字符串的字节长度
///
/// # 注意
/// 此函数返回的是字节数而非字符数。对于多字节字符（如中文），字符数可能不等于字节数
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::length("hello"), 5);
/// assert_eq!(str::length("你好"), 6); // 中文字符占 3 个字节
/// ```
pub fn length(s: &str) -> usize {
    s.len()
}

/// 检查字符串是否为空字符串
///
/// # 参数
/// * `s` - 要检查的字符串
///
/// # 返回值
/// 如果字符串长度为 0，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::is_empty_str(""));
/// assert!(!str::is_empty_str("hello"));
/// ```
pub fn is_empty_str(s: &str) -> bool {
    s.is_empty()
}

/// 检查字符串是否不为空
///
/// # 参数
/// * `s` - 要检查的字符串
///
/// # 返回值
/// 如果字符串长度大于 0，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert!(str::is_not_empty("hello"));
/// assert!(!str::is_not_empty(""));
/// ```
pub fn is_not_empty(s: &str) -> bool {
    !s.is_empty()
}

/// 替换字符串中的子串
///
/// # 参数
/// * `s` - 原始字符串
/// * `from` - 要被替换的子串
/// * `to` - 替换后的字符串
///
/// # 返回值
/// 返回替换后的新字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::replace("hello world", "world", "rust"), "hello rust");
/// assert_eq!(str::replace("aaa", "a", "b"), "bbb");
/// ```
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// 分割字符串
///
/// # 参数
/// * `s` - 要分割的字符串
/// * `delimiter` - 分隔符
///
/// # 返回值
/// 返回分割后的字符串向量
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// let parts = str::split("a,b,c", ",");
/// assert_eq!(parts, vec!["a", "b", "c"]);
/// ```
pub fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

/// 连接字符串数组
///
/// # 参数
/// * `strings` - 要连接的字符串切片
/// * `delimiter` - 分隔符
///
/// # 返回值
/// 返回连接后的字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// let result = str::join(&["a", "b", "c"], ",");
/// assert_eq!(result, "a,b,c");
/// ```
pub fn join(strings: &[&str], delimiter: &str) -> String {
    strings.join(delimiter)
}

/// 重复字符串指定次数
///
/// # 参数
/// * `s` - 要重复的字符串
/// * `count` - 重复次数
///
/// # 返回值
/// 返回重复后的新字符串
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::repeat("abc", 3), "abcabcabc");
/// assert_eq!(str::repeat("a", 5), "aaaaa");
/// ```
pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}

/// 获取子字符串
///
/// # 参数
/// * `s` - 原始字符串
/// * `start` - 起始位置（包含）
/// * `end` - 结束位置（不包含）
///
/// # 返回值
/// 返回从 `start` 到 `end` 位置的子字符串切片
///
/// # 注意
/// 此函数使用字节索引，对于多字节字符可能需要谨慎使用
///
/// # 示例
///
/// ```
/// use huturs_core::str;
///
/// assert_eq!(str::substring("hello", 1, 4), "ell");
/// assert_eq!(str::substring("rust", 0, 2), "ru");
/// ```
pub fn substring(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]
}