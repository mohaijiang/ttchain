#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
use sp_std::vec::Vec;
use frame_support::{traits::{Currency,ExistenceRequirement,ExistenceRequirement::{AllowDeath, KeepAlive}},PalletId};
use sp_runtime::{traits::AccountIdConversion};
use frame_support::dispatch::DispatchResult;
use frame_support::sp_runtime::traits::Convert;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::Currency;
	use super::*;
	use frame_support::sp_runtime::traits::Convert;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// 可以分润的矿工账户数量
		type NumberOfIncomeMiner: Get<usize>;

		/// 金额转换数字
		type BalanceToNumber: Convert<BalanceOf<Self>, u128>;
		// 数字转金额
		type NumberToBalance: Convert<u128,BalanceOf<Self>>;
		/// 支付费用和持有余额的货币。
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	/// 订单金额
	#[pallet::storage]
	#[pallet::getter(fn order_price)]
	pub(super) type OrderPrice<T: Config> = StorageMap<_, Twox64Concat, u64, BalanceOf<T>, OptionQuery>;

	/// 订单到期记录
	#[pallet::storage]
	#[pallet::getter(fn order_deadline)]
	pub(super) type OrderDeadline<T: Config> = StorageMap<_,Twox64Concat,T::BlockNumber,Vec<u64>,OptionQuery>;

	/// 矿工待领取金额
	#[pallet::storage]
	#[pallet::getter(fn miner_price)]
	pub(super) type MinerPrice<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		DonationReceived(T::AccountId, BalanceOf<T>, BalanceOf<T>),

		Withdrawal(T::AccountId, BalanceOf<T>, BalanceOf<T>),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {

			let generation  = 100 as u32;

			//判断当前块高是否大于订单等待时长
			let order_deadline_set = OrderDeadline::<T>::get(now).unwrap_or(Vec::<u64>::new());
			for order_index in &order_deadline_set {

				//TODO...校验文件状态 如果文件状态为完成，进行清算

				//获取订单金额
				match OrderPrice::<T>::get(order_index) {
					Some(price) => {

						/// 世代订单应发放金额逻辑，备用
						// // 订单创建区块
						// let order_create_block_number = 0 as u128;
						// // 订单存储时长
						// let duration = 100 as u128;
						// // 上一次世代区块
						// let last_generation = now - generation;
						// // 订单完成区块
						// let order_deadline = 128 as u128;
						// //本次区块占比
						// let mut generation_price = 0 as u128;
						// if order_create_block_number < last_generation {	// 前世代创建订单
						// 	if order_deadline <= now {		//当前世代完成订单
						// 		if order_deadline > last_generation {
						// 			generation_price = price * (order_deadline - last_generation) / duration;
						// 		}
						// 	}else {  //当前世代无法完成订单
						// 		generation_price =  price * (now - order_create_block_number) / duration
						// 	}
						// } else { 	// 当前世代创建订单
						// 	if order_deadline <= now {		//当前世代完成订单
						// 		generation_price = price;
						// 	}else {  //当前世代无法完成订单
						// 		generation_price = price * generation / duration;
						// 	}
						// }


						//获取订单矿工集合
						let mut miners : Vec<T::AccountId> = Vec::<T::AccountId>::new();
						//截取前10个订单完成者，有权利分润
						miners.truncate(T::NumberOfIncomeMiner::get());
						// 计算实际完成者数量
						let workers = miners.len();
						//总订单金额u128
						let price_u128 = T::BalanceToNumber::convert(price.clone());
						// 计算每人可分配金额
						let per_worker_income = price_u128/(workers as u128);
 						// 矿工循环，计算收益
						for mut miner in &miners {

							match MinerPrice::<T>::get(miner) {
								Some(t) => {
									let income_after = T::NumberToBalance::convert(T::BalanceToNumber::convert(t)+ per_worker_income);
									MinerPrice::<T>::insert(miner,income_after);
								}
								None => {
									MinerPrice::<T>::insert(miner, T::NumberToBalance::convert(per_worker_income));
								}
							}
						}
					}
					None => {}
				}
				//获取订单矿工集合

				// 调用订单清算
				/*match OrderInfo::<T>::get(order_index) {
					Some(mut order_info) => {
						if let StorageOrderStatus::Pending = order_info.status {
							order_info.status = StorageOrderStatus::Canceled;
							OrderInfo::<T>::insert(order_index,order_info.clone());
							//发送订单取消时间事件
							Self::deposit_event(Event::OrderCanceled(order_index.clone() , order_info.cid));
						}
					},
					None => ()
				}*/
			}

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
		/// 订单金额已经配置
		StorageOrderPriceSetted
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
		pub fn donate(
			origin : OriginFor<T>,
			amount: BalanceOf<T>
		) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			let _ = T::Currency::transfer(&donor, &Self::account_id(), amount, ExistenceRequirement::AllowDeath);

			Self::deposit_event(Event::DonationReceived(donor, amount, Self::pot()));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn withdrawal2(
			origin : OriginFor<T>,
			amount: BalanceOf<T>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			//查询矿工待领取金额
			let price_opt = MinerPrice::<T>::get(&who);

			if price_opt.is_some() {
				let amount :BalanceOf<T> = price_opt.unwrap();

				if T::BalanceToNumber::convert(amount) > 0 {
					&Self::withdrawal(&who,amount)?;
					MinerPrice::<T>::remove(&who);
					Self::deposit_event(Event::Withdrawal(who, amount, Self::pot()));
				}
			}
			Ok(())
		}
	}
}

const PALLET_ID: PalletId = PalletId(*b"ttchain!");

impl <T:Config> Pallet<T> {
	/// The account ID that holds the Charity's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}

	fn withdrawal(account_id: &T::AccountId,amount: BalanceOf<T>) -> DispatchResult{
		T::Currency::transfer(&Self::account_id(),account_id,amount, ExistenceRequirement::AllowDeath)
	}
}



pub trait PaymentInterface {
	type AccountId;
	type BlockNumber;
	type Balance;
	/// 记录订单金额
	fn pay_order(order_index: &u64, order_price: &Self::Balance,deadline: &Self::BlockNumber, account_id: &Self::AccountId) -> DispatchResult;
	/// 订单取消退款
	fn cancel_order(order_index: &u64, order_price: &u128,deadline: &Self::BlockNumber, account_id: &Self::AccountId);
}

impl<T: Config> PaymentInterface for Pallet<T> {
	type AccountId = T::AccountId;
	type BlockNumber = T::BlockNumber;
	type Balance = BalanceOf<T>;

	fn pay_order(order_index: &u64, order_price: &Self::Balance,deadline: &Self::BlockNumber, account_id: &Self::AccountId) -> DispatchResult{

		match OrderPrice::<T>::get(order_index) {
			// Return an error if the value has not been set.
			None => {
				// 记录订单金额
				OrderPrice::<T>::insert(order_index,order_price);
				//记录订单到期区块
				let mut order_deadline_set = OrderDeadline::<T>::get(&deadline).unwrap_or(Vec::<u64>::new());
				order_deadline_set.push(*order_index);
				OrderDeadline::<T>::insert(&deadline,order_deadline_set);

				// 转账用户订单金额
				T::Currency::transfer(&account_id, &Self::account_id(), *order_price, ExistenceRequirement::AllowDeath)
			},
			Some(old) => {
            	// 已有订单金额，理论上不可能，暂时不修改数据
				Err(Error::<T>::NoneValue)?
			},
		}
	}

	fn cancel_order(order_index: &u64,order_price: &u128,deadline: &Self::BlockNumber, account_id: &Self::AccountId){
		match OrderPrice::<T>::get(order_index) {
			Some(old) => {
				let dispatch_result = T::Currency::transfer(&Self::account_id(),&account_id, T::NumberToBalance::convert(*order_price), ExistenceRequirement::AllowDeath);
				if dispatch_result.is_ok() {
					//记录订单到期区块
					let mut order_deadline_set = OrderDeadline::<T>::get(&deadline).unwrap_or(Vec::<u64>::new());
					order_deadline_set.retain(|&x| x == *order_index);
					OrderDeadline::<T>::insert(&deadline,order_deadline_set);
					OrderPrice::<T>::remove(order_index);
				}

			},
			None => ()
		}
	}
}