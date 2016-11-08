#[derive(Debug)]
struct Balance { amount: f64 }

impl PartialEq for Balance {
    fn eq(&self, other: &Balance) -> bool {
        self.amount == other.amount
    }
}

#[derive(Debug)]
struct Account { no: &'static str, name: &'static str, balance: Balance }

trait AccountService {
    fn new(no: &'static str, name: &'static str, amount: f64) -> Self;

    fn balance(&self) -> &Balance;

    fn credit(&self, amount: f64) -> Balance {
        Balance { amount: self.balance().amount + amount }
    }

    fn debit(&self, amount: f64) -> Balance {
        if self.balance().amount < amount {
            panic!("Insufficient balance in account");
        }
        Balance { amount: self.balance().amount - amount }
    }
}

impl AccountService for Account {
    fn new(no: &'static str, name: &'static str, amount: f64) -> Account {
        Account {
            no: no,
            name: name,
            balance: Balance { amount: amount }
        }
    }

    fn balance(&self) -> &Balance {
        &self.balance
    }
}

fn main() {
    let mut account : Account = AccountService::new("a1", "John", 0.0);
    println!("Account: {:?}", account);

    account.balance = account.credit(100.0);
    assert!(account.balance == Balance { amount: 100.0 });

    account.balance = account.debit(20.0);
    assert!(account.balance == Balance { amount: 80.0 });

    println!("Balance: {:?}", account.balance);
}
