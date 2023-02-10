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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
        use super::*;

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
	pub type LockId<T: Config> = StorageMap<
            _,
            Blake2_128Concat,
            T::AccountId,
            T::AccountId,
            OptionQuery,
        >;

        #[pallet::storage]
	#[pallet::getter(fn access)]
	pub type LockAccess<T: Config> = StorageDoubleMap<
            _,
            Blake2_128Concat,
            T::AccountId,
            Blake2_128Concat,
            T::AccountId,
            (),
            OptionQuery,
        >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
                /// New lock registration. [lock public key, controller account id]
                LockRegistered { lock: T::AccountId, owner: T::AccountId },
                /// User got access to lock. [lock public key, user account id]
                AccessGranted { lock: T::AccountId, user: T::AccountId },
                /// User lost access to lock. [lock public key, user account id]
                AccessRevoked { lock: T::AccountId, user: T::AccountId },
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
		pub fn new_lock(origin: OriginFor<T>, lock_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        ensure!(!LockId::<T>::contains_key(&lock_id), Error::<T>::LockExists);

			LockId::<T>::insert(&lock_id, &who);

                        Self::deposit_event(Event::LockRegistered{ lock: lock_id, owner: who });

			Ok(())
		}

		/// Grant access
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn grant_access(origin: OriginFor<T>, lock_id: T::AccountId, user: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        let owner = Self::lock(&lock_id);

                        if let Some(a) = owner {
                            ensure!(a == who, Error::<T>::LockControlDenied);
                        } else {
                            return Err(Error::<T>::LockUnknown.into());
                        }

                        LockAccess::<T>::insert(lock_id, user, ());

                        Ok(())
    		}

                /// Revoke access
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn revoke_access(origin: OriginFor<T>, lock_id: T::AccountId, user: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

                        let owner = Self::lock(&lock_id);

                        if let Some(a) = owner {
                            ensure!(a == who, Error::<T>::LockControlDenied);
                        } else {
                            return Err(Error::<T>::LockUnknown.into());
                        }

                        LockAccess::<T>::remove(lock_id, user);

                        Ok(())
    		}

	}
}
