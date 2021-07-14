#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		codec::{Decode, Encode},
		dispatch::DispatchResult, pallet_prelude::*
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive( Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum StorageOrderStatus {
		/// 待处理.
		Pending,
		/// 已完成.
		Finished,
		/// 已取消.
		Canceled,
	}

	impl Default for StorageOrderStatus {
		fn default() -> Self {
			StorageOrderStatus::Pending
		}
	}

	/// A single bid on a gilt, an item of a *queue* in `Queues`.
	#[derive(Encode, Decode, RuntimeDebug,Clone, Eq, PartialEq, Default)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct StorageOrder<AccountId,BlockNumber> {
		/// 订单索引
		pub index: u128,
		/// cid
		pub cid: Vec<u8>,
		/// AccountId
		pub account_id: AccountId,
		/// 文件名
		pub file_name: Vec<u8>,
		/// 支付价格
		pub price: u128,
		/// 存储期限
		pub storage_deadline: BlockNumber,
		/// 文件大小
		pub size: u32,
		/// 块高
		pub block_number: BlockNumber,
		/// 订单状态
		pub status: StorageOrderStatus,
		/// 副本数
		pub replication: u32,
	}

	impl<AccountId, BlockNumber> StorageOrder<AccountId,BlockNumber> {
		fn new (index: u128, cid: Vec<u8>, account_id: AccountId, file_name: Vec<u8>,
			   price: u128, storage_deadline: BlockNumber, size: u32, block_number: BlockNumber) -> Self {
			StorageOrder {
				index,
				cid,
				account_id,
				file_name,
				price,
				storage_deadline,
				size,
				block_number,
				status: StorageOrderStatus::Pending,
				replication: 0,
			}
		}
	}

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// 存储订单个数
	#[pallet::storage]
	#[pallet::getter(fn order_count)]
	pub(super) type OrderCount<T: Config> = StorageValue<_, u128, ValueQuery>;

	// 存储订单信息
	#[pallet::storage]
	#[pallet::getter(fn order_info)]
	pub(super) type OrderInfo<T: Config> = StorageMap<_, Twox64Concat, u128, StorageOrder<T::AccountId,T::BlockNumber>, OptionQuery>;

	// 用户订单个数
	#[pallet::storage]
	#[pallet::getter(fn user_order_count)]
	pub(super) type UserOrderCount<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u128, ValueQuery>;

	// 用户订单数据
	#[pallet::storage]
	#[pallet::getter(fn user_order_index)]
	pub(super) type UserOrderIndex<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u128, u128, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		/// 订单创建
		OrderCreated(u128, Vec<u8>, T::AccountId, Vec<u8>, T::BlockNumber, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_order(origin: OriginFor<T>, cid: Vec<u8>, file_name: Vec<u8>, price: u128,
							duration: T::BlockNumber, size: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			//获取订单长度
			let order_index = OrderCount::<T>::get();
			//获得当前块高
			let block_number = <frame_system::Pallet<T>>::block_number();
			//获得存储期限
			let storage_deadline = block_number + duration;
			//创建订单
			let order = StorageOrder::new(order_index,cid.into(),who.clone(),file_name.into(),price,storage_deadline,size,block_number);
			//存入区块数据
			OrderInfo::<T>::insert(&order_index, order.clone());
			//获得用户索引个数
			let user_order_index = UserOrderCount::<T>::get(&who);
			//存入用户索引数据
			UserOrderIndex::<T>::insert(&who,&user_order_index,order_index);
			//订单长度+1
			let order_index = order_index + 1;
			OrderCount::<T>::put(order_index);
			//用户索引个数+1
			let user_order_index = user_order_index + 1 ;
			UserOrderCount::<T>::insert(&who,user_order_index);
			//发送订单创建事件
			Self::deposit_event(Event::OrderCreated(order.index,order.cid,order.account_id,order.file_name,
													order.storage_deadline,order.size));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
