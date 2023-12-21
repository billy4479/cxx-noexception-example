use anyhow::{anyhow, Result};

pub fn only_even(num: i32) -> Result<()> {
    if num % 2 == 0 {
        Ok(())
    } else {
        Err(anyhow!("Only even numbers!"))
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn only_even(num: i32) -> Result<()>;
    }
}
