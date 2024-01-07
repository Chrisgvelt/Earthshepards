use codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::{RuntimeDebug};
use sp_core::{H256};
use node_primitives::{Balance};
use scale_info::TypeInfo;
use frame_support::{BoundedVec, traits::ConstU32};

pub type ProductId = u32;
pub type ProductPrice = Balance;
pub type InvoiceId = u32;
pub type CO2Emission = u32;

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Product {
    pub co2_savings: CO2Emission,
	pub invoice_id: InvoiceId,
	pub product_type: BoundedVec<u8, ConstU32<8>>,
	pub price: ProductPrice,
	pub hash: H256,
}