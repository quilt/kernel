use arborist::Tree;
use bonsai::{log2, subtree_index_to_general};
use interface::{Address, Contract};
use std::{cmp::min, collections::BTreeMap, mem::transmute};
use wabt::wat2wasm;

#[derive(Clone)]
pub struct HelloWorld {
    asm: Vec<u8>,
}

impl Contract for HelloWorld {
    fn new() -> Self {
        let asm = wat2wasm(
            r#"
            (module
                (import "env" "print" (func $print (param i32) (param i32)))
                (import "env" "eth2_return" (func $return (param i32) (param i32) (result i32)))
                (memory (export "memory") 1)
                (data (i32.const 1000) "hello world")
                (func $main (export "main") (result i32)
                    (call $print (i32.const 1000) (i32.const 11))

                    (; Return a value to the caller ;)
                    (i32.store (i32.const 10) (i32.const 42))
                    (call $return (i32.const 10) (i32.const 4))
                )
            )
        "#,
        )
        .unwrap();

        Self { asm }
    }

    fn asm(&self) -> &[u8] {
        &self.asm
    }

    fn to_map(&self) -> BTreeMap<u128, [u8; 32]> {
        let len = self.asm().len() as u128;
        let mut tree = Tree::new();

        // insert data root
        tree.insert(2, u32_to_value(0));

        // insert length mix-in
        tree.insert(7, u32_to_value(len as u32));

        // insert code chunks by index
        tree.insert_bytes(6, self.asm().to_vec());

        println!("len (from contract wrapper): {}", self.asm().len());

        tree.to_map()
    }
}

fn u32_to_value(n: u32) -> [u8; 32] {
    let mut buf = [0u32; 8];
    buf[0] = n;
    unsafe { transmute::<[u32; 8], [u8; 32]>(buf) }
}

#[cfg(test)]
mod test {
    use super::*;
    use arrayref::array_ref;
    use oof::Oof;

    fn build_value(n: &u32) -> Vec<u8> {
        u32_to_value(*n).to_vec()
    }
}
