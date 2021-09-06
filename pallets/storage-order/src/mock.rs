#![cfg(test)]


use crate as storage_order;
use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup,ConvertInto},generic
};
use frame_system as system;

/// 引用元数据
pub use primitives::{
	p_storage_order::OrderPage,
	p_worker::MinerOrderPage,
	constants::{time::*},
	*
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		StorageOrder: storage_order::{Pallet, Call, Storage, Event<T>},
		Payment: payment::{Pallet, Call, Storage, Event<T>},
		Worker: worker::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const OrderWaitingTime: BlockNumber = 30 * MINUTES;
	pub const PerByteDayPrice: u64 = 10;
}

/// storage order Runtime config
impl storage_order::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type OrderWaitingTime = OrderWaitingTime;
	type PerByteDayPrice = PerByteDayPrice;
	type BalanceToNumber = ConvertInto;
	type BlockNumberToNumber = ConvertInto;
	type PaymentInterface = Payment;
}


parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const NumberOfIncomeMiner: usize = 10;
}

/// Configure the payment in pallets/payment.
impl payment::Config for Test {
	type Event = Event;
	type NumberOfIncomeMiner = NumberOfIncomeMiner;
	type BalanceToNumber = ConvertInto;
	type NumberToBalance = ConvertInto;
	type Currency = Balances;
	type StorageOrderInterface = StorageOrder;
	type WorkerInterface = Worker;
}


parameter_types! {
	pub const ReportInterval: BlockNumber = 1 * DAYS;
	//定义文件副本收益限额 eg：前10可获得奖励
	pub const AverageIncomeLimit: u8 = 10;
}

/// storage order Runtime config
impl worker::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type ReportInterval = ReportInterval;
	type BalanceToNumber = ConvertInto;
	type StorageOrderInterface = StorageOrder;
	type AverageIncomeLimit = AverageIncomeLimit;
}


pub struct ExtBuilder {
}
impl Default for ExtBuilder {
	fn default() -> Self {
		Self { }
	}
}
impl ExtBuilder {

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances:
				vec![
					(1, 10 ),
					(2, 20 ),
					(3, 30 ),
					(4, 40 ),
					(12, 10 ),
				]
		}
			.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}