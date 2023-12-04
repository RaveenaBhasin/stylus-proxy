// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::Address, prelude::*, call::delegate_call, msg};


sol_storage! {
    #[entrypoint]
    pub struct Proxy {
        MetaInformation meta_information;
    }

    pub struct MetaInformation {
        address owner; 
        address implementation_address;
    }

}


#[external]
impl Proxy {
    pub fn get_implementation(&self) -> Result<Address, Vec<u8>> {
        let addr = self.meta_information.implementation_address.get();
        Ok(addr)
    }

    pub fn only_owner(&mut self) -> Result<(), Vec<u8>> {
        self.only_owner_impl()?;
        Ok(())
    }
}

impl Proxy {
    pub unsafe fn call_implementation(&mut self, data:Vec<u8>) -> Result<(), Vec<u8>> {
        let implementation_address = self.get_implementation()?;
        delegate_call(self, implementation_address, &data)?;
        Ok(())
    }

    pub fn only_owner_impl(&mut self) -> Result<(), Vec<u8>> {
        let owner = self.meta_information.owner.get();
        if owner != msg::sender() {
            return Err(format!("Invalid").into());
        }
        Ok(())
    }

    pub fn set_owner(&mut self, owner: Address) -> Result<(), Vec<u8>> {
        self.meta_information.owner.set(owner);
        Ok(())
    }
}