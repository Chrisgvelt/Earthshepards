use crate::*;

use pallet_collective::EnsureMember;
use pallet_collective::Instance2;

impl pallet_marketplace::Config for Runtime {
    type Event = Event;
    type MaxProducts = ConstU32<20>;
	type AddOrigin = EnsureMember<AccountId, Instance2>;
	type RemoveOrigin = EnsureMember<AccountId, Instance2>;
}