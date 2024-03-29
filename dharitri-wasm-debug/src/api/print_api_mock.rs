use crate::{BigUintPrinter, DebugApi};
use dharitri_wasm::{api::PrintApi, types::BigUint};

impl PrintApi for DebugApi {
    fn print_biguint(&self, biguint: &BigUint<Self>) {
        println!(
            "{:?}",
            BigUintPrinter {
                value: biguint.clone()
            }
        );
    }
}
