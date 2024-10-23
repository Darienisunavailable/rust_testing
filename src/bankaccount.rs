#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Create new account, make it a BankAccount
        // Set balance
        let account = BankAccount {
            balance: initial_balance,
        };

        // Return the account struct
        account
    }

    pub fn deposit(&mut self, amount: f64) {
        // Ignore operation if amount is negative
        if amount < 0f64 {
            return;
        }

        // Should increase balance
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        // If amount is greater than balance or negative
        // Balance should be unchanged
        if amount > self.balance || amount < 0f64 {
            return;
        }
        // Should decrease balance
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account1 = BankAccount::new(500.0);
        assert_eq!(account1.balance(), 500.0);

    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        // Tests normal case
        // Tests if negative
        let mut account1 = BankAccount::new(500.0);
        account1.deposit(1000.0);
        assert_eq!(account1.balance(), 1500.0);
        account1.deposit(-500.0);
        assert_ne!(account1.balance(), 1000.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account1 = BankAccount::new(500.0);
        account1.withdraw(250.0);
        assert_eq!(account1.balance(), 250.0);
        account1.withdraw(300.0);
        assert_eq!(account1.balance(), 250.0);
        account1.withdraw(-500.0);
        assert_eq!(account1.balance(), 250.0);
    }

    // Add more tests here
}