use crate::mock::*;
use frame_support::{assert_ok};
use frame_support::storage::KeyPrefixIterator;
use frame_support::traits::{OffchainWorker};
use rand::Rng;
use std::{thread, time::Duration};

/*
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(EventStorageModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(EventStorageModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(EventStorageModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

*/
/***
Storage tests
*/
#[test]
fn should_create_custom_event(){
	new_test_ext().execute_with(|| {
		//when
		assert_eq!(EventStorageModule::get_storage_size(),Some(0));
		//given
		for i in 0..25{
			let st = format!("ABC: {}",i);
			assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),st));
		}
		//then
		assert_eq!(EventStorageModule::get_storage_size(),Some(25));
		run_to_next_block();
	});
}
#[test]
fn should_find_items_by_key(){
	new_test_ext().execute_with(|| {
		//given
		let test_content = generate_random_content();
		assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),test_content.clone()));
		let mut keys: KeyPrefixIterator<i64> = EventStorageModule::get_custom_event_keys().unwrap();
		//then
		// there is only one item
		assert_eq!(keys.by_ref().count(),1);
		// and
		// we can find only this item
		for key in keys.by_ref(){
			let event = EventStorageModule::get_custom_event(key).unwrap();
			assert_eq!(event.content,test_content);
		}
	});
}
/***
Automated history removal
*/
#[test]
fn should_remove_old_events(){
	new_test_ext().execute_with(|| {

		//when
		for _i in 0..25 {
			assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),generate_random_content()));
		}
		//given
		// 25 events in my storage
		let keys: KeyPrefixIterator<i64> = EventStorageModule::get_custom_event_keys().unwrap();
		assert_eq!(keys.count(),25);

		//time from pallet mock configuration
		thread::sleep(Duration::from_secs(HistorySize::get().try_into().unwrap()));
		//then
		//all events should be removed after the next blog finalization
		run_to_next_block();
		let keys: KeyPrefixIterator<i64> = EventStorageModule::get_custom_event_keys().unwrap();
		assert_eq!(keys.count(),0);

	});
}


fn generate_random_content()->String{
	let mut rng = rand::thread_rng();
	//println!("Integer: {}", rng.gen_range(0..10000));
	//let content = "Content ".to_string();
	format!("Content {}",rng.gen_range(0..10000))
}

//Helper fake block generating function
fn run_to_next_block() {
	System::set_block_number(System::block_number() + 1);
	EventStorageModule::offchain_worker(System::block_number());
}