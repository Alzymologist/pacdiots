#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{inherent::Vec, parameter_types, sp_runtime::RuntimeDebug, BoundedVec};
use scale_info::TypeInfo;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// Lock descriptor
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Tile <T: Config> {
    /// Lock's public key
    id: T::AccountId,
    /// Lock's owner
    owner: T::AccountId,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
	
        #[pallet::storage]
	#[pallet::getter(fn lock)]
	pub type LockID<T> = StorageMap<
            _,
            Blake2_128Concat,
            T::AccountId,
            T::AccountId,
            OptionQuery,
        >;

        #[pallet::storage]
	#[pallet::getter(fn access)]
	pub type LockAccess<T> = StorageDoubleMap<
            _,
            Blake2_128Concat,
            T::AccountId,
            Blake2_128Concat,
            T::AccountId,
            (),
            OptionQuery,
        >

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
                /// Lock already exists
                LockExists,
                /// Attempt to control a lock you do not own
                LockControlDenied,
                /// Lock does not exist in database
                LockUnknown,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn new_lock(origin: OriginFor<T>, lockId: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        ensure!(!LockId::<T>::contains_key(lockId), Error::<T>::LockExists);

			LockId::<T>::insert(lockId, who);

			Ok(())
		}

		/// Grant access
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn allow_access(origin: OriginFor<T>, lockId: T::AccountId, user: T::AccountId) -> DispatchResult {
			let _who = ensure_signed(origin)?;

                        let owner = lock(lockId);

                        if Some(a) = owner {
                            ensure!(a == who, LockControlDenied);
                        } else {
                            return Err(Error::<T>::LockUnknown);
                        }

                        LockAccess<T>::insert(lockId, user, ());
    		}

                /// Revoke access
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn allow_access(origin: OriginFor<T>, lockId: T::AccountId, user: T::AccountId) -> DispatchResult {
			let _who = ensure_signed(origin)?;

                        let owner = lock(lockId);

                        if Some(a) = owner {
                            ensure!(a == who, LockControlDenied);
                        } else {
                            return Err(Error::<T>::LockUnknown);
                        }

                        LockAccess<T>::remove(lockId, user, ());
    		}

	}
}
