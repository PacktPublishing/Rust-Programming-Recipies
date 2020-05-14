mod error;
pub use error::TransactionError;
use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> anyhow::Result<Transaction> {
    let trans = get_transactions(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err(TransactionError::Mess("Could not find transaction with that name").into())
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    /*std::fs::read_to_string(fname)
    .map_err(|e| e.into())
    .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
    */

    /*Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )*/
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}
