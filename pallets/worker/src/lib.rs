#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
use sp_std::vec::Vec;
use frame_support::{
	traits::{Currency},
	ensure,
	codec::{Decode, Encode},
	dispatch::DispatchResult, pallet_prelude::*
};
use sp_runtime::traits::Convert;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use storage_order::StorageOrderInterface;
use storage_order::StorageOrderStatus;


#[frame_support::pallet]
pub mod pallet {
	use frame_system::pallet_prelude::*;
	use super::*;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// 工作量证明上报间隔
		type ReportInterval: Get<Self::BlockNumber>;

		/// 支付费用和持有余额的货币。
		type Currency: Currency<Self::AccountId>;

		/// 金额转换数字
		type BalanceToNumber: Convert<BalanceOf<Self>, u128>;

		/// 订单接口
		type StorageOrderInterface: StorageOrderInterface<AccountId = Self::AccountId, BlockNumber = Self::BlockNumber>;
	}


	#[derive(Encode, Decode, RuntimeDebug,Clone, Eq, PartialEq, Default)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct ReportInfo {
		/// 订单索引
		pub orders: Vec<u64>,
		/// 总存储量
		pub total_storage: u64,
		/// 已用存储
		pub used_storage: u64
	}

	impl ReportInfo {
		fn new (orders: Vec<u64>, total_storage: u64, used_storage: u64) -> Self {
			ReportInfo {
				orders,
				total_storage,
				used_storage
			}
		}
	}

	/// 矿工个数
	#[pallet::storage]
	#[pallet::getter(fn miner_count)]
	pub(super) type MinerCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// 矿工列表
	#[pallet::storage]
	#[pallet::getter(fn miners)]
	pub(super) type Miners<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	/// 总存储
	#[pallet::storage]
	#[pallet::getter(fn total_storage)]
	pub(super) type TotalStorage<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// 已用存储
	#[pallet::storage]
	#[pallet::getter(fn used_storage)]
	pub(super) type UsedStorage<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// 矿工收益
	#[pallet::storage]
	#[pallet::getter(fn miner_income)]
	pub(super) type MinerIncome<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	/// 矿工总存储
	#[pallet::storage]
	#[pallet::getter(fn miner_total_storage)]
	pub(super) type MinerTotalStorage<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	/// 矿工已用存储
	#[pallet::storage]
	#[pallet::getter(fn miner_used_storage)]
	pub(super) type MinerUsedStorage<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	/// 矿工订单数据
	#[pallet::storage]
	#[pallet::getter(fn miner_order)]
	pub(super) type MinerOrder<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, ValueQuery>;

	/// 订单对应矿工集合
	#[pallet::storage]
	#[pallet::getter(fn miner_set_of_order)]
	pub(super) type MinerSetOfOrder<T: Config> = StorageMap<_, Twox64Concat, u64, Vec<T::AccountId>, ValueQuery>;

	/// 时空证明报告
	#[pallet::storage]
	#[pallet::getter(fn report)]
	pub(super) type Report<T: Config> = StorageDoubleMap<_, Twox64Concat, T::BlockNumber, Twox64Concat, T::AccountId, ReportInfo, OptionQuery>;



	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// 复制证明完成
		ProofOfReplicationFinish(u64),
		/// 注册成功
		RegisterSuccess(T::AccountId),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			0
		}
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
		pub fn register(
			origin: OriginFor<T>,
			total_storage: u64,
			used_storage: u64
		) -> DispatchResult {
			//判断是否签名正确
			let who = ensure_signed(origin)?;
			//查询当前矿工节点
			let mut miners = Miners::<T>::get();
			//遍历矿工是否存在，如果存在则进行覆盖操作，如果不存在则进行添加
			match miners.binary_search(&who) {
				Ok(_) => {
					let old_miner_total_storage = MinerTotalStorage::<T>::get(&who);
					let old_miner_used_storage = MinerUsedStorage::<T>::get(&who);
					//添加矿工总存储
					MinerTotalStorage::<T>::insert(&who,total_storage);
					//添加矿工已用存储
					MinerUsedStorage::<T>::insert(&who,used_storage);
					//添加矿工总存储
					let total_storage = TotalStorage::<T>::get() - old_miner_total_storage + total_storage;
					TotalStorage::<T>::put(total_storage);
					//添加矿工已用存储
					let used_storage = UsedStorage::<T>::get() - old_miner_used_storage + used_storage;
					UsedStorage::<T>::put(used_storage);
				},
				Err(index) => {
					//添加矿工
					miners.insert(index, who.clone());
					Miners::<T>::put(miners);
					//矿工个数+1
					let count = MinerCount::<T>::get();
					MinerCount::<T>::put(count + 1);
					//添加矿工总存储
					MinerTotalStorage::<T>::insert(&who,total_storage);
					//添加矿工已用存储
					MinerUsedStorage::<T>::insert(&who,used_storage);
					//添加矿工总存储
					let total_storage = TotalStorage::<T>::get() + total_storage;
					TotalStorage::<T>::put(total_storage);
					//添加矿工已用存储
					let used_storage = UsedStorage::<T>::get() + used_storage;
					UsedStorage::<T>::put(used_storage);
				}
			}
			Self::deposit_event(Event::RegisterSuccess(who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn proof_of_replication(
			origin: OriginFor<T>,
			miner: T::AccountId,
			order_index: u64,
			cid: Vec<u8>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//校验是否为矿工
			ensure!(&who == &miner, Error::<T>::IllegalMiner);
			//获取订单
			let order_info = T::StorageOrderInterface::get_storage_order(&order_index).ok_or(Error::<T>::OrderDoesNotExist)?;
			//检验文件cid是否正确
			ensure!(&order_info.cid == &cid, Error::<T>::IllegalFileCID);
			//校验文件状态 如果文件状态为待处理则改为已完成
			match &order_info.status {
				StorageOrderStatus::Canceled => Err(Error::<T>::OrderDoesNotExist)?,
				_ => (),
			}
			//判断订单是否已经提交
			let mut miners = MinerSetOfOrder::<T>::get(&order_index);
			//遍历矿工是否存在，如果存在则报已经完成订单，如果不存在则进行添加
			match miners.binary_search(&miner) {
				Ok(_) => Err(Error::<T>::AlreadyCallOrderFinish)?,
				Err(index) => {
					miners.insert(index, miner.clone());
					MinerSetOfOrder::<T>::insert(&order_index,miners);
				}
			}
			//添加订单副本
			T::StorageOrderInterface::add_order_replication(&order_index);
			//存入矿工订单数据
			let mut orders = MinerOrder::<T>::get(&miner);
			orders.push(order_index);
			MinerOrder::<T>::insert(&miner,orders);

			//发送订单完成事件
			Self::deposit_event(Event::ProofOfReplicationFinish(order_index));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn proof_of_spacetime(
			origin: OriginFor<T>,
			orders: Vec<u64>,
			total_storage: u64,
			used_storage: u64
		) -> DispatchResult {
			todo!()
		}
	}
}
