use chrono::Utc;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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

#[test]
fn check_create_custom_event(){
	new_test_ext().execute_with(|| {
			//let tstamp = Utc::now().timestamp_millis();
		println!("Check custom struct {}",Utc::now().timestamp_nanos());
		for i in 0..25{
			let st = format!("ABC: {}",i);
			//assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),st));
			println!("ABC {}",i);
		}
		assert_ok!(EventStorageModule::remove_historical_events(Origin::signed(1)));
		//assert_eq!(TemplateModule::check_custom_struct(Origin::signed(1)),42);
	});
}