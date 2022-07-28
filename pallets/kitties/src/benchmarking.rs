//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as KittiesModule;
use frame_benchmarking::{ account, benchmarks, whitelisted_caller };
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {
		let dna = b"icebear".to_vec();
		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty(RawOrigin::Signed(caller), dna.clone())
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	create_kitty_random {
		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty_random(RawOrigin::Signed(caller))
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer {
		let sender: T::AccountId = whitelisted_caller();
		let dna = b"icebear".to_vec();
		KittiesModule::<T>::create_kitty(RawOrigin::Signed(sender.clone()).into(), dna.clone());
		let kitty = KittiesOwned::<T>::get(&sender);
		let receiver: T::AccountId = account("bob", 0, 0);
	}: transfer(RawOrigin::Signed(sender), receiver.clone(), dna)

	verify {
		assert_eq!(KittiesOwned::<T>::get(&receiver), kitty);
	}

	impl_benchmark_test_suite!(KittiesModule, crate::mock::new_test_ext(), crate::mock::Test);
}
