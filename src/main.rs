use crate::bankaccount::BankAccount;

mod bankaccount;

fn main() {
    println!("Hello, world!");
    let mut account1 = BankAccount::new(500.0);
    account1.deposit(1000.0);
    account1.withdraw(2000.0);
    println!("{}",account1.balance());
}
