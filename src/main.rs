use rust_testing_demo::BankAccount;
use std::io::{self, Write};

fn main() {
    println!("=== Bank Account System ===\n");

    // Create test accounts
    let mut roger_account = BankAccount::new(
        "ACC001".to_string(),
        "Roger".to_string(),
        5000.0,
    );
    
    let mut alice_account = BankAccount::new(
        "ACC002".to_string(),
        "Alice".to_string(),
        3000.0,
    );

    println!("Initial Balances:");
    println!("Roger ({}): RM {:.2}", roger_account.account_number, roger_account.get_balance());
    println!("Alice ({}): RM {:.2}\n", alice_account.account_number, alice_account.get_balance());

    // Demo operations
    println!("Operations:");
    
    // Deposit
    print!("1. Roger deposits RM 1000.00... ");
    io::stdout().flush().unwrap();
    match roger_account.deposit(1000.0) {
        Ok(balance) => println!("✓ New balance: RM {:.2}", balance),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Withdraw
    print!("2. Alice withdraws RM 500.00... ");
    io::stdout().flush().unwrap();
    match alice_account.withdraw(500.0) {
        Ok(balance) => println!("✓ New balance: RM {:.2}", balance),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Transfer
    print!("3. Roger transfers RM 2000.00 to Alice... ");
    io::stdout().flush().unwrap();
    match roger_account.transfer(&mut alice_account, 2000.0) {
        Ok(_) => println!("✓ Transfer successful"),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Failed transfer (insufficient funds)
    print!("4. Alice tries to transfer RM 10000.00 to Roger... ");
    io::stdout().flush().unwrap();
    match alice_account.transfer(&mut roger_account, 10000.0) {
        Ok(_) => println!("✓ Transfer successful"),
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\nFinal Balances:");
    println!("Roger ({}): RM {:.2}", roger_account.account_number, roger_account.get_balance());
    println!("Alice ({}): RM {:.2}", alice_account.account_number, alice_account.get_balance());
}
