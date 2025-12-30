.PHONY: all build run test test-unit test-integration test-system clean help

# Default target
all: build test

# Build the project
build:
	@echo "Building project..."
	cargo build

# Run the application
run:
	@echo "Running application..."
	cargo run

# Run all tests
test:
	@echo "Running all tests..."
	cargo test

# Run only unit tests
test-unit:
	@echo "Running unit tests..."
	cargo test --lib

# Run only integration tests
test-integration:
	@echo "Running integration tests..."
	cargo test --test integration_tests

# Run only system tests
test-system:
	@echo "Running system tests..."
	cargo test --test system_tests

# Run tests with output
test-verbose:
	@echo "Running tests with output..."
	cargo test -- --nocapture --test-threads=1

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Show help
help:
	@echo "Available targets:"
	@echo "  make build              - Build the project"
	@echo "  make run                - Run the application"
	@echo "  make test               - Run all tests"
	@echo "  make test-unit          - Run unit tests only"
	@echo "  make test-integration   - Run integration tests only"
	@echo "  make test-system        - Run system tests only"
	@echo "  make test-verbose       - Run tests with output"
	@echo "  make clean              - Clean build artifacts"
	@echo "  make help               - Show this help message"
