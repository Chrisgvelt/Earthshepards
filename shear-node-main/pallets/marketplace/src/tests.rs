use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err, BoundedVec};
use sp_core::H256;

use super::*;

#[test]
/*
 * Check if alliance member can register a product.
 */
fn register_product_with_alliance_member() {
	new_test_ext().execute_with(|| {
		let product: Product = Product {
			co2_savings: 10,
			invoice_id: 1,
			product_type: BoundedVec::default(),
			price: 100,
			hash: H256::default(),
		};
		assert_ok!(Marketplace::register_product(Origin::signed(1), 1, product.clone()));
		assert_eq!(Marketplace::products(1), Some(product));
	});
}

#[test]
/*
 * Check if non alliance member can register a product.
 */
fn register_product_with_non_alliance_member() {
	new_test_ext().execute_with(|| {
		let product: Product = Product {
			co2_savings: 10,
			invoice_id: 1,
			product_type: BoundedVec::default(),
			price: 100,
			hash: H256::default(),
		};
		// Non alliance member 2 can't register product.
		assert_err!(Marketplace::register_product(Origin::signed(2), 1, product.clone()), sp_runtime::traits::BadOrigin);
		// Unsigned member can't register product.
		assert_err!(Marketplace::register_product(Origin::none(), 1, product.clone()), sp_runtime::traits::BadOrigin);
	});
}

#[test]
/*
 * Anyone(of course alliance member) can override new product to existing product id.
 */
fn register_to_existing_product_id() {
	new_test_ext().execute_with(|| {
		let product: Product = Product {
			co2_savings: 10,
			invoice_id: 1,
			product_type: BoundedVec::default(),
			price: 100,
			hash: H256::default(),
		};

		let product2: Product = Product {
			co2_savings: 11,
			invoice_id: 2,
			product_type: BoundedVec::default(),
			price: 101,
			hash: H256::default(),
		};

		// Register "product" to id 1.
		assert_ok!(Marketplace::register_product(Origin::signed(1), 1, product.clone()));
		// If someone tries to register to id 1 again, it will cause error "AlreadyAdded".
		assert_err!(Marketplace::register_product(Origin::signed(1), 1, product2.clone()), Error::<Test>::AlreadyAdded);
	});
}

#[test]
/*
 *	Try remove non existing product id
 */
fn remove_non_existing_product_id() {
	new_test_ext().execute_with(|| {
		// Not any product registered.
		// Try remove id 1.
		assert_err!(Marketplace::remove_product(Origin::signed(1), 1), Error::<Test>::NotFound);
		// Removing non existing product id causes an error.
	});
}

#[test]
/*
 *	Check if creator can remove product.
 */
fn remove_product_with_creator() {
	new_test_ext().execute_with(|| {
		let product: Product = Product {
			co2_savings: 10,
			invoice_id: 1,
			product_type: BoundedVec::default(),
			price: 100,
			hash: H256::default(),
		};
		// Register product 1 with member 1.
		assert_ok!(Marketplace::register_product(Origin::signed(1), 1, product.clone()));
		// Member 1 can remove product 1.
		assert_ok!(Marketplace::remove_product(Origin::signed(1), 1));
		// Product should not be in Store
		assert_eq!(Marketplace::products(1), None);
	});
}
