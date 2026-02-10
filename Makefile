.PHONY: test test-core test-all clean help

# 运行 huturs-core 的所有测试（包括 test 目录下的集成测试）
test:
	@echo "Running huturs-core tests..."
	@cd huturs-core && cargo test

# 运行 huturs-core 的库测试
test-core:
	@echo "Running huturs-core library tests..."
	@cargo test -p huturs-core --lib

# 运行所有测试（包括整个 workspace）
test-all:
	@echo "Running all tests in workspace..."
	@cargo test

# 清理构建文件
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# 显示帮助信息
help:
	@echo "Available targets:"
	@echo "  test      - Run huturs-core tests (including integration tests)"
	@echo "  test-core - Run huturs-core library tests only"
	@echo "  test-all  - Run all tests in the workspace"
	@echo "  clean     - Clean build artifacts"
	@echo "  help      - Show this help message"