[🇨🇳 中文](README_CN.md)
[🇬🇧 English](README.md)
---

# HutuRs

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

## 文档
详细的 API 文档和使用示例，请访问：
📚 [https://taoes.github.io/huturs/huturs_core/index.html](https://taoes.github.io/huturs/huturs_core/index.html)


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