#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod dependcydemo {
    use ink_core::storage;
    use ink_prelude::{format, vec::Vec, string::String};

    use schnorrkel::PublicKey;
    use scale::{Decode, Encode};


    const SIGNING_CTX: &[u8] = b"substrate";
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct Dependcydemo {
        /// Stores a single `bool` value on the storage.
        manager: storage::Value<AccountId>,
    }

    impl Dependcydemo {
        
        #[ink(constructor)]
        fn new(&mut self, init_account_id: AccountId) {
            self.manager.set(init_account_id);
        }
      
        #[ink(message)]
        fn authorization(&mut self,signature:[u8;64],new_manager:AccountId)->Result<(),String>{
            //just for test 
            let caller = self.env().caller();
            match PublicKey::from_bytes(&caller.encode()) {
                Ok(pk) => {
                    if pk.verify_simple_preaudit_deprecated(
                        SIGNING_CTX, &new_manager.encode(), &signature[..],
                    ).is_ok(){
                        self.manager.set(new_manager);
                        Ok(())
                    }else {
                        Err(String::from("Signature content does not match verification content!"))
                    }
                },
             Err(_err) => Err(String::from("PublicKey convert")),
            } 
        }

        
      /*   #[ink(message)]
        fn get(&self) -> bool {
            *self.value
        } */
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            // Note that even though we defined our `#[ink(constructor)]`
            // above as `&mut self` functions that return nothing we can call
            // them in test code as if they were normal Rust constructors
            // that take no `self` argument but return `Self`.
            let dependcydemo = Dependcydemo::default();
            assert_eq!(dependcydemo.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut dependcydemo = Dependcydemo::new(false);
            assert_eq!(dependcydemo.get(), false);
            dependcydemo.flip();
            assert_eq!(dependcydemo.get(), true);
        }
    }
}
