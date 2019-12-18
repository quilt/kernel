use interface::{Address, RawTransaction};

use crate::{error::Error, state::State};

#[cfg(not(test))]
use crate::host::{eth2_callModule, eth2_loadModule, print};

#[cfg(not(test))]
pub fn process_raw_transactions<'a, K, V, T: State<K, V>>(
    db: &mut T,
    mut transactions: &[u8],
) -> Result<(), Error<Address>> {
    while transactions.len() > 0 {
        let tx = RawTransaction::from_ptr(transactions.as_ptr());
        transactions = &transactions[tx.len() as usize..];

        let msg = "Preparing to get code";

        #[cfg(not(test))]
        unsafe {
            print(msg.as_ptr() as *const u32, msg.len() as u32);
        }

        match db.code(tx.to()) {
            Ok(code) => unsafe {
                let msg = "got code!!";

                #[cfg(not(test))]
                print(msg.as_ptr() as *const u32, msg.len() as u32);

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
