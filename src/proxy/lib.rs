// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::Address, prelude::*, call::delegate_call, msg};
// mod counter;
// use crate::counter::Counter;

sol_storage! {
    #[entrypoint]
    pub struct Proxy {
        bool is_initialized;
        MetaInformation meta_information;

        // #[borrow]
        // Counter counter;
    }

    pub struct MetaInformation {
        address owner; 
        address implementation_address;
    }

}


#[external]
// #[inherit(Counter)]
impl Proxy {
    pub fn init(&mut self, owner: Address) -> Result<(), Vec<u8>> {
        if self.is_initialized.get() {
            return Err(format!("Already initialized").into());
        }
        self.meta_information.owner.set(owner);
        self.is_initialized.set(true);
        Ok(())
    }
    
    pub fn get_implementation(&self) -> Result<Address, Vec<u8>> {
        let addr = self.meta_information.implementation_address.get();
        Ok(addr)
    }

    pub fn set_implementation(&mut self, implementation: Address) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self.meta_information.implementation_address.set(implementation);
        Ok(())
    }

    // pub fn only_owner_function(&mut self) -> Result<(), Vec<u8>> {
    //     self.only_owner()?;
    //     Ok(())
    // }
}

impl Proxy {
    pub unsafe fn call_implementation(&mut self, data:Vec<u8>) -> Result<(), Vec<u8>> {
        let implementation_address = self.get_implementation()?;
        delegate_call(self, implementation_address, &data)?;
        Ok(())
    }

    pub fn only_owner(&mut self) -> Result<(), Vec<u8>> {
        let owner = self.meta_information.owner.get();
        if owner != msg::sender() {
            return Err(format!("Invalid").into());
        }
        Ok(())
    }

}