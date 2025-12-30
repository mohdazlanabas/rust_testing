/// A simple bank account with basic operations
#[derive(Debug, Clone)]
pub struct BankAccount {
    pub account_number: String,
    pub holder_name: String,
    balance: f64,
}

impl BankAccount {
    /// Creates a new bank account with initial balance
    pub fn new(account_number: String, holder_name: String, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: initial_balance,
        }
    }

    /// Deposits money into the account
    pub fn deposit(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(self.balance)
    }

    /// Withdraws money from the account
    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    /// Returns the current balance
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    /// Transfers money to another account
    pub fn transfer(&mut self, to_account: &mut BankAccount, amount: f64) -> Result<(), String> {
        self.withdraw(amount)?;
        match to_account.deposit(amount) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Rollback if deposit fails
                self.deposit(amount).ok();
                Err(e)
            }
        }
    }
}

// ============================================================================
// UNIT TESTS - Test individual functions in isolation
// ============================================================================
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        assert_eq!(account.account_number, "ACC001");
        assert_eq!(account.holder_name, "Roger");
        assert_eq!(account.get_balance(), 1000.0);
    }

    #[test]
    fn test_deposit_positive_amount() {
        let mut account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        let result = account.deposit(500.0);
        assert!(result.is_ok());
        assert_eq!(account.get_balance(), 1500.0);
    }

    #[test]
    fn test_deposit_negative_amount() {
        let mut account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        let result = account.deposit(-100.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Deposit amount must be positive");
        assert_eq!(account.get_balance(), 1000.0); // Balance unchanged
    }

    #[test]
    fn test_withdraw_sufficient_funds() {
        let mut account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        let result = account.withdraw(300.0);
        assert!(result.is_ok());
        assert_eq!(account.get_balance(), 700.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let mut account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        let result = account.withdraw(1500.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
        assert_eq!(account.get_balance(), 1000.0); // Balance unchanged
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BankAccount::new(
            "ACC001".to_string(),
            "Roger".to_string(),
            1000.0,
        );
        let result = account.withdraw(-100.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Withdrawal amount must be positive");
    }
}
