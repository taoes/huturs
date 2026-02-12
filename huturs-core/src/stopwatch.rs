//! 秒表工具类模块
//! 提供精确的时间测量功能，用于测量代码执行时间或操作耗时
//!
//! # 示例
//!
//! ```
//! use huturs_core::stopwatch::StopWatch;
//!
//! // 创建并启动秒表
//! let mut stopwatch = StopWatch::start_new();
//!
//! // 执行一些操作
//! std::thread::sleep(std::time::Duration::from_millis(100));
//!
//! // 停止秒表
//! stopwatch.stop();
//!
//! // 获取耗时
//! println!("耗时: {:?}", stopwatch.elapsed());
//! ```

use std::time::{Duration, Instant};

/// 秒表结构体，用于测量时间间隔
///
/// 提供精确到纳秒级别的时间测量功能，支持暂停、继续和重置操作
///
/// # 示例
///
/// ```
/// use huturs_core::stopwatch::StopWatch;
///
/// // 创建未启动的秒表
/// let mut sw = StopWatch::new();
///
/// // 启动秒表
/// sw.start();
///
/// // 停止秒表
/// sw.stop();
///
/// // 获取耗时
/// let elapsed = sw.elapsed();
/// ```
pub struct StopWatch {
    /// 开始时间点
    start_time: Option<Instant>,
    /// 累计耗时
    elapsed: Duration,
    /// 是否正在运行
    is_running: bool,
}

impl StopWatch {
    /// 创建一个新的未启动的秒表
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let sw = StopWatch::new();
    /// assert!(!sw.is_running());
    /// ```
    pub fn new() -> Self {
        StopWatch {
            start_time: None,
            elapsed: Duration::ZERO,
            is_running: false,
        }
    }

    /// 创建一个新的秒表并立即启动
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let sw = StopWatch::start_new();
    /// assert!(sw.is_running());
    /// ```
    pub fn start_new() -> Self {
        let mut sw = StopWatch::new();
        sw.start();
        sw
    }

    /// 启动秒表
    ///
    /// 如果秒表已经在运行，此方法不会产生任何效果
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::new();
    /// sw.start();
    /// assert!(sw.is_running());
    /// ```
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }

    /// 停止秒表
    ///
    /// 如果秒表未运行，此方法不会产生任何效果
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// sw.stop();
    /// assert!(!sw.is_running());
    /// ```
    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            if self.is_running {
                self.elapsed += start.elapsed();
                self.is_running = false;
            }
        }
    }

    /// 重置秒表
    ///
    /// 将秒表恢复到初始状态，清除所有计时数据
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// std::thread::sleep(std::time::Duration::from_millis(10));
    /// sw.reset();
    /// assert_eq!(sw.elapsed(), std::time::Duration::ZERO);
    /// ```
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::ZERO;
        self.is_running = false;
    }

    /// 获取累计耗时
    ///
    /// 如果秒表正在运行，返回从开始到当前的耗时加上之前的累计耗时
    /// 如果秒表已停止，返回累计的总耗时
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// std::thread::sleep(std::time::Duration::from_millis(10));
    /// let elapsed = sw.elapsed();
    /// assert!(elapsed >= std::time::Duration::from_millis(10));
    /// ```
    pub fn elapsed(&self) -> Duration {
        if self.is_running {
            if let Some(start) = self.start_time {
                return self.elapsed + start.elapsed();
            }
        }
        self.elapsed
    }

    /// 检查秒表是否正在运行
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::new();
    /// assert!(!sw.is_running());
    /// sw.start();
    /// assert!(sw.is_running());
    /// sw.stop();
    /// assert!(!sw.is_running());
    /// ```
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// 获取以毫秒为单位的耗时
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// std::thread::sleep(std::time::Duration::from_millis(100));
    /// let millis = sw.elapsed_millis();
    /// assert!(millis >= 100);
    /// ```
    pub fn elapsed_millis(&self) -> u128 {
        self.elapsed().as_millis()
    }

    /// 获取以微秒为单位的耗时
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// std::thread::sleep(std::time::Duration::from_micros(100));
    /// let micros = sw.elapsed_micros();
    /// assert!(micros >= 100);
    /// ```
    pub fn elapsed_micros(&self) -> u128 {
        self.elapsed().as_micros()
    }

    /// 获取以纳秒为单位的耗时
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let mut sw = StopWatch::start_new();
    /// thread::sleep(Duration::from_millis(1));
    /// let nanos = sw.elapsed_nanos();
    /// assert!(nanos > 0);
    /// ```
    pub fn elapsed_nanos(&self) -> u128 {
        self.elapsed().as_nanos()
    }

    /// 获取以秒为单位的耗时（浮点数）
    ///
    /// # 示例
    ///
    /// ```
    /// use huturs_core::stopwatch::StopWatch;
    ///
    /// let mut sw = StopWatch::start_new();
    /// std::thread::sleep(std::time::Duration::from_millis(100));
    /// let secs = sw.elapsed_secs_f64();
    /// assert!(secs >= 0.1);
    /// ```
    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for StopWatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StopWatch")
            .field("elapsed", &self.elapsed())
            .field("is_running", &self.is_running)
            .finish()
    }
}

impl std::fmt::Display for StopWatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.elapsed();
        write!(
            f,
            "{}.{:03}s",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        )
    }
}