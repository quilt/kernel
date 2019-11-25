use crate::error::Error;
use crate::host::{eth2_callModule, eth2_loadModule};
use crate::state::State;
use crate::transaction::Transaction;

type A = [u8; 20];

pub fn process_raw_transactions<'a, K, V, T: State<A, K, V>>(
    db: &mut T,
    mut transactions: &[u8],
) -> Result<(), Error<A>> {
    while transactions.len() > 0 {
        let tx = Transaction::from_ptr(transactions.as_ptr());
        transactions = &transactions[tx.length() as usize..];

        match db.code(tx.to()) {
            Ok(code) => unsafe {
                eth2_loadModule(0, code.as_ptr() as *const u32, code.len() as u32);
                eth2_callModule(
                    0,
                    "main".as_ptr() as *const u32,
                    4,
                    tx.data().as_ptr() as *const u32,
                    tx.data().len() as u32,
                    0 as *const u32,
                    0,
                );
            },
            Err(_) => panic!("couldn't find code"),
        }
    }

    Ok(())
}
