#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use schnorrkel::{PublicKey,Signature,signing_context};

#[ink::contract]
mod dependcydemo {
  
    use super::*;
    use ink_prelude::{string::String};
 
    use scale::Encode;

    const SIGNING_CTX: &[u8] = b"substrate";
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dependcydemo {
        /// Stores a single `bool` value on the storage.
        manager:AccountId,
    }

    impl Dependcydemo {
        
        #[ink(constructor)]
       pub fn new(init_account_id: AccountId)->Self {
            Self{
                manager:init_account_id
            }
        }

        #[ink(message)]
        pub fn authorization(&mut self,signature:[u8;64],new_manager:AccountId)->Result<(),String>{
            //just for test 
            let caller = self.env().caller();
            let context = signing_context(b"this signature does this thing");
            let signature = Signature::from_bytes(&signature).unwrap();
            match PublicKey::from_bytes(&caller.encode()) {
                Ok(pk) => {
                    if pk.verify(context.bytes(&new_manager.encode()), &signature).is_ok(){
                        self.manager = new_manager;
                        Ok(())
                    }else{
                        Err(String::from("Signature content does not match verification content!"))
                    }
                },
             Err(_err) => Err(String::from("PublicKey convert")),
            } 
        }
    }
}
