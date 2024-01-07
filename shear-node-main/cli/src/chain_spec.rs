use hex_literal::hex;
use node_primitives::*;
use shear_runtime::{
	SessionKeys, BabeConfig, BalancesConfig, CouncilConfig, AuthorityDiscoveryConfig, TreasuryConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, MaxNominations, SudoConfig, IndicesConfig,// ElectionsConfig, 
	SessionConfig, StakerStatus, StakingConfig, SystemConfig, TechnicalCommitteeConfig,
	BABE_GENESIS_EPOCH_CONFIG, wasm_binary_unwrap,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use grandpa_primitives::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use sc_service::{Properties}; //import Properties
use sc_telemetry::serde_json::Map;
use jsonrpc_core::Value;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;


fn session_keys(
	babe: BabeId, 
	grandpa: GrandpaId, 
	im_online: ImOnlineId, 
	authority_discovery: AuthorityDiscoveryId
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		// get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

/// Shear Native Token Properties
pub fn shear_properties() -> Map<String, Value> {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "SHE".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());
	properties
}

/// Development config (change these values to test different settings)
pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(shear_properties()),
		// Extensions
		None,
	))
}

/// Local testnet config
pub fn local_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Shear")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Shear"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Shear"),
					get_account_id_from_seed::<sr25519::Public>("Ciprian"),
					get_account_id_from_seed::<sr25519::Public>("Fenn"),
					get_account_id_from_seed::<sr25519::Public>("Artim"),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(shear_properties()),
		// Extensions
		None,
	))
}

/// Staging config
pub fn staging_network_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![];

	Ok(ChainSpec::from_genesis(
		"Shear Staging",
		"shear_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		None,
		None,
		None,
		// Properties
		Some(shear_properties()),
		Default::default(),
	))
}

/// Staging config
fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)> = vec![
		(
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["32f81b7c663680ce823bbae6245ee0eb542f4690c058f9adf5e02847d3548774"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["32f81b7c663680ce823bbae6245ee0eb542f4690c058f9adf5e02847d3548774"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["32f81b7c663680ce823bbae6245ee0eb542f4690c058f9adf5e02847d3548774"]
				.unchecked_into(),
			// 5DV6nkDt73cA8qcKbPbxCcpzHi3w57krqB9Do9oRTsFcXUAt
			hex!["45f85a7a86b44daeb687e8d9b7ae021b42caf1df74d141c902c6b9b47a21e3cb"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["32f81b7c663680ce823bbae6245ee0eb542f4690c058f9adf5e02847d3548774"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["32f81b7c663680ce823bbae6245ee0eb542f4690c058f9adf5e02847d3548774"]
				.unchecked_into(),
		),
		(
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"]
				.unchecked_into(),
			// 5DV6nkDt73cA8qcKbPbxCcpzHi3w57krqB9Do9oRTsFcXUAt
			hex!["5af9d534dc1b53d86c31a5980870c80e7ab72f06665f9b0a68f5df94b81e9c75"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"]
				.unchecked_into(),
		),
		(
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["1660eef409945d2ba320ece90fc4cd2a5d975868b8c37a88681ffd24d3596956"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["1660eef409945d2ba320ece90fc4cd2a5d975868b8c37a88681ffd24d3596956"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["1660eef409945d2ba320ece90fc4cd2a5d975868b8c37a88681ffd24d3596956"]
				.unchecked_into(),
			// 5DV6nkDt73cA8qcKbPbxCcpzHi3w57krqB9Do9oRTsFcXUAt
			hex!["b621452e2ae34791b4373a63b756e4fd71d418cacaaf38e075f8ded4821ad1c8"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["1660eef409945d2ba320ece90fc4cd2a5d975868b8c37a88681ffd24d3596956"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["1660eef409945d2ba320ece90fc4cd2a5d975868b8c37a88681ffd24d3596956"]
				.unchecked_into(),
		),
		(
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["22408bf02a753c787c3e0484bd6596e55c999ed851884d1e5a138a9c7334f41f"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["22408bf02a753c787c3e0484bd6596e55c999ed851884d1e5a138a9c7334f41f"].into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["22408bf02a753c787c3e0484bd6596e55c999ed851884d1e5a138a9c7334f41f"]
				.unchecked_into(),
			// 5DV6nkDt73cA8qcKbPbxCcpzHi3w57krqB9Do9oRTsFcXUAt
			hex!["b86be97d8f1f75d83cd52c22bb77274a43d6782822d7b61b8bb6d866a344d5fa"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["22408bf02a753c787c3e0484bd6596e55c999ed851884d1e5a138a9c7334f41f"]
				.unchecked_into(),
			// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
			hex!["22408bf02a753c787c3e0484bd6596e55c999ed851884d1e5a138a9c7334f41f"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5GeebE2d2bXU1j7va84dXtwa9L4uzKZY2SJrTz7ofvDHHxuG
		"5cb70db349acee116069f4d61ff9729783f40ad9ad6222931d29975b2d95ae07"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts
	)
}


/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	const STASH: Balance = 1_000_000__000_000_000_000; // 1M

	let num_endowed_accounts = endowed_accounts.len();
	let initial_supply: Balance  = 1_000_000_000__000_000_000_000; // 1B
	let treasury_balance: Balance = 100_000_000__000_000_000_000; // 100M
	let user_balance: Balance = initial_supply - treasury_balance; // 900M
	let each_user_balance: Balance = user_balance / num_endowed_accounts as Balance;

	let min_nominator_bond = 1_000__000_000_000_000; // 1K
	let min_validator_bond = 1_000_000__000_000_000_000; // 1M


	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, each_user_balance)).collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			min_nominator_bond,
			min_validator_bond,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		alliance_membership: Default::default(),
		treasury: TreasuryConfig {
			treasury_balance,
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
