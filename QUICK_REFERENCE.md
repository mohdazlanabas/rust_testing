# Rust Testing Quick Reference

## Test Commands Cheat Sheet

### Basic Commands
```bash
cargo test                           # Run all tests
cargo run                            # Run the application
cargo build                          # Build the project
```

### Selective Test Execution
```bash
cargo test --lib                     # Unit tests only (in src/)
cargo test --test integration_tests  # Integration tests only
cargo test --test system_tests       # System tests only
cargo test test_name                 # Run specific test by name
```

### Test Output Control
```bash
cargo test -- --nocapture            # Show println! output
cargo test -- --test-threads=1       # Run tests sequentially
cargo test -- --show-output          # Show output for passing tests too
```

### Using Makefile (shortcut)
```bash
make test                # All tests
make test-unit           # Unit tests
make test-integration    # Integration tests
make test-system         # System tests
make test-verbose        # Tests with full output
make run                 # Run application
```

## Test Assertion Patterns

### Equality Assertions
```rust
assert_eq!(actual, expected);        // Values must match
assert_ne!(actual, not_expected);    // Values must differ
```

### Boolean Assertions
```rust
assert!(condition);                  // Must be true
assert!(!condition);                 // Must be false
```

### Result Assertions
```rust
assert!(result.is_ok());             // Must be Ok
assert!(result.is_err());            // Must be Err
assert_eq!(result.unwrap(), value);  // Unwrap and compare
assert_eq!(result.unwrap_err(), "error message");
```

### Option Assertions
```rust
assert!(option.is_some());           // Must contain value
assert!(option.is_none());           // Must be empty
assert_eq!(option.unwrap(), value);  // Unwrap and compare
```

## Test Structure Template

### Unit Test
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
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

### Integration Test
```rust
// tests/integration_tests.rs
use your_crate_name::YourStruct;

#[test]
fn test_integration_scenario() {
    // Set up multiple components
    let mut component1 = Component1::new();
    let mut component2 = Component2::new();
    
    // Test their interaction
    component1.interact_with(&mut component2);
    
    // Verify results
    assert_eq!(component1.state(), expected_state);
}
```

### System Test
```rust
// tests/system_tests.rs
use std::process::Command;

#[test]
fn test_program_execution() {
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to execute");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("expected output"));
}
```

## Testing Best Practices Checklist

- [ ] Test names clearly describe what they test
- [ ] Each test is independent (no shared state)
- [ ] Test both success and failure paths
- [ ] Use descriptive assertion messages
- [ ] Keep unit tests fast (< 1ms)
- [ ] Test edge cases and boundary conditions
- [ ] Don't test implementation details
- [ ] Use helper functions for common setup

## Common Test Attributes

```rust
#[test]                              // Mark as test
#[ignore]                            // Skip by default
#[should_panic]                      // Must panic to pass
#[should_panic(expected = "msg")]    // Must panic with message
#[cfg(test)]                         // Compile only for tests
```

## Test Organization Patterns

### In-file Unit Tests
```rust
// src/lib.rs
pub fn function() { }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_function() { }
}
```

### Separate Test Module
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

### External Integration Test
```rust
// tests/integration_test.rs
use crate_name::module;

#[test]
fn test_integration() { }
```

## Debugging Failed Tests

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

## Quick Modification Exercise

1. **Add a new unit test:**
   - Open `src/lib.rs`
   - Add test for `get_balance()` in `#[cfg(test)]` section
   - Run `cargo test --lib`

2. **Add a new integration test:**
   - Open `tests/integration_tests.rs`
   - Add test for multiple deposits
   - Run `cargo test --test integration_tests`

3. **Break a test intentionally:**
   - Change `assert_eq!(balance, 1000.0)` to wrong value
   - Run `cargo test` to see failure message

## Performance Tips

- Unit tests should be < 1ms
- Integration tests can be 10-100ms
- System tests may take seconds
- Use `cargo test --release` for performance testing
- Profile with `cargo bench` for benchmarks

---

*Keep this file open while learning testing in Rust*
