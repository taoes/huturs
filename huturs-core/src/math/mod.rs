//! 数学工具类模块
//! 提供数学计算相关的工具函数

/// 计算两个数的和
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/// 计算两个数的差
pub fn subtract<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

/// 计算两个数的积
pub fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

/// 计算两个数的商
pub fn divide<T: std::ops::Div<Output = T>>(a: T, b: T) -> T {
    a / b
}

/// 计算绝对值
pub fn abs(x: i64) -> i64 {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// 计算绝对值（浮点数）
pub fn abs_f64(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// 计算最大值
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// 计算最小值
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

/// 计算平方
pub fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

/// 计算立方
pub fn cube<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x * x
}

/// 计算幂运算
pub fn power<T: std::ops::Mul<Output = T> + Copy>(base: T, exponent: u32) -> T {
    let mut result = base;
    for _ in 1..exponent {
        result = result * base;
    }
    result
}

/// 检查是否为偶数
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// 检查是否为奇数
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// 计算平均值
pub fn average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

/// 计算总和
pub fn sum<T: std::ops::Add<Output = T> + Copy>(numbers: &[T]) -> T {
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = result + num;
    }
    result
}

/// 计算最大值（数组）
pub fn max_in_array<T: PartialOrd + Copy>(numbers: &[T]) -> Option<T> {
    if numbers.is_empty() {
        return None;
    }
    let mut max_val = numbers[0];
    for &num in &numbers[1..] {
        if num > max_val {
            max_val = num;
        }
    }
    Some(max_val)
}

/// 计算最小值（数组）
pub fn min_in_array<T: PartialOrd + Copy>(numbers: &[T]) -> Option<T> {
    if numbers.is_empty() {
        return None;
    }
    let mut min_val = numbers[0];
    for &num in &numbers[1..] {
        if num < min_val {
            min_val = num;
        }
    }
    Some(min_val)
}
