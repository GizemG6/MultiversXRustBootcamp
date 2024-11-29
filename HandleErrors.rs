// Define the Account Trait
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account Trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be greater than zero.".to_string())
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        } else if amount > self.balance {
            Err("Insufficient funds.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 101,
        holder_name: "Alice".to_string(),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 102,
        holder_name: "Bob".to_string(),
        balance: 1000.0,
    };

    // Deposit into account1
    match account1.deposit(300.0) {
        Ok(_) => println!("Deposit successful. New balance: {:.2}", account1.balance()),
        Err(e) => println!("Failed to deposit: {}", e),
    }

    // Try to deposit a negative amount
    match account1.deposit(-50.0) {
        Ok(_) => println!("Deposit successful. New balance: {:.2}", account1.balance()),
        Err(e) => println!("Failed to deposit: {}", e),
    }

    // Withdraw from account2
    match account2.withdraw(200.0) {
        Ok(_) => println!("Withdrawal successful. New balance: {:.2}", account2.balance()),
        Err(e) => println!("Failed to withdraw: {}", e),
    }

    // Try to withdraw more than the balance
    match account2.withdraw(1500.0) {
        Ok(_) => println!("Withdrawal successful. New balance: {:.2}", account2.balance()),
        Err(e) => println!("Failed to withdraw: {}", e),
    }

    // Display final balances
    println!("Final balance for {}: {:.2}", account1.holder_name, account1.balance());
    println!("Final balance for {}: {:.2}", account2.holder_name, account2.balance());
}
