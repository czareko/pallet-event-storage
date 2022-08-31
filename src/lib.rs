#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	self as system,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use chrono::{Duration, Utc};
	use frame_support::log;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use crate::system;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct CustomEvent<T: Config> {
		pub time_stamp: i64,
		pub content: String,
		pub reporter: <T as frame_system::Config>::AccountId,
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn on_finalize(n: T::BlockNumber) {
			println!("On Finalize Event ----------");

		}

		/// Offchain Worker entry point.
		///
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("Hello World from offchain workers!");
			let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
			let res = Self::remove_history();
			println!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);
			log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn custom_events)]
	pub(super) type CustomEvents<T: Config> = StorageMap<_, Twox64Concat, i64, CustomEvent<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		EventStored(i64,T::AccountId),
		HistoricalEventsRemoved(i32,T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		EventNotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1000 + T::DbWeight::get().writes(1))]
		pub fn create_custom_event(origin: OriginFor<T>,message: String) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let content_value = message;
			let tstamp = Utc::now().timestamp_nanos();
			let custom_event = CustomEvent { time_stamp: tstamp, content: content_value.clone(), reporter: who.clone()};

			<CustomEvents<T>>::insert(tstamp,custom_event);

			Self::deposit_event(Event::EventStored(tstamp,who.clone()));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn remove_historical_events(origin: OriginFor<T>)->DispatchResult{
			//who will be the caller here when we run it as a automated background task?
			let who = ensure_signed(origin)?;
			let mut to_remove =vec![];
			let mut removed_elems=0;

			/*
			If the storage will be altered during the iteration we can have undefined results
			To limit this problem, we split this process into two parts.
			*/
			let keys = <CustomEvents<T>>::iter_keys();
			for key in keys{
				let tnow = (Utc::now()-Duration::seconds(1)).timestamp_nanos();
				if key<tnow {
					to_remove.push(key);
				}
				else{
					println!("Else");
				}
			}
			println!("To remove: {}",to_remove.len());
			for key in to_remove{
				println!("Removing: {}",key);
				<CustomEvents<T>>::remove(key);
				removed_elems+=1;
			}

			//Self::storage_size();
			Self::deposit_event(Event::HistoricalEventsRemoved(removed_elems,who.clone()));
			Ok(())
		}

		#[pallet::weight(1000)]
		pub fn check_public(origin: OriginFor<T>)->DispatchResult{
			println!("Check_Public IN");
			Ok(())
		}
	}
	impl<T: Config> Pallet<T> {

		fn storage_size()->Option<i32>{
			let events = <CustomEvents<T>>::iter_keys();
			let ev_size = events.count() as i32;
			if ev_size == 0 {
				None
			}
			else{
				println!("Storage size: {}",ev_size);
				Some(ev_size)
			}
		}

		fn remove_history()->Option<i32>{
			println!("PRV:");
			let mut to_remove =vec![];
			let mut removed_elems=0;

			/*
			If the storage will be altered during the iteration we can have undefined results
			To limit this problem, we split this process into two parts.
			*/
			let keys = <CustomEvents<T>>::iter_keys();
			for key in keys{
				let tnow = (Utc::now()-Duration::seconds(1)).timestamp_nanos();
				if key<tnow {
					to_remove.push(key);
				}
				else{
					println!("Else");
				}
			}
			println!("To remove: {}",to_remove.len());
			for key in to_remove{
				println!("Removing: {}",key);
				<CustomEvents<T>>::remove(key);
				removed_elems+=1;
			}
			Some(removed_elems)
		}
	}
}

