use crate as pallet_marketplace;
use frame_support::{
	parameter_types, ord_parameter_types,
	traits::{ConstU16, ConstU32, ConstU64}
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
pub use node_primitives::AccountId;
// use crate::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances,
		AllianceCouncil: pallet_collective,
		AllianceMembership: pallet_membership,
		Marketplace: pallet_marketplace,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

type Balance = u64;

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const CouncilMotionDuration: u64 = 3;
	pub const CouncilMaxProposals: u32 = 100;
	pub const MaxMembers: u32 = 100;
}

impl pallet_collective::Config for Test {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = MaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

impl pallet_membership::Config for Test {
	type Event = Event;
	type AddOrigin = system::EnsureRoot<u64>;
	type RemoveOrigin = system::EnsureRoot<u64>;
	type SwapOrigin = system::EnsureRoot<u64>;
	type ResetOrigin = system::EnsureRoot<u64>;
	type PrimeOrigin = system::EnsureRoot<u64>;
	type MembershipInitialized = AllianceCouncil;
	type MembershipChanged = AllianceCouncil;
	type MaxMembers = MaxMembers;
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl pallet_marketplace::Config for Test {
	type Event = Event;
	type MaxProducts = ConstU32<20>;
	type AddOrigin = system::EnsureSignedBy<One, u64>;
	type RemoveOrigin = system::EnsureSignedBy<One, u64>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = GenesisConfig {
		system: frame_system::GenesisConfig::default(),
		balances: pallet_balances::GenesisConfig { balances: vec![(1, 1000), (2, 1000), (3, 1000)] },
		alliance_membership: Default::default(),
		alliance_council: pallet_collective::GenesisConfig {
			members: vec![1],
			phantom: Default::default(),
		}
	}
	.build_storage()
	.unwrap()
	.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
