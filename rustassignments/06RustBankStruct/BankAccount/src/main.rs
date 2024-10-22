#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{balance: initial_balance}
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        println!("Deposited ${:?}", amount);
        return self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        // if account balance is greater or equal to withdrawl amount
        if self.balance >= amount{
            return self.balance -= amount;
        }
        
        // else, not enough money: do nothing
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        return self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Test for creating a new account
        let account = BankAccount::new(100.00);
        // check for account balance, if exists then there is an account
        assert_eq!(account.balance(), 100.00)
    }

    #[test]
    fn test_deposit() {
        // Test for depositing money
        let mut account = BankAccount::new(50.00);
        account.deposit(50.00);
        // check account balance to verify
        assert_eq!(account.balance(), 100.00);
    }

    
    mod test_withdraw {
        use super::*;

        #[test]
        fn good_withdraw() {
            // Test for withdrawing money
            let mut account = BankAccount::new(100.00);
            account.withdraw(50.00);
            // check account balance to verify
            assert_eq!(account.balance(), 50.00);
        }

        #[test]
        fn bad_withdraw() {
            // Test for no withdrawing money
            let mut account = BankAccount::new(10.00);
            account.withdraw(50.00);
            // check account balance to verify
            assert_eq!(account.balance(), 10.00);
        }
    }
    
    #[test]
    fn test_balance(){
        let account = BankAccount::new(1000.00);
        // check current account balance
        assert_eq!(account.balance(), 1000.00);
    }
}