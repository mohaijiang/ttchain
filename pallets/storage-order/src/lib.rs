#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		ensure,
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

		/// 订单等待时间
		type OrderWaitingTime: Get<Self::BlockNumber>;
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

	// 矿工订单个数
	#[pallet::storage]
	#[pallet::getter(fn miner_order_count)]
	pub(super) type MinerOrderCount<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u128, ValueQuery>;

	// 矿工订单数据
	#[pallet::storage]
	#[pallet::getter(fn miner_order_index)]
	pub(super) type MinerOrderIndex<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, u128, u128, OptionQuery>;

	// 块高存储订单集合
	#[pallet::storage]
	#[pallet::getter(fn order_set_of_block)]
	pub(super) type OrderSetOfBlock<T: Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<u128>, OptionQuery>;

	// 订单对应矿工集合
	#[pallet::storage]
	#[pallet::getter(fn miner_set_of_order)]
	pub(super) type MinerSetOfOrder<T: Config> = StorageMap<_, Twox64Concat, u128, Vec<T::AccountId>, OptionQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 订单创建
		OrderCreated(u128, Vec<u8>, T::AccountId, Vec<u8>, T::BlockNumber, u32),
		/// 订单完成
		OrderFinish(u128),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// 非法矿工
		IllegalMiner,
		/// 非法文件CID
		IllegalFileCID,
		/// 订单已经取消
		OrderCancelled,
		/// 已经调用订单完成
		AlreadyCallOrderFinish,
		/// 订单不存在
		OrderDoesNotExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_order(
			origin: OriginFor<T>,
			cid: Vec<u8>,
			file_name: Vec<u8>,
			price: u128,
			duration: T::BlockNumber,
			size: u32
		) -> DispatchResult {
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
			let order = StorageOrder::new(
				order_index,
				cid.clone(),
				who.clone(),
				file_name.clone(),
				price,
				storage_deadline,
				size,
				block_number.clone());
			//存入区块数据
			OrderInfo::<T>::insert(&order_index, order.clone());
			//获得用户索引个数
			let user_order_index = UserOrderCount::<T>::get(&who);
			//存入用户索引数据
			UserOrderIndex::<T>::insert(&who,&user_order_index,order_index);
			//订单长度+1
			OrderCount::<T>::put(order_index + 1);
			//用户索引个数+1
			UserOrderCount::<T>::insert(&who,user_order_index + 1);
			//添加块高存储订单集合
			let mut order_set = OrderSetOfBlock::<T>::get(&block_number).unwrap_or(Vec::<u128>::new());
			order_set.push(order_index);
			OrderSetOfBlock::<T>::insert(&block_number,order_set);
			//发送订单创建事件
			Self::deposit_event(Event::OrderCreated(order.index,order.cid,order.account_id,order.file_name,
													order.storage_deadline,order.size));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn finish_order(
			origin: OriginFor<T>,
			miner: T::AccountId,
			order_index: u128,
			cid: Vec<u8>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//校验是否为矿工
			ensure!(&who == &miner, Error::<T>::IllegalMiner);
			//获取订单
			let mut order_info = OrderInfo::<T>::get(&order_index).ok_or(Error::<T>::OrderDoesNotExist)?;
			//检验文件cid是否正确
			ensure!(&order_info.cid == &cid, Error::<T>::IllegalFileCID);
			//校验文件状态 如果文件状态为待处理则改为已完成
			match &order_info.status {
				StorageOrderStatus::Canceled => Err(Error::<T>::OrderDoesNotExist)?,
				StorageOrderStatus::Pending => order_info.status = StorageOrderStatus::Finished,
				_ => (),
			}
			//判断订单是否已经提交
			let mut miners = MinerSetOfOrder::<T>::get(&order_index).unwrap_or(Vec::<T::AccountId>::new());
			//遍历矿工是否存在，如果存在则报已经完成订单，如果不存在则进行添加
			match miners.binary_search(&miner) {
				// If the search succeeds, the caller is already a miners, so just return
				Ok(_) => Err(Error::<T>::AlreadyCallOrderFinish)?,
				// If the search fails, the caller is not a miners and we learned the index where
				// they should be inserted
				Err(index) => {
					miners.insert(index, miner.clone());
					MinerSetOfOrder::<T>::insert(&order_index,miners);
				}
			}
			//订单信息副本数+1
			order_info.replication = order_info.replication + 1;
			//维护订单信息
			OrderInfo::<T>::insert(&order_index,order_info);
			//获得矿工索引个数
			let miner_order_index = MinerOrderCount::<T>::get(&miner);
			//存入矿工索引数据
			MinerOrderIndex::<T>::insert(&miner,&miner_order_index,order_index);
			//矿工索引个数+1
			MinerOrderCount::<T>::insert(&miner,miner_order_index + 1);

			//发送订单完成事件
			Self::deposit_event(Event::OrderFinish(order_index));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
