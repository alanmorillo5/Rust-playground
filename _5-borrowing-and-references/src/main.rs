fn main() {
    // REFERENCES AND BORROWING
    
    // References borrow values without taking ownership using &. Can be immutable or mutable.
    let mut x = 5;
    let r = &mut x;

    *r += 1;
    *r -= 3;

    println!("Values of x: {}", x);

    let mut account = BankAccount {
        owner: "Alan".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();
    // Mutable borrow to withdraw money
    account.withdraw(45.5);
    account.check_balance();
}

// struct - a data structure that allows you to group multiple fields together under one name.
struct BankAccount {
    owner: String,
    balance: f64,
}

// impl - attaches methods and functions for a struct or enum.
impl BankAccount {
    
    fn withdraw(&mut self, amt: f64) {
        println!("Withdrawing {} from account owned by {}.", amt, self.owner);
        self.balance -= amt;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {:.2}.", self.owner, self.balance);
    }
}
