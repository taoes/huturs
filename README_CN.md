# Huturs

一个全面的 Rust 工具库，提供字符串处理、日期时间操作和数学计算的常用工具。

## 概述

Huturs 是一组可重用的工具函数集合，旨在简化 Rust 中的常见编程任务。它为日常操作提供了清晰、高效的 API。

## 特性

- **字符串工具**: 常见字符串操作，包括修剪、大小写转换、搜索等
- **日期时间工具**: 时间戳处理、日期格式化和时间计算
- **数学工具**: 基础数学运算、数组计算和数值工具

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
huturs-core = "0.1.0"
```

## 使用方法

### 字符串操作

```rust
use huturs_core::string;

fn main() {
    // 检查字符串是否为空
    let empty = string::is_empty("");
    
    // 转换为大写
    let upper = string::to_uppercase("hello");
    
    // 反转字符串
    let reversed = string::reverse("rust");
    
    // 分割字符串
    let parts = string::split("a,b,c", ",");
}
```

### 日期时间操作

```rust
use huturs_core::date;

fn main() {
    // 获取当前时间戳（秒）
    let timestamp = date::current_timestamp();
    
    // 获取当前时间戳（毫秒）
    let timestamp_ms = date::current_timestamp_millis();
    
    // 检查时间戳是否在未来
    let is_future = date::is_future(timestamp + 1000);
    
    // 计算两个时间戳之间的差值
    let timestamp1 = date::current_timestamp();
    let timestamp2 = timestamp1 - 3600; // 1小时前
    let diff = date::diff_seconds(timestamp1, timestamp2);
    
    // 添加秒数到时间戳
    let future_time = date::add_seconds(timestamp, 3600);
    
    // 获取时间戳对应的天数
    let days = date::get_days(timestamp);
}
```

### 数学运算

```rust
use huturs_core::math;

fn main() {
    // 基础运算
    let sum = math::add(5, 3);
    let product = math::multiply(4, 6);
    
    // 数组运算
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avg = math::average(&numbers);
    let max_val = math::max_in_array(&numbers);
    
    // 检查奇偶性
    let is_even = math::is_even(4);
}
```

## 模块

### `string`
- 空值和空白检查
- 大小写转换（大写/小写）
- 修剪操作
- 字符串反转
- 模式匹配（包含、开头、结尾）
- 字符串替换和分割
- 字符串连接和重复

### `date`
- 当前时间戳获取（秒/毫秒）
- 时间戳格式化
- 时间差计算
- 未来/过去时间戳检查
- 时间算术（加/减秒）
- 时间单位转换（分钟、小时、天）

### `math`
- 基础算术运算（加、减、乘、除）
- 绝对值计算
- 最大值/最小值
- 幂运算（平方、立方、幂）
- 奇偶性检查（偶数/奇数）
- 数组运算（求和、平均值、最大值、最小值）

## 项目结构

```
huturs/
├── huturs-core/       # 核心库
│   ├── src/
│   │   ├── string/    # 字符串工具
│   │   ├── date/      # 日期时间工具
│   │   └── math/      # 数学工具
│   └── tests/         # 集成测试
├── example/           # 示例用法
└── Cargo.toml         # 工作区配置
```

## 开发

### 构建

```bash
cargo build
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 或使用 makefile
make test
```

### 清理

```bash
cargo clean
```

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 项目状态

本项目目前正在积极开发中。版本 0.1.0 是包含核心功能的初始版本。

## 路线图

- [ ] 添加更多字符串操作工具
- [ ] 增强日期时间格式化选项
- [ ] 添加高级数学函数
- [ ] 实现文件 I/O 工具
- [ ] 添加全面的文档和示例

## 贡献

欢迎贡献！以下是如何参与的方式：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 进行修改
4. 运行测试确保一切正常 (`cargo test`)
5. 提交更改 (`git commit -m 'Add some amazing feature'`)
6. 推送到分支 (`git push origin feature/amazing-feature`)
7. 打开 Pull Request

### 开发指南

- 编写清晰、可读的代码，并添加适当的文档
- 为新功能添加测试
- 遵循 Rust 最佳实践和约定
- 保持函数专注和简洁
