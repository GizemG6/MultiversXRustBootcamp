// Define the Account trait
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Define the BankAccount struct
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Implement the Account trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else if amount > self.balance {
            println!(
                "Insufficient balance in account {}. Available balance: ${:.2}",
                self.account_number, self.balance
            );
        } else {
            println!("Withdraw amount must be positive.");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Alice"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Bob"),
        balance: 300.0,
    };

    // Perform operations on the accounts
    account1.deposit(200.0); // Deposit into account1
    account2.withdraw(100.0); // Withdraw from account2

    // Check and print balances
    println!(
        "Account {} balance: ${:.2}",
        account1.account_number,
        account1.balance()
    );
    println!(
        "Account {} balance: ${:.2}",
        account2.account_number,
        account2.balance()
    );
}
