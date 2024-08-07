//IMPORTANT
//Struct a data structure that alllows you to group multiple fields
//together under one name.

// Define the structure BankAccount with fields owner and balance
struct BankAccount {
    owner: String,
    balance: f64,
}

// Implement methods for the BankAccount structure
impl BankAccount {
    // Define the withdraw method that takes a mutable reference to self
    // This method allows modification of the account's balance
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawn {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // Define the check_bal method that takes an immutable reference to self
    // This method allows reading the account's balance without modifying it
    fn check_bal(&mut self) {
        println!("Account owned by {} has balance of {}", self.owner, self.balance);
    }
}

fn main() {
    // Create a new BankAccount instance with owner Alice and balance 150.55
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow to check the balance
    // This calls the check_bal method, which borrows account immutably
    // Multiple immutable borrows are allowed, and no modifications can be made
    account.check_bal();

    // Mutable borrow to withdraw money
    // This calls the withdraw method, which borrows account mutably
    // Only one mutable borrow is allowed at a time to ensure exclusive write access
    account.withdraw(50.0);

    // Check the balance again to see the updated balance after withdrawal
    // This is another immutable borrow
    account.check_bal();
}
