use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};

use node_template_runtime::{
	AccountId, AuthorityDiscoveryConfig ,BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, SessionConfig, StakingConfig, opaque::SessionKeys,
	StakerStatus, Balance, WASM_BINARY, Signature,currency::DOLLARS
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use pallet_staking::Forcing;
use hex_literal::hex;


use sc_service::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;

// The URL for the telemetry server.
// const STAGING_TELEMETRzY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId,AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			vec![
				authority_keys_from_seed("Alice")
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}


pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}



///ttc
pub fn ttc_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;


	Ok(ChainSpec::from_genesis(
		// Name
		"ttc Testnet",
		// ID
		"ttc_testnet",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![(
				hex!["63a3dfbe0e5e7c8bae53569cb9433928f5813b27319aa2a71dd093c1ac9e0666"].into(),
				hex!["63a3dfbe0e5e7c8bae53569cb9433928f5813b27319aa2a71dd093c1ac9e0666"].into(),
				hex!["e6d8f9b41bc64362176fae74b510ff16de998a252a311f12f7d4f63c2c1b3f05"].unchecked_into(),
				hex!["a6c275817f9960e3d7f67ccdf7468713e1b6b8e2d6c55b3ddc4ee84316718049"].unchecked_into(),
				hex!["63a3dfbe0e5e7c8bae53569cb9433928f5813b27319aa2a71dd093c1ac9e0666"].unchecked_into(),
				 ),(
				// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
				hex!["5831843b9bc00cc54cd8485ee6487ea7aae3674e335335da83e3884c0ca1a9c0"].into(),
				// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
				hex!["5831843b9bc00cc54cd8485ee6487ea7aae3674e335335da83e3884c0ca1a9c0"].into(),
				// 5CQ7gVQj96m8y79qPCqrM291rSNREfZ1Tf2fiLPSJReWTNy2
				hex!["9e19d291982a538eb67521f809dffeb7695d1791f49fac4e95f1d1bafe67014f"].unchecked_into(),
				// 5FyNaMc6GaioN7K9QzPJDEtGThJ1HmcruRdgtiRxaoAwn2VD
				hex!["ee3143dd33ec90c17253f9f2eda9af74b7e11759a6951083d1fff9351054b676"].unchecked_into(),
				hex!["63a3dfbe0e5e7c8bae53569cb9433928f5813b27319aa2a71dd093c1ac9e0666"].unchecked_into(),

			),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
						 get_account_id_from_seed::<sr25519::Public>("Alice"),
						 get_account_id_from_seed::<sr25519::Public>("Bob"),
						 get_account_id_from_seed::<sr25519::Public>("Charlie"),
						 get_account_id_from_seed::<sr25519::Public>("Dave"),
						 get_account_id_from_seed::<sr25519::Public>("Eve"),
						 get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						 get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						 get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						 get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						 get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						 get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						 get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}



fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe , authority_discovery}
}


/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId,AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const STASH: Balance = 100 * DOLLARS;
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(node_template_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},

		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		},
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },

	}
}





