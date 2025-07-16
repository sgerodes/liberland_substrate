use super::*;
use frame_support::traits::OnRuntimeUpgrade;

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

type DbWeight = <Runtime as frame_system::Config>::DbWeight;

pub mod initialize_evm_chainid {
	use super::*;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let weight = DbWeight::get().writes(1);

			// TODO: Update once official IDs are obtained

			// EVM ChainId - mainnet
			#[cfg(not(feature = "testnet-runtime"))]
			pallet_evm_chain_id::ChainId::<Runtime>::put(1234u64);

			// EVM ChainId - testnet
			#[cfg(feature = "testnet-runtime")]
			pallet_evm_chain_id::ChainId::<Runtime>::put(5678u64);

			weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}
