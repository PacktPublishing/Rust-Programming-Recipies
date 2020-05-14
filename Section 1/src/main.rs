extern crate d1_filework;
use d1_filework::{get_first_transaction_for, get_transactions};
//use failure::Error;

fn main() -> anyhow::Result<()> {
    let trans = get_transactions("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    //First trans
    let t = get_first_transaction_for("test_data/transactions.json", "Mate");
    match t {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => println!("Error {}", e),
        //If run on unstable rust you can use the below and get a backtrace, though it
        //may require running with the RUST_BACKTRACE=1 environment variable
        //Err(e) => println!("Error {}, Backtrace = : {}", e, e.backtrace()),
    }

    Ok(())
}
