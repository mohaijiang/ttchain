use crate::{mock::*};
use pallet_balances::Error as pallet_error;
use frame_support::{assert_ok,assert_noop};

#[test]
fn it_works_for_default_value() {
	ExtBuilder::default().build().execute_with(|| {
		let cid = Vec::from("abc");
		let filename = Vec::from("abc");

		// Dispatch a signed extrinsic.
		assert_ok!(StorageOrder::create_order(Origin::signed(1),cid.clone(),filename.clone(),10,10,100));
		// Read pallet storage and assert an expected result.

		assert_noop!(
			StorageOrder::create_order(Origin::signed(1),cid,filename,11,10,100),
			pallet_error::<Test>::InsufficientBalance
		);
	});
}
