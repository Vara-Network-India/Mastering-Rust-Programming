//IMPORTANT
//Struct a data structure that alllows you to group multiple fields
//together under one name.

fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    }

    //immutable borrow to check the balance
    account.check_balance();
}

Struct BankAccount::{
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawn {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has balance of", self.owner, self.balance);
    }
} 