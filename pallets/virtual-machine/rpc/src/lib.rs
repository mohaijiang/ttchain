//! RPC interface for the transaction payment module.

use std::convert::TryInto;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT, MaybeDisplay}};
use std::sync::Arc;
use codec::Codec;
use sp_rpc::number::NumberOrHex;
use virtual_machine_runtime_api::ExitVirtualMachine;
pub use virtual_machine_runtime_api::VirtualMachineApi as VirtualMachineRuntimeApi;
use sp_core::{
	Bytes
};


#[rpc]
pub trait VirtualMachineApi<Block, AccountId, BlockNumber, Balance> {

	#[rpc(name = "virtualMachine_getVirtualMachineInfo")]
	fn get_virtual_machine_info(&self,id: Bytes) -> Result<ExitVirtualMachine>;

}

/// A struct that implements the `VirtualMachineInfoApi`.
pub struct VirtualMachine<C, M> {
    // If you have more generics, no need to StorageOrder<C, M, N, P, ...>
    // just use a tuple like StorageOrder<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> VirtualMachine<C, M> {
    /// Create new `VirtualMachine` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block, AccountId, BlockNumber, Balance> VirtualMachineApi<<Block as BlockT>::Hash,AccountId,BlockNumber,Balance> for VirtualMachine<C, Block>
    where
        Block: BlockT,
        C: Send + Sync + 'static,
        C: ProvideRuntimeApi<Block>,
        C: HeaderBackend<Block>,
        C::Api: VirtualMachineRuntimeApi<Block, AccountId, BlockNumber, Balance>,
        AccountId: Clone + std::fmt::Display + Codec,
        BlockNumber: Clone + std::fmt::Display + Codec,
        Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex> + std::ops::Add<Output = Balance>,
{
	fn get_virtual_machine_info(&self,id: Bytes) -> Result<ExitVirtualMachine> {
		let api = self.client.runtime_api();
		let best = self.client.info().best_hash;
		let at = BlockId::hash(best);
		let parm =  id.to_vec();
		let runtime_api_result = api.get_virtual_machine_info(&at,parm);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
