use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::storage::KeyPrefixIterator;
use frame_support::traits::{OffchainWorker};
use rand::Rng;
use std::{thread, time::Duration};

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
			let st = format!("ABC: {}",i).as_bytes().to_vec();
			assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),st));
		}
		//then
		assert_eq!(EventStorageModule::get_storage_size(),Some(25));
		run_to_next_block();
	});
}
/***
Tests item search, it's not exposed outside but very useful to be sure if we know what we want to delete and what we saved before.
*/
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
#[test]
fn should_thow_error_for_unauthorized_users(){
	new_test_ext().execute_with(|| {
		//given
		let account_id: u64 = 2;
		//then
		assert_noop!(EventStorageModule::create_custom_event(Origin::signed(account_id),generate_random_content()), Error::<Test>::UnauthorizedCaller);
	});

}

//Helper for generating random content to our custom events
fn generate_random_content()->Vec<u8>{
	let mut rng = rand::thread_rng();
	format!("Content {}",rng.gen_range(0..10000)).as_bytes().to_vec()
}

//Helper fake block generating function
fn run_to_next_block() {
	System::set_block_number(System::block_number() + 1);
	EventStorageModule::offchain_worker(System::block_number());
}