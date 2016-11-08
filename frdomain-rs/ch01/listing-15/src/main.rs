#[derive(Debug, Clone)]
struct Balance {
    pub amount: f64
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

#[derive(Debug, Clone)]
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

    pub fn credit(self, amount: f64) -> Account {
        Account {
            no: self.no.clone(),
            name: self.name.clone(),
            balance: Balance::new(self.balance.amount + amount)
        }
    }

    pub fn debit(self, amount: f64) -> Account {
        if self.balance.amount < amount {
            panic!("Insufficient balance in account");
        }
        Account {
            no: self.no.clone(),
            name: self.name.clone(),
            balance: Balance::new(self.balance.amount - amount)
        }
    }
}

fn main() {
    let a = Account::new("a1".to_string(), "John".to_string(), Balance::new(0.0));
    assert!(a.balance == Balance { amount: 0.0 });

    let b = a.credit(100.0);
    // assert!(a.balance == Balance { amount: 0.0 });
    assert!(b.balance == Balance { amount: 100.0 });

    let c = b.debit(20.0);
    // assert!(a.balance == Balance { amount: 0.0 });
    // assert!(b.balance == Balance { amount: 100.0 });
    assert!(c.balance == Balance { amount: 80.0 });

    // println!("Account (a) Balance: {:?}", a.balance);
    // println!("Account (b) Balance: {:?}", b.balance);
    println!("Account (c) Balance: {:?}", c.balance);
}
