// #[derive(Debug, PartialEq, PartialOrd)]
// struct Amount(f64);

#[derive(Debug)]
struct Balance {
    pub amount: f64 // Amount
}

impl Balance {
    pub fn new(amount: f64) -> Balance {
        Balance {
            amount: amount
        }
    }
}

impl PartialEq for Balance {
    fn eq(&self, other: &Balance) -> bool {
        self.amount == other.amount
    }
}

#[derive(Debug)]
struct Account {
    pub no: String,
    pub name: String,
    pub balance: Balance
}

impl Account {
    pub fn new(no: String, name: String, balance: Balance) -> Account {
        Account {
            no: no,
            name: name,
            balance: balance
        }
    }

    pub fn credit(&mut self, amount: f64) {
        self.balance = Balance::new(self.balance.amount + amount);
    }

    pub fn debit(&mut self, amount: f64) {
        if self.balance.amount < amount {
            panic!("Insufficient balance in account");
        }
        self.balance = Balance::new(self.balance.amount - amount);
    }
}

fn main() {
    let mut account = Account::new("a1".to_string(), "John".to_string(), Balance::new(0.0));
    assert!(account.balance == Balance { amount: 0.0 });
    println!("Account: {:?}", account);

    account.credit(100.0);
    assert!(account.balance == Balance { amount: 100.0 });
    println!("Credit: {:?}", 100.0);

    account.debit(20.0);
    assert!(account.balance == Balance { amount: 80.0 });
    println!("Debit: {:?}", 20.0);

    println!("Balance: {:?}", account.balance);
}
