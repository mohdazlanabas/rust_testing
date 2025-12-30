// ============================================================================
// INTEGRATION TESTS - Test multiple components working together
// ============================================================================
use rust_testing_demo::BankAccount;

#[test]
fn test_transfer_between_accounts() {
    let mut account1 = BankAccount::new(
        "ACC001".to_string(),
        "Roger".to_string(),
        1000.0,
    );
    let mut account2 = BankAccount::new(
        "ACC002".to_string(),
        "Alice".to_string(),
        500.0,
    );

    let result = account1.transfer(&mut account2, 300.0);
    
    assert!(result.is_ok());
    assert_eq!(account1.get_balance(), 700.0);
    assert_eq!(account2.get_balance(), 800.0);
}

#[test]
fn test_transfer_insufficient_funds() {
    let mut account1 = BankAccount::new(
        "ACC001".to_string(),
        "Roger".to_string(),
        1000.0,
    );
    let mut account2 = BankAccount::new(
        "ACC002".to_string(),
        "Alice".to_string(),
        500.0,
    );

    let result = account1.transfer(&mut account2, 1500.0);
    
    assert!(result.is_err());
    assert_eq!(account1.get_balance(), 1000.0); // Unchanged
    assert_eq!(account2.get_balance(), 500.0);  // Unchanged
}

#[test]
fn test_multiple_operations_workflow() {
    let mut account = BankAccount::new(
        "ACC001".to_string(),
        "Roger".to_string(),
        1000.0,
    );

    // Sequence of operations
    account.deposit(500.0).unwrap();
    assert_eq!(account.get_balance(), 1500.0);
    
    account.withdraw(200.0).unwrap();
    assert_eq!(account.get_balance(), 1300.0);
    
    account.deposit(300.0).unwrap();
    assert_eq!(account.get_balance(), 1600.0);
    
    account.withdraw(100.0).unwrap();
    assert_eq!(account.get_balance(), 1500.0);
}

#[test]
fn test_chain_transfers() {
    let mut account1 = BankAccount::new("ACC001".to_string(), "Roger".to_string(), 1000.0);
    let mut account2 = BankAccount::new("ACC002".to_string(), "Alice".to_string(), 500.0);
    let mut account3 = BankAccount::new("ACC003".to_string(), "Bob".to_string(), 200.0);

    // Roger transfers to Alice
    account1.transfer(&mut account2, 300.0).unwrap();
    assert_eq!(account1.get_balance(), 700.0);
    assert_eq!(account2.get_balance(), 800.0);

    // Alice transfers to Bob
    account2.transfer(&mut account3, 200.0).unwrap();
    assert_eq!(account2.get_balance(), 600.0);
    assert_eq!(account3.get_balance(), 400.0);

    // Bob transfers to Roger
    account3.transfer(&mut account1, 100.0).unwrap();
    assert_eq!(account3.get_balance(), 300.0);
    assert_eq!(account1.get_balance(), 800.0);
}
