#[derive(Debug)]
struct Account {
    id: u32,
    balance: u32,
}

fn transfer_funds(accounts: &mut Vec<Account>, from_id: u32, to_id: u32, amount: u32) -> Result<(), String> {
    if from_id == to_id {
        return Err("Cannot transfer to the same account".to_string())
    }

    let from_idx = accounts.iter().position(|acc| acc.id == from_id)
        .ok_or_else(|| String::from("Sender account not found"))?;

    let to_idx = accounts.iter().position(|acc| acc.id == to_id)
        .ok_or_else(|| String::from("Receiver account not found"))?;

    if accounts[from_idx].balance < amount {
        return Err("Insufficient funds".to_string())
    }

    accounts[from_idx].balance -= amount;
    accounts[to_idx].balance += amount;
    Ok(())
}

fn foo() -> Result<i32, String> {

    let x: Option<i32> = None;
    let y: Result<i32, String> = {
        x.ok_or_else(|| String::from("x is None"))?;

        if 2 == 2 {
            Ok(2)
        } else {
            Err("2 != 2".to_string())
        }
    };
    y
}

fn main() {
    let mut accounts = vec![
        Account { id: 1, balance: 100 },
        Account { id: 2, balance: 50 },
    ];

    match transfer_funds(&mut accounts, 1, 2, 30) {
        Ok(()) => println!("Transfer successful! New state: {:?}", accounts),
        Err(e) => println!("Transfer failed: {}", e),
    }

    let x = foo().unwrap_or_else(|e| {
        println!("Error: {}", e);
        0
    });
    println!("x = {}", x);
}