use huturs_core::util::*;

#[test]
pub fn test_hex_encode() {
    let hex_value = hex_encoding("hello, world!");
    assert_eq!(hex_value, "68656c6c6f2c20776f726c6421");

    let hex_value = hex_encoding("hello, hutuRs!");
    assert_eq!(hex_value, "68656c6c6f2c2068757475527321");
}

#[test]
pub fn test_hex_decode() {
    let raw_value = hex_decoding("68656c6c6f2c20776f726c6421");
    assert_eq!(raw_value.to_string(), "hello, world!");

    let raw_value = hex_decoding("68656c6c6f2c2068757475527321");
    assert_eq!(raw_value.to_string(), "hello, hutuRs!");
}

#[test]
pub fn test_page_transToStartEnd() {
    // 测试第1页
    let (start, end) = page_transToStartEnd(1, 10);
    assert_eq!(start, 0);
    assert_eq!(end, 10);

    // 测试第2页
    let (start, end) = page_transToStartEnd(2, 10);
    assert_eq!(start, 10);
    assert_eq!(end, 20);

    // 测试第5页
    let (start, end) = page_transToStartEnd(5, 20);
    assert_eq!(start, 80);
    assert_eq!(end, 100);
}

#[test]
pub fn test_page_totalPage() {
    // 整除情况
    assert_eq!(page_totalPage(9, 3), 3);
    assert_eq!(page_totalPage(10, 5), 2);

    // 向上取整情况
    assert_eq!(page_totalPage(10, 3), 4);
    assert_eq!(page_totalPage(20, 3), 7);

    // 边界情况
    assert_eq!(page_totalPage(0, 10), 0);
    assert_eq!(page_totalPage(1, 10), 1);
}

#[test]
pub fn test_page_rainbow() {
    // 测试总页数大于显示数量，当前页在左侧
    let result = page_rainbow(1, 10, 5);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    // 测试总页数大于显示数量，当前页在中间
    let result = page_rainbow(5, 10, 5);
    assert_eq!(result, vec![3, 4, 5, 6, 7]);

    // 测试总页数大于显示数量，当前页在右侧
    let result = page_rainbow(10, 10, 5);
    assert_eq!(result, vec![6, 7, 8, 9, 10]);

    // 测试总页数小于显示数量
    let result = page_rainbow(2, 5, 10);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    // 测试总页数等于显示数量
    let result = page_rainbow(5, 5, 5);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    // 测试偶数显示数量
    let result = page_rainbow(5, 20, 6);
    assert_eq!(result, vec![3, 4, 5, 6, 7, 8]);
}
