#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{BoundedVec};

mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Bound of Products owned by an user
		type MaxProducts: Get<u32>;

		/// add product
		type AddOrigin: EnsureOrigin<Self::Origin, Success = <Self as frame_system::Config>::AccountId>;

		/// remove product
		type RemoveOrigin: EnsureOrigin<Self::Origin, Success = <Self as frame_system::Config>::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clc_price)]
	pub type CLCPrice<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn co2_eea_price)]
	pub type CO2EEAPrice<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn products)]
	pub(crate) type Products<T: Config> = StorageMap<_, Blake2_128Concat, ProductId, Option<Product>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn product_owner)]
	pub(crate) type ProductOwner<T: Config> = StorageMap<_, Blake2_128Concat, ProductId, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn product_owned)]
	pub(crate) type ProductOwned<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<ProductId, T::MaxProducts>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn co2_emissions)]
	pub(crate) type CO2Emissions<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, CO2Emission, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_co2_emissions)]
	pub(crate) type TotalCO2Emissions<T: Config> = StorageValue<_, CO2Emission, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pending_nft_minting)]
	pub(crate) type PendingNFTMinting<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<ProductId, T::MaxProducts>, ValueQuery>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An event triggered when a product is registered.
		ProductRegistered { product_id: ProductId , account_id: T::AccountId},

		/// An event triggered when a product is removed.
		ProductRemoved { product_id: ProductId , account_id: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already exist.
		AlreadyAdded,
		// The product does not exist.
		NotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Registers a product to the blockchain storage.
		#[pallet::weight(0)]
		pub fn register_product(origin: OriginFor<T>, product_id: ProductId, product: Product) -> DispatchResult {

			// Check if the account is alliance member
			let account_id = T::AddOrigin::ensure_origin(origin)?;

			match <Products<T>>::contains_key(product_id) {
				// Return an error if the id has already been used.
				true => {
					return Err(Error::<T>::AlreadyAdded.into());
				},

				false => {
					// Add new product to Products storage
					Products::<T>::insert(&product_id, Some(product));

					// Emit an event that new product was added.
					Self::deposit_event(Event::ProductRegistered { product_id , account_id });

					Ok(())
				}
			}
		}

		// Removes a product from the blockchain storage.
		#[pallet::weight(0)]
		pub fn remove_product(origin: OriginFor<T>, product_id: ProductId) -> DispatchResult {

			// Check if the account is alliance member
			let account_id = T::RemoveOrigin::ensure_origin(origin)?;

			match <Products<T>>::contains_key(product_id) {
				true => {
					// Remove a product from Products storage
					Products::<T>::remove(&product_id);

					// Emit an event that new product was added.
					Self::deposit_event(Event::ProductRemoved { product_id, account_id });
					Ok(())
				},
				// Return an error if the product does not exist.
				false => {
					return Err(Error::<T>::NotFound.into());
				}
			}
		}

	}
}

