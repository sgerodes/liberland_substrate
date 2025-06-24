// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::H160;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Account linked to EVM address
		AccountLinked { account_id: T::AccountId, evm_address: H160 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Account already linked
		AccountAlreadyLinked,
	}

	#[pallet::storage]
	pub type AccountIdToEvm<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, H160, OptionQuery>;

	#[pallet::storage]
	pub type EvmToAccountId<T: Config> =
		StorageMap<_, Twox64Concat, H160, T::AccountId, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn link_account(origin: OriginFor<T>, evm_address: H160) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure the account is not linked already
			ensure!(!AccountIdToEvm::<T>::contains_key(&who), Error::<T>::AccountAlreadyLinked);
			ensure!(
				!EvmToAccountId::<T>::contains_key(evm_address),
				Error::<T>::AccountAlreadyLinked
			);

			// Map account
			AccountIdToEvm::<T>::insert(&who, evm_address);
			EvmToAccountId::<T>::insert(evm_address, &who);

			// Emit event
			Self::deposit_event(Event::<T>::AccountLinked { account_id: who, evm_address });

			Ok(())
		}
	}
}
