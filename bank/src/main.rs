#[derive(Debug)]
struct Account {
    id: u32,
    balanse: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balanse: 0,
            holder,
        }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: Vec::new(),
        }
    }
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, "John Doe".to_string());
    // println!("{:#?}", bank);
    print_account(account);
}
