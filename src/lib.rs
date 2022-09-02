#![cfg_attr(not(feature = "std"), no_std)]
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
	use frame_support::pallet_prelude::*;
	use frame_support::storage::KeyPrefixIterator;
	use frame_system::pallet_prelude::*;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct CustomEvent<T: Config> {
		// This is a very tech example, to make it more useful from business perspective this should be the first place to expand.
		//pub time_stamp: i64, <<--- we have this as a key, but probably for more complex business operation we can need this inside as well.
		pub content: Vec<u8>,
		pub reporter: <T as frame_system::Config>::AccountId, //we limited access to our method so we could remove this from here as well.
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/// Offchain Worker entry point.
		///
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			let res = Self::remove_history();
			Self::deposit_event(Event::HistoricalEventsRemoved(res.unwrap(),block_number));
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		//History size expressed in hours
		#[pallet::constant]
		type HistorySize: Get<i64>;

		//Account authorized to execute public pallet method
		#[pallet::constant]
		type AuthorizedAccountId: Get<<Self as frame_system::Config>::AccountId>;
	}

	//i64 is expensive as a key, we should think about something smaller
	#[pallet::storage]
	#[pallet::getter(fn custom_events)]
	pub(super) type CustomEvents<T: Config> = StorageMap<_, Twox64Concat, i64, CustomEvent<T>,OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		EventStored(i64,T::AccountId), //It will be always the same account. Maybe we don't need it here?
		HistoricalEventsRemoved(i32,T::BlockNumber),
	}

	// Should be expanded with more detailed behaviours tracking.
	#[pallet::error]
	pub enum Error<T> {
		StorageStatusException,
		EventNotFound,
		UnauthorizedCaller,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(1000 + T::DbWeight::get().writes(1))]
		pub fn create_custom_event(origin: OriginFor<T>,content_value: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == T::AuthorizedAccountId::get(), Error::<T>::UnauthorizedCaller);
			let time_stamp = Utc::now().timestamp_nanos();
			let custom_event = CustomEvent { /*time_stamp: tstamp, */content: content_value.clone(), reporter: who.clone()};

			<CustomEvents<T>>::insert(time_stamp,custom_event);

			Self::deposit_event(Event::EventStored(time_stamp,who.clone()));
			Ok(())
		}
	}
	impl<T: Config> Pallet<T> {

		//We don't want to expose this method outside
		fn remove_history()->Option<i32>{
			let mut to_remove =vec![];
			let mut removed_elems=0;

			/*
			If the storage will be altered during the iteration we can have undefined results
			To limit this problem, we split this process into two parts.
			*/
			let keys = <CustomEvents<T>>::iter_keys();
			for key in keys{
				let tnow = (Utc::now()-Duration::seconds(T::HistorySize::get())).timestamp_nanos();
				if key<tnow {
					to_remove.push(key);
				}
			}
			for key in to_remove{
				<CustomEvents<T>>::remove(key);
				removed_elems+=1;
			}
			Some(removed_elems)
		}

		//Methods for test storage behaviors
		pub fn get_storage_size()->Option<i32>{
			let events = <CustomEvents<T>>::iter_keys();
			let ev_size = events.count() as i32;
			Some(ev_size)
		}

		pub fn get_custom_event_keys()->Option<KeyPrefixIterator<i64>>{
			Some(<CustomEvents<T>>::iter_keys())
		}
		pub fn get_custom_event(key: i64)->Option<CustomEvent<T>>{
			let ans = <CustomEvents<T>>::get(key);
			Some(ans.unwrap())
		}
	}
}

