//! Benchmarking setup for pallet-template

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Pallet as Template;

benchmarks! {
	create_order {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let balance = BalanceOf::<T>::from(12u32);
		let bl = T::BlockNumber::from(10u32);
		let size = 10u32;
	}: _(RawOrigin::Signed(caller),  Vec::from("abc"), Vec::from("abc"),balance,bl,size)
	verify {
		assert_eq!(OrderInfo::<T>::get(s as u64).map(|x| x.index),Some(s as u64));
	}
}

impl_benchmark_test_suite!(
	Template,
	crate::mock::ExtBuilder::default().build(),
	crate::mock::Test,
);
