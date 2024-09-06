use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_support::pallet_prelude::Get;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((7, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 100);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(7), claim.clone()));
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(7), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(7), claim.clone(), 1));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![3, 2, 1]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(7), claim.clone(), 1),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(7), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 5),
			Error::<Test>::NotClaimOwner
		);
	})
}