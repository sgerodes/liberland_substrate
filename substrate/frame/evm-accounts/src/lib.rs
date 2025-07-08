// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::PalletId;
use frame_system::pallet_prelude::*;
use pallet_evm::AddressMapping;
use sp_core::H160;
use sp_runtime::traits::AccountIdConversion;
use sp_std::marker::PhantomData;
use sp_std::prelude::*;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

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

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub linked_accounts: Vec<(T::AccountId, H160)>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { linked_accounts: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for (account, evm_account) in &self.linked_accounts {
				AccountIdToEvm::<T>::insert(account, evm_account);
				EvmToAccountId::<T>::insert(evm_account, account);
			}
		}
	}

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

pub struct EvmAccountMapping<T>(PhantomData<T>);

impl<T: Config> AddressMapping<T::AccountId> for EvmAccountMapping<T> {
	fn into_account_id(address: sp_core::H160) -> T::AccountId {
		EvmToAccountId::<T>::get(&address)
			// TODO: Replace with unique address mapping
			.unwrap_or(PalletId(*b"evmaccou").into_account_truncating())
	}
}
