// ============================================================================
// SYSTEM TESTS - Test the entire application end-to-end
// ============================================================================
use std::process::Command;

#[test]
fn test_system_runs_successfully() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .current_dir("../")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Application should run without errors");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify key elements are present in output
    assert!(stdout.contains("Bank Account System"), "Should display title");
    assert!(stdout.contains("Roger"), "Should mention Roger's account");
    assert!(stdout.contains("Alice"), "Should mention Alice's account");
    assert!(stdout.contains("Initial Balances"), "Should show initial balances");
    assert!(stdout.contains("Final Balances"), "Should show final balances");
}

#[test]
fn test_system_output_contains_operations() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .current_dir("../")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify all operations are executed
    assert!(stdout.contains("deposits"), "Should show deposit operation");
    assert!(stdout.contains("withdraws"), "Should show withdraw operation");
    assert!(stdout.contains("transfers"), "Should show transfer operation");
    assert!(stdout.contains("Error: Insufficient funds"), "Should show error handling");
}

#[test]
fn test_system_calculates_balances_correctly() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .current_dir("../")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Roger: 5000 + 1000 - 2000 = 4000
    // Alice: 3000 - 500 + 2000 = 4500
    assert!(stdout.contains("4000.00"), "Roger's final balance should be RM 4000.00");
    assert!(stdout.contains("4500.00"), "Alice's final balance should be RM 4500.00");
}
