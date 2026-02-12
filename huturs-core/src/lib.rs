//! huturs-core - 核心工具库
//! 提供字符串、日期、数学等常用工具函数

// 金额处理模块
#[cfg(feature = "amount")]
#[cfg_attr(docsrs, doc(cfg(feature = "amount")))]
pub mod amount;
#[cfg(feature = "amount")]
pub use amount::*;

// 日期时间处理模块
#[cfg(feature = "datetime")]
#[cfg_attr(docsrs, doc(cfg(feature = "datetime")))]
pub mod datetime;
#[cfg(feature = "datetime")]
pub use datetime::*;

// 文件操作模块
#[cfg(feature = "file")]
#[cfg_attr(docsrs, doc(cfg(feature = "file")))]
pub mod file;
#[cfg(feature = "file")]
pub use file::*;

// 数学计算模块
#[cfg(feature = "math")]
#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
pub mod math;
#[cfg(feature = "math")]
pub use math::*;

// 字符串处理模块
#[cfg(feature = "str")]
#[cfg_attr(docsrs, doc(cfg(feature = "str")))]
pub mod str;
#[cfg(feature = "str")]
pub use str::*;

// 时间戳处理模块
#[cfg(feature = "timestamp")]
#[cfg_attr(docsrs, doc(cfg(feature = "timestamp")))]
pub mod timestamp;
#[cfg(feature = "timestamp")]
pub use timestamp::*;

// 计时器模块
#[cfg(feature = "stopwatch")]
#[cfg_attr(docsrs, doc(cfg(feature = "stopwatch")))]
pub mod stopwatch;
#[cfg(feature = "stopwatch")]
pub use stopwatch::*;

// 通用工具类
#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
pub mod util;
#[cfg(feature = "util")]
pub use util::*;