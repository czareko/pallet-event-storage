use crate::mock::*;
use frame_support::{assert_ok};
use frame_support::storage::KeyPrefixIterator;
use frame_support::traits::{OffchainWorker};

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
		let test_content = "random string".to_string();
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
//Helper fake block generating function
fn run_to_next_block() {
	System::set_block_number(System::block_number() + 1);
	EventStorageModule::offchain_worker(System::block_number());
}