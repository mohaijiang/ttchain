#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
use sp_std::vec::Vec;
pub use primitives::p_computing_power::ExitVirtualMachine;
use codec::Codec;

sp_api::decl_runtime_apis! {

    pub trait VirtualMachineApi<AccountId,BlockNumber,Balance> where
        AccountId: Codec,
        BlockNumber: Codec,
        Balance: Codec
    {
		fn get_virtual_machine_info(id:Vec<u8>) -> ExitVirtualMachine;
    }
}
