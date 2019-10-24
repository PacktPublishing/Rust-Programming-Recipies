extern crate d1_filework;
use d1_filework::{get_transactions,get_first_transaction_for};
use failure::Error;

fn main() -> Result<(), Error> {
    let trans = get_transactions("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    //First trans
    let t = get_first_transaction_for("test_data/transactions.json", "Mate");
    match t {
        Ok(v) => println!("Found transaction: {:?}",v),
        Err(e)=> println!("Error {}, Backtrace = : {}",e, e.backtrace()),
    }

    Ok(())
}

