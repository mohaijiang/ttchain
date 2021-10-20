#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
use sp_std::vec::Vec;
use frame_support::{
	traits::{Currency},
	dispatch::DispatchResult, pallet_prelude::*
};
use primitives::p_computing_power::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// 支付费用和持有余额的货币。
		type Currency: Currency<Self::AccountId>;
	}

	/// 虚拟机信息个数
	#[pallet::storage]
	#[pallet::getter(fn virtual_machine_count)]
	pub(super) type VirtualMachineCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// 存储虚拟机信息
	#[pallet::storage]
	#[pallet::getter(fn virtual_machine_info)]
	pub(super) type VirtualMachineInfo<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, VirtualMachine<T::AccountId,T::BlockNumber>, OptionQuery>;


	///存储索引
	#[pallet::storage]
	#[pallet::getter(fn added_files_count)]
	pub(super) type IndexStorage<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 虚拟机信息创建
		VirtualMachineInfoCreated(Vec<u8>,T::BlockNumber),
		/// 虚拟机信息修改
		VirtualMachineInfoUpdated(T::AccountId,u64,u128),
		/// 虚拟机信息删除
		VirtualMachineInfoDeleted(T::AccountId,T::BlockNumber,u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// 虚拟机信息不存在
		VirtualMachineInfoNotExist,
		///订单ID重复
		VirtualMachineIdRepeat
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		///创建虚机商品信息
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_virtual_machine_info(
			origin: OriginFor<T>,
			id: Vec<u8>,
			server_ipv4: Vec<u8>,
			server_port: u32,
			cpu_cores: u32,
			ram_size: u128,
			disk_size: u128,
			bandwidth: u32,
			server_period: u32,
			price: u128,
			operating_system: Vec<u8>
		) -> DispatchResult {
			//判断是否签名正确
			let who = ensure_signed(origin)?;
			//校验id是否重复
			if let Some(order_info) = VirtualMachineInfo::<T>::get(id.clone()){
				Err(Error::<T>::VirtualMachineIdRepeat)?
			}
			//获取虚拟机商品个数
			let virtual_machine_index = VirtualMachineCount::<T>::get();
			//获取索引
			// let index_value = IndexStorage::<T>::get();
			//获得当前块高
			let block_number = <frame_system::Pallet<T>>::block_number();
			//创建订单
			let virtual_machine_info = VirtualMachine::new(
				id.clone(),
				who.clone(),
				server_ipv4.clone(),
				server_port.clone(),
				cpu_cores.clone(),
				ram_size.clone(),
				disk_size.clone(),
				bandwidth.clone(),
				server_period.clone(),
				price.clone(),
				operating_system.clone(),
				block_number.clone());
			//存入虚拟机商品信息
			VirtualMachineInfo::<T>::insert(id.clone(), virtual_machine_info.clone());
			//订单长度+1
			VirtualMachineCount::<T>::put(virtual_machine_index + 1);
			//索引+1
			// IndexStorage::<T>::put(index_value + 1);
			//发送虚机商品创建事件
			Self::deposit_event(Event::VirtualMachineInfoCreated(virtual_machine_info.id,virtual_machine_info.block_number));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}


}


impl<T: Config> Pallet<T> {

	pub fn get_virtual_machine_info(id: Vec<u8>) -> ExitVirtualMachine {
		let params:Vec<u8> = id.to_vec();
		let mut exit_flag = false;
		if let Some(virtual_machine_info) = VirtualMachineInfo::<T>::get(params){
			exit_flag = true
		}
		let exitVirtualMachine = ExitVirtualMachine::new(exit_flag);
		exitVirtualMachine
	}
}
