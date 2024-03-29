//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
	use node_primitives::Balance;

	pub const PICO_SHEAR: Balance = 1;
	pub const NANO_SHEAR: Balance = 1_000 * PICO_SHEAR;
	pub const MICRO_SHEAR: Balance = 1_000 * NANO_SHEAR;
	pub const MILLI_SHEAR: Balance = 1_000 * MICRO_SHEAR; // assume this is worth about a cent.
	pub const SHEAR: Balance = 1_000 * MILLI_SHEAR;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * MILLI_SHEAR + (bytes as Balance) * 6 * MILLI_SHEAR
	}
}

pub mod time {
	use node_primitives::{BlockNumber, Moment};
	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_babe` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: Moment = 5000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	// TODO: Update this to 4 Hours for prod
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * HOURS;
	pub const EPOCH_DURATION_IN_SLOTS: u64 = {
		const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

		(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
	};
}
