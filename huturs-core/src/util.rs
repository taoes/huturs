pub fn hex_encoding(str: &str) -> String {
    str.chars()
        .map(|c| format!("{:x}", c as u8))
        .collect::<String>()
}

pub fn hex_decoding(str: &str) -> String {
    (0..str.len())
        .step_by(2)
        .map(|i| {
            let high = str.chars().nth(i).unwrap().to_digit(16).unwrap() as u8;
            let low = str.chars().nth(i + 1).unwrap().to_digit(16).unwrap() as u8;
            (high << 4 | low) as char
        })
        .collect::<String>()
}

// 分页工具

/// 将页码转换为起始和结束索引
///
/// # 参数
/// * `page` - 当前页码（从1开始）
/// * `size` - 每页大小
///
/// # 返回值
/// 返回元组 (起始索引, 结束索引)，索引从0开始
///
/// # 示例
/// ```
/// use huturs_core::util::page_transToStartEnd;
/// let (start, end) = page_transToStartEnd(2, 10);
/// assert_eq!(start, 10);  // 第2页的起始索引
/// assert_eq!(end, 20);    // 第2页的结束索引
/// ```
pub fn page_transToStartEnd(page: i32, size: i32) -> (i32, i32) {
    ((page - 1) * size, page * size)
}

/// 计算总页数
///
/// # 参数
/// * `total` - 总记录数
/// * `size` - 每页大小
///
/// # 返回值
/// 返回总页数，向上取整
///
/// # 示例
/// ```
/// use huturs_core::util::page_totalPage;
/// assert_eq!(page_totalPage(10, 3), 4);   // 10条记录，每页3条，共4页
/// assert_eq!(page_totalPage(9, 3), 3);    // 9条记录，每页3条，共3页
/// assert_eq!(page_totalPage(20, 3), 7);   // 20条记录，每页3条，共7页
/// ```
pub fn page_totalPage(total: i32, size: i32) -> i32 {
    (total + size - 1) / size
}

/// 生成分页彩虹条页码数组
///
/// # 参数
/// * `page_no` - 当前页码（从1开始）
/// * `total_page` - 总页数
/// * `display_count` - 显示的页码数量
///
/// # 返回值
/// 返回要显示的页码数组
///
/// # 示例
/// ```
/// use huturs_core::util::page_rainbow;
/// let result = page_rainbow(5, 20, 6);
/// assert_eq!(result, vec![3, 4, 5, 6, 7, 8]);
/// ```
pub fn page_rainbow(page_no: i32, total_page: i32, display_count: i32) -> Vec<i32> {
    let is_even = (display_count & 1) == 0;
    let left = display_count >> 1;
    let mut right = display_count >> 1;
    let mut length = display_count;

    if is_even {
        right += 1;
    }

    if total_page < display_count {
        length = total_page;
    }

    let mut result = Vec::with_capacity(length as usize);

    if total_page >= display_count {
        if page_no <= left {
            for i in 0..length {
                result.push(i + 1);
            }
        } else if page_no > total_page - right {
            for i in 0..length {
                result.push(i + total_page - display_count + 1);
            }
        } else {
            for i in 0..length {
                result.push(i + page_no - left + if is_even { 1 } else { 0 });
            }
        }
    } else {
        for i in 0..length {
            result.push(i + 1);
        }
    }

    result
}
