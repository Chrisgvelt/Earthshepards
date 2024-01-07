use crate::*;

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * SHEAR;
	pub const ItemDeposit: Balance = 1 * SHEAR;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * SHEAR;
	pub const MetadataDepositPerByte: Balance = 1 * SHEAR;
}

impl pallet_uniques::Config for Runtime {
	type Event = Event;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = pallet_uniques::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}
