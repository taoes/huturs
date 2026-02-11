//! 数学工具类模块
//! 提供数学计算相关的工具函数

/// 计算两个数的和
///
/// # 参数
/// * `a` - 第一个加数
/// * `b` - 第二个加数
///
/// # 返回值
/// 返回两个数的和
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::add(2, 3), 5);
/// assert_eq!(math::add(1.5, 2.5), 4.0);
/// ```
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/// 计算两个数的差
///
/// # 参数
/// * `a` - 被减数
/// * `b` - 减数
///
/// # 返回值
/// 返回第一个数减去第二个数的结果
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::subtract(5, 3), 2);
/// assert_eq!(math::subtract(3, 5), -2);
/// ```
pub fn subtract<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}

/// 计算两个数的积
///
/// # 参数
/// * `a` - 第一个乘数
/// * `b` - 第二个乘数
///
/// # 返回值
/// 返回两个数的乘积
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::multiply(3, 4), 12);
/// assert_eq!(math::multiply(2.5, 4.0), 10.0);
/// ```
pub fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

/// 计算两个数的商
///
/// # 参数
/// * `a` - 被除数
/// * `b` - 除数
///
/// # 返回值
/// 返回第一个数除以第二个数的结果
///
/// # 注意
/// 此函数不处理除数为零的情况，调用者需要确保除数不为零
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::divide(10, 2), 5);
/// assert_eq!(math::divide(7, 2), 3);
/// ```
pub fn divide<T: std::ops::Div<Output = T>>(a: T, b: T) -> T {
    a / b
}

/// 计算整数的绝对值
///
/// # 参数
/// * `x` - 输入的整数
///
/// # 返回值
/// 返回该整数的绝对值
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::abs(5), 5);
/// assert_eq!(math::abs(-5), 5);
/// assert_eq!(math::abs(0), 0);
/// ```
pub fn abs(x: i64) -> i64 {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// 计算浮点数的绝对值
///
/// # 参数
/// * `x` - 输入的浮点数
///
/// # 返回值
/// 返回该浮点数的绝对值
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::abs_f64(5.0), 5.0);
/// assert_eq!(math::abs_f64(-5.5), 5.5);
/// ```
pub fn abs_f64(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

/// 计算两个数中的最大值
///
/// # 参数
/// * `a` - 第一个数
/// * `b` - 第二个数
///
/// # 返回值
/// 返回两个数中较大的那个
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::max(5, 3), 5);
/// assert_eq!(math::max(3, 5), 5);
/// ```
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// 计算两个数中的最小值
///
/// # 参数
/// * `a` - 第一个数
/// * `b` - 第二个数
///
/// # 返回值
/// 返回两个数中较小的那个
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::min(5, 3), 3);
/// assert_eq!(math::min(3, 5), 3);
/// ```
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

/// 计算数的平方
///
/// # 参数
/// * `x` - 输入的数
///
/// # 返回值
/// 返回该数的平方值
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::square(5), 25);
/// assert_eq!(math::square(3), 9);
/// ```
pub fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

/// 计算数的立方
///
/// # 参数
/// * `x` - 输入的数
///
/// # 返回值
/// 返回该数的立方值
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::cube(3), 27);
/// assert_eq!(math::cube(2), 8);
/// ```
pub fn cube<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x * x
}

/// 计算数的幂运算
///
/// # 参数
/// * `base` - 底数
/// * `exponent` - 指数
///
/// # 返回值
/// 返回 base 的 exponent 次方
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert_eq!(math::power(2, 3), 8);
/// assert_eq!(math::power(3, 2), 9);
/// ```
pub fn power<T: std::ops::Mul<Output = T> + Copy>(base: T, exponent: u32) -> T {
    let mut result = base;
    for _ in 1..exponent {
        result = result * base;
    }
    result
}

/// 检查整数是否为偶数
///
/// # 参数
/// * `n` - 要检查的整数
///
/// # 返回值
/// 如果该数是偶数，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert!(math::is_even(4));
/// assert!(math::is_even(0));
/// assert!(!math::is_even(3));
/// ```
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// 检查整数是否为奇数
///
/// # 参数
/// * `n` - 要检查的整数
///
/// # 返回值
/// 如果该数是奇数，返回 `true`；否则返回 `false`
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// assert!(math::is_odd(3));
/// assert!(math::is_odd(1));
/// assert!(!math::is_odd(4));
/// ```
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// 计算浮点数数组的平均值
///
/// # 参数
/// * `numbers` - 浮点数数组
///
/// # 返回值
/// 返回数组的平均值，如果数组为空则返回 0.0
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// assert_eq!(math::average(&nums), 3.0);
/// ```
pub fn average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

/// 计算数组的总和
///
/// # 参数
/// * `numbers` - 数值数组
///
/// # 返回值
/// 返回数组中所有元素的总和
///
/// # 注意
/// 如果数组为空，此函数会 panic
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// let nums = vec![1, 2, 3, 4, 5];
/// assert_eq!(math::sum(&nums), 15);
/// ```
pub fn sum<T: std::ops::Add<Output = T> + Copy>(numbers: &[T]) -> T {
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = result + num;
    }
    result
}

/// 计算数组中的最大值
///
/// # 参数
/// * `numbers` - 数值数组
///
/// # 返回值
/// 返回数组中的最大值，如果数组为空则返回 `None`
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// let nums = vec![1, 5, 3, 9, 2];
/// assert_eq!(math::max_in_array(&nums), Some(9));
/// ```
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

/// 计算数组中的最小值
///
/// # 参数
/// * `numbers` - 数值数组
///
/// # 返回值
/// 返回数组中的最小值，如果数组为空则返回 `None`
///
/// # 示例
///
/// ```
/// use huturs_core::math;
///
/// let nums = vec![1, 5, 3, 9, 2];
/// assert_eq!(math::min_in_array(&nums), Some(1));
/// ```
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