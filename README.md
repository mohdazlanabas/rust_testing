# readme.md

**Creation Date:** 2025-12-30
**GitHub Remote:** https://github.com/mohdazlanabas/rust_testing
**Developed by:** net1io.com
**Copyright (C) 2020**

---

# Rust Testing Demo

## App Introduction

A barebone Rust application designed to demonstrate three essential testing types: **Unit Tests**, **Integration Tests**, and **System Tests**. The application implements a simple banking system with accounts, deposits, withdrawals, and transfers.

This project provides hands-on experience with Rust's built-in testing framework and best practices for organizing tests in a real-world application structure.

## App Language and Frameworks

**Primary Language:** Rust (1.70+)

**Frameworks & Tools:**
- Rust's built-in testing framework (`#[test]`, `#[cfg(test)]`)
- Cargo (Rust's package manager and build system)
- Standard Library only (no external dependencies)

**Testing Types:**
- Unit Tests (inline with code)
- Integration Tests (separate `tests/` directory)
- System Tests (end-to-end execution tests)

## App Architecture

### Testing Types Explained

#### 1. Unit Tests
**Location:** `src/lib.rs` (within the module they test)

**Purpose:** Test individual functions in isolation

**Characteristics:**
- Test single functions/methods
- Run fast
- No external dependencies
- Use `#[cfg(test)]` and `#[test]` attributes
- Located in the same file as the code they test

**Examples in this project:**
- `test_create_account()` - Tests account creation
- `test_deposit_positive_amount()` - Tests deposit logic
- `test_withdraw_insufficient_funds()` - Tests error handling

#### 2. Integration Tests
**Location:** `tests/integration_tests.rs` (separate `tests/` directory)

**Purpose:** Test multiple components working together

**Characteristics:**
- Test interactions between modules
- Treat the crate as an external library
- Each file in `tests/` is a separate crate
- Test realistic workflows

**Examples in this project:**
- `test_transfer_between_accounts()` - Tests transfer between two accounts
- `test_chain_transfers()` - Tests multiple transfers in sequence
- `test_multiple_operations_workflow()` - Tests deposit + withdraw combinations

#### 3. System Tests
**Location:** `tests/system_tests.rs`

**Purpose:** Test the entire application end-to-end

**Characteristics:**
- Test the complete system as a black box
- Verify actual program execution
- Check command-line output
- Validate business logic across the entire application

**Examples in this project:**
- `test_system_runs_successfully()` - Ensures app runs without crashes
- `test_system_output_contains_operations()` - Verifies all operations execute
- `test_system_calculates_balances_correctly()` - Validates final state

### Core Components

**BankAccount struct** (`src/lib.rs`)
```rust
pub struct BankAccount {
    pub account_number: String,
    pub holder_name: String,
    balance: f64,
}
```

**Methods:**
- `new()` - Create account
- `deposit()` - Add funds
- `withdraw()` - Remove funds
- `get_balance()` - Check balance
- `transfer()` - Move funds between accounts

## App File Structure

```
rust_testing/
├── .git/                       # Git repository
├── .gitignore                  # Git ignore patterns
├── Cargo.toml                  # Project manifest and dependencies
├── Makefile                    # Build automation and shortcuts
├── readme.md                   # This file
├── src/
│   ├── lib.rs                  # Library code with BankAccount struct
│   │                           # Contains UNIT TESTS
│   └── main.rs                 # Executable binary for demo
└── tests/
    ├── integration_tests.rs    # INTEGRATION TESTS
    └── system_tests.rs         # SYSTEM TESTS
```

### Key Files

**Cargo.toml**
- Package name: `rust-testing-demo`
- Version: 0.1.0
- Edition: 2021
- No external dependencies (uses Rust standard library only)

**Makefile**
- Provides convenient shortcuts for common commands
- See "Using Makefile Commands" section below

**.gitignore**
- Comprehensive ignore patterns for multiple project types
- Ignores build artifacts, dependencies, and sensitive files

## App Compiling Steps

### Prerequisites
- Rust toolchain (1.70+)
- Cargo (comes with Rust)
- Make (optional, for using Makefile shortcuts)

### Installation
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/mohdazlanabas/rust_testing.git
cd rust_testing

# Build the project
cargo build
# OR using Makefile
make build

# Build with release optimizations
cargo build --release
```

### Verify Installation
```bash
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# Verify project compiles
cargo check
```

## App Run Steps

### Running the Application
```bash
# Run the demo application
cargo run
# OR using Makefile
make run

# Run with release optimizations
cargo run --release
```

### Expected Output
```
=== Bank Account System ===

Initial Balances:
Roger (ACC001): RM 5000.00
Alice (ACC002): RM 3000.00

Operations:
1. Roger deposits RM 1000.00... ✓ New balance: RM 6000.00
2. Alice withdraws RM 500.00... ✓ New balance: RM 2500.00
3. Roger transfers RM 2000.00 to Alice... ✓ Transfer successful
4. Alice tries to transfer RM 10000.00 to Roger... ✗ Error: Insufficient funds

Final Balances:
Roger (ACC001): RM 4000.00
Alice (ACC002): RM 4500.00
```

## Testing Instructions

### Test Commands Quick Reference

#### Basic Commands
```bash
cargo test                           # Run all tests
cargo run                            # Run the application
cargo build                          # Build the project
```

#### Selective Test Execution
```bash
cargo test --lib                     # Unit tests only (in src/)
cargo test --test integration_tests  # Integration tests only
cargo test --test system_tests       # System tests only
cargo test test_name                 # Run specific test by name
```

#### Test Output Control
```bash
cargo test -- --nocapture            # Show println! output
cargo test -- --test-threads=1       # Run tests sequentially
cargo test -- --show-output          # Show output for passing tests too
```

#### Using Makefile Commands
```bash
make build               # Build the project
make run                 # Run the application
make test                # Run all tests
make test-unit           # Unit tests only
make test-integration    # Integration tests only
make test-system         # System tests only
make test-verbose        # Tests with output (nocapture + sequential)
make clean               # Clean build artifacts
make help                # Show all available targets
```

#### Debugging Failed Tests
```bash
# Run single test with output
cargo test test_name -- --nocapture

# Run tests one at a time
cargo test -- --test-threads=1

# Show backtrace on panic
RUST_BACKTRACE=1 cargo test

# Run with verbose output
cargo test -- --nocapture --test-threads=1 --show-output
```

## Key Testing Concepts

### Assertions

#### Equality Assertions
```rust
assert_eq!(actual, expected);        // Values must match
assert_ne!(actual, not_expected);    // Values must differ
```

#### Boolean Assertions
```rust
assert!(condition);                  // Must be true
assert!(!condition);                 // Must be false
```

#### Result Assertions
```rust
assert!(result.is_ok());             // Must be Ok
assert!(result.is_err());            // Must be Err
assert_eq!(result.unwrap(), value);  // Unwrap and compare
assert_eq!(result.unwrap_err(), "error message");
```

#### Option Assertions
```rust
assert!(option.is_some());           // Must contain value
assert!(option.is_none());           // Must be empty
assert_eq!(option.unwrap(), value);  // Unwrap and compare
```

### Common Test Attributes
```rust
#[test]                              // Mark as test
#[ignore]                            // Skip by default
#[should_panic]                      // Must panic to pass
#[should_panic(expected = "msg")]    // Must panic with message
#[cfg(test)]                         // Compile only for tests
```

### Test Organization

#### In-file Unit Tests
```rust
// src/lib.rs
pub fn function() { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // Arrange - Set up test data
        let mut account = BankAccount::new(...);

        // Act - Execute the function
        let result = account.deposit(100.0);

        // Assert - Verify the outcome
        assert!(result.is_ok());
        assert_eq!(account.get_balance(), 100.0);
    }
}
```

#### Separate Test Module
```rust
// src/lib.rs
pub mod my_module;

// src/my_module.rs
pub fn function() { }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_function() { }
}
```

#### External Integration Test
```rust
// tests/integration_test.rs
use crate_name::module;

#[test]
fn test_integration() { }
```

### Error Handling in Tests
```rust
// Test that a function returns an error
let result = account.withdraw(10000.0);
assert!(result.is_err());
assert_eq!(result.unwrap_err(), "Insufficient funds");
```

## Learning Path

### Beginner
1. Read the code in `src/lib.rs`
2. Run `cargo test --lib` to see unit tests
3. Modify a function and watch tests fail
4. Add a new unit test for `get_balance()`

### Intermediate
1. Study `tests/integration_tests.rs`
2. Run `cargo test --test integration_tests`
3. Add a new integration test for a 4-way transfer
4. Test edge cases (zero transfers, same account)

### Advanced
1. Examine `tests/system_tests.rs`
2. Run `cargo test --test system_tests`
3. Add test for error scenarios in system output
4. Create mock/stub for external dependencies

## Quick Modification Exercises

### Exercise 1: Add a new unit test
- Open `src/lib.rs`
- Add test for `get_balance()` in `#[cfg(test)]` section
- Run `cargo test --lib`

### Exercise 2: Add a new integration test
- Open `tests/integration_tests.rs`
- Add test for multiple deposits
- Run `cargo test --test integration_tests`

### Exercise 3: Break a test intentionally
- Change `assert_eq!(balance, 1000.0)` to wrong value
- Run `cargo test` to see failure message
- Observe the error output and fix it

## Best Practices

### Testing Best Practices Checklist

- [ ] Test names clearly describe what they test
- [ ] Each test is independent (no shared state)
- [ ] Test both success and failure paths
- [ ] Use descriptive assertion messages
- [ ] Keep unit tests fast (< 1ms)
- [ ] Test edge cases and boundary conditions
- [ ] Don't test implementation details
- [ ] Use helper functions for common setup
- [ ] Structure tests in three parts: Arrange-Act-Assert
- [ ] Use doc comments for complex tests

### Performance Guidelines

- **Unit tests:** Should be < 1ms
- **Integration tests:** Can be 10-100ms
- **System tests:** May take seconds
- Use `cargo test --release` for performance testing
- Profile with `cargo bench` for benchmarks

## Common Testing Mistakes to Avoid

❌ Testing implementation details instead of behavior  
❌ Tests that depend on execution order  
❌ Slow tests in the unit test suite  
❌ Not testing edge cases  
❌ Unclear test names  

✅ Test public API behavior  
✅ Independent, isolated tests  
✅ Fast unit tests, slower integration tests  
✅ Comprehensive edge case coverage  
✅ Clear, descriptive test names  

## Next Steps

1. Add tests for concurrent operations (if using threads)
2. Implement property-based testing with `proptest`
3. Add benchmarks with `criterion`
4. Test with mock objects using `mockall`
5. Measure code coverage with `tarpaulin`

## Troubleshooting

**Problem:** Tests fail to compile  
**Solution:** Run `cargo check` first to identify syntax errors

**Problem:** System tests fail  
**Solution:** Ensure you're in the correct directory when running tests

**Problem:** Tests hang  
**Solution:** Use `--test-threads=1` to run sequentially and identify the problematic test

## Git Repository

This project uses Git for version control.

**Repository:** https://github.com/mohdazlanabas/rust_testing

### Common Git Commands
```bash
# Clone the repository
git clone https://github.com/mohdazlanabas/rust_testing.git

# Check status
git status

# View commit history
git log --oneline

# Pull latest changes
git pull origin main
```

## Resources

- [Rust Book - Testing Chapter](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [GitHub Repository](https://github.com/mohdazlanabas/rust_testing)

---

**Developed by net1io.com**  
**Copyright (C) 2020**

*Project created for rapid Rust testing education.*
