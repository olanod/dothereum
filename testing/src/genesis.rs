// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// Copyright 2019-2020 Dothereum UG (DE).
// This file is part of Dothereum.

// Dothereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Dothereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Dothereum.  If not, see <http://www.gnu.org/licenses/>.

//! Genesis Configuration.

use crate::keyring::*;
use keyring::{Ed25519Keyring, Sr25519Keyring};
use dothereum_runtime::{
	GenesisConfig, BalancesConfig, SessionConfig, AuraConfig, SystemConfig,
	GrandpaConfig, IndicesConfig, ContractsConfig, WASM_BINARY,
};
use dothereum_runtime::constants::currency::*;
use primitives::ChangesTrieConfiguration;
use sr_primitives::Perbill;


/// Create genesis runtime configuration for tests.
pub fn config(support_changes_trie: bool, code: Option<&[u8]>) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			changes_trie_config: if support_changes_trie { Some(ChangesTrieConfiguration {
				digest_interval: 2,
				digest_levels: 2,
			}) } else { None },
			code: code.map(|x| x.to_vec()).unwrap_or_else(|| WASM_BINARY.to_vec()),
		}),
		indices: Some(IndicesConfig {
			ids: vec![alice(), bob(), charlie(), dave(), eve(), ferdie()],
		}),
		balances: Some(BalancesConfig {
			balances: vec![
				(alice(), 111 * DOLLARS),
				(bob(), 100 * DOLLARS),
				(charlie(), 100_000_000 * DOLLARS),
				(dave(), 111 * DOLLARS),
				(eve(), 101 * DOLLARS),
				(ferdie(), 100 * DOLLARS),
			],
			vesting: vec![],
		}),
		session: Some(SessionConfig {
			keys: vec![
				(alice(), to_session_keys(
					&Ed25519Keyring::Alice,
					&Sr25519Keyring::Alice,
				)),
				(bob(), to_session_keys(
					&Ed25519Keyring::Bob,
					&Sr25519Keyring::Bob,
				)),
				(charlie(), to_session_keys(
					&Ed25519Keyring::Charlie,
					&Sr25519Keyring::Charlie,
				)),
			]
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(Default::default()),
	}
}
