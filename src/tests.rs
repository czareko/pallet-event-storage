use chrono::Utc;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{OnInitialize,OnFinalize,OffchainWorker};

//use sp_core::offchain::{testing, OffchainWorkerExt, TransactionPoolExt};

//use std::sync::Arc;

//use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};


// use sp_runtime::{
// 	testing::{Header, TestXt},
// 	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
// 	RuntimeAppPublic,
// };

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
fn check_create_custom_event(){
	new_test_ext().execute_with(|| {
			//let tstamp = Utc::now().timestamp_millis();
		run_to_block(10);
		println!("Check custom struct {}",Utc::now().timestamp_nanos());
		for i in 0..25{
			let st = format!("ABC: {}",i);
			assert_ok!(EventStorageModule::create_custom_event(Origin::signed(1),st));
			//println!("ABC {}",i);
		}
		//assert_ok!(EventStorageModule::remove_historical_events(Origin::signed(1)));
		println!("HISTORY REMOVE");
		EventStorageModule::offchain_worker(System::block_number());
		//assert_eq!(TemplateModule::check_custom_struct(Origin::signed(1)),42);
	});
}

#[test]
fn check_offchain_worker(){
	/*
	new_test_ext().execute_with(|| {
		let (offchain, state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();

		let keystore = KeyStore::new();

		let mut t = sp_io::TestExternalities::default();
		t.register_extension(OffchainWorkerExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(Arc::new(keystore)));


		price_oracle_response(&mut state.write());

		println!("Offchain test method");

		t.execute_with(|| {

			let tx = pool_state.write().transactions.pop().unwrap();
			assert!(pool_state.read().transactions.is_empty());
			//let tx = Extrinsic::decode(&mut &*tx).unwrap();
			//assert_eq!(tx.signature, None);
			// when
			let price = 15523;
			// let price = Example::fetch_price().unwrap();
			// then
			assert_eq!(price, 15523);
		});
	});
	*/
}
/*
#[test]
fn check_storage_size(){
	let b_number = System::block_number();
}
//This is completely copy paste method, just to simulate test transaction
fn price_oracle_response(state: &mut testing::OffchainState) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD".into(),
		response: Some(br#"{"USD":20385.4}"#.to_vec()),
		sent: true,
		..Default::default()
	});
}
*/

//#[test]
fn check_storage_size(){
	new_test_ext().execute_with(|| {
		run_to_block(50);
		assert_ok!(EventStorageModule::check_public(Origin::signed(1)));
		println!("BNUMBER: {}",System::block_number());
	});

//	let b_number = System::block_number();
//	println!("ABC: {}",b_number);
}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			//EventStorageModule::on_finalize(System::block_number());
			EventStorageModule::offchain_worker(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		//EventStorageModule::on_initialize(System::block_number());
	}
}