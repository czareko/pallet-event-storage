#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use chrono::{Duration, Utc};
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Hash};
	use frame_system::pallet_prelude::*;
//	use sp_runtime::print;
//	use sp_runtime::runtime_logger::RuntimeLogger;


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

/*
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn check_custom_struct(origin: OriginFor<T>) -> DispatchResult {

			let message = "event message".to_string();

			//let bt = Bytes::from(vec![1,2]);

			//let event = CustomEvent{value: bt, time_stamp: Utc::now()};

			//println!("CustomEvent (value length): {}",event.value.to_vec().len());
			//println!("CustomEvent (time): {}",event.time_stamp);

			// Emit an event.
			let who = ensure_signed(origin)?;
			let something = 42;
			<Something<T>>::put(something);
			//<Something<T>>::
			Self::deposit_event(Event::EventStored(something, who));
			Ok(())
		}*/
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(1000 + T::DbWeight::get().writes(1))]
		pub fn create_custom_event(origin: OriginFor<T>,message: String) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let content_value = message;
			let tstamp = Utc::now().timestamp_nanos();
			//let tstamp_key = T::Hashing::hash_of(&tstamp);
			let custom_event = CustomEvent { time_stamp: tstamp, content: content_value.clone(), reporter: who.clone()};

			<CustomEvents<T>>::insert(tstamp,custom_event);

			Self::deposit_event(Event::EventStored(tstamp,who.clone()));

			// print("ABC:");
			// debug::RuntimeLogger::init();
			// debug::info!("DEBUG INFO");

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn remove_historical_events(origin: OriginFor<T>)->DispatchResult{
			//who will be the caller here when we run it as a automated background task?
			let who = ensure_signed(origin)?;
			let mut to_remove =vec![];
			let mut removed_elems=0;

			//let storage_snapshot = <CustomEvents<T>>::
			/*
			If the storage will be altered during the iteration we can have undefined results
			To limit this problem, we split this process into two parts.
			*/
			let keys = <CustomEvents<T>>::iter_keys();
			for key in keys{
				let tnow = (Utc::now()-Duration::seconds(1)).timestamp_nanos();
				//let value = <CustomEvents<T>>::get(key).unwrap();
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

			Self::deposit_event(Event::HistoricalEventsRemoved(removed_elems,who.clone()));
			Ok(())
		}
	}
}

