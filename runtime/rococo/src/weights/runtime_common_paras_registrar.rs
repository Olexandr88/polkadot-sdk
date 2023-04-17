// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_common::paras_registrar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::paras_registrar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_common_paras_registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::paras_registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::paras_registrar::WeightInfo for WeightInfo<T> {
	/// Storage: Registrar NextFreeParaId (r:1 w:1)
	/// Proof Skipped: Registrar NextFreeParaId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `8706`
		// Minimum execution time: 27_294_000 picoseconds.
		Weight::from_parts(27_595_000, 0)
			.saturating_add(Weight::from_parts(0, 8706))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:0 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	/// Proof Skipped: Paras UpcomingParasGenesis (max_values: None, max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `572`
		//  Estimated: `29480`
		// Minimum execution time: 6_434_075_000 picoseconds.
		Weight::from_parts(6_507_768_000, 0)
			.saturating_add(Weight::from_parts(0, 29480))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:0 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	/// Proof Skipped: Paras UpcomingParasGenesis (max_values: None, max_size: None, mode: Measured)
	fn force_register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `489`
		//  Estimated: `28650`
		// Minimum execution time: 6_391_182_000 picoseconds.
		Weight::from_parts(6_485_890_000, 0)
			.saturating_add(Weight::from_parts(0, 28650))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeHash (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar PendingSwap (r:0 w:1)
	/// Proof Skipped: Registrar PendingSwap (max_values: None, max_size: None, mode: Measured)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `435`
		//  Estimated: `17955`
		// Minimum execution time: 41_497_000 picoseconds.
		Weight::from_parts(42_731_000, 0)
			.saturating_add(Weight::from_parts(0, 17955))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Registrar Paras (r:1 w:0)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:2 w:2)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar PendingSwap (r:1 w:1)
	/// Proof Skipped: Registrar PendingSwap (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Crowdloan Funds (r:2 w:2)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slots Leases (r:2 w:2)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `743`
		//  Estimated: `34901`
		// Minimum execution time: 48_650_000 picoseconds.
		Weight::from_parts(51_891_000, 0)
			.saturating_add(Weight::from_parts(0, 34901))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Paras FutureCodeHash (r:1 w:1)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeRestrictionSignal (r:1 w:1)
	/// Proof Skipped: Paras UpgradeRestrictionSignal (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeCooldowns (r:1 w:1)
	/// Proof Skipped: Paras UpgradeCooldowns (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingUpgrades (r:1 w:1)
	/// Proof Skipped: Paras UpcomingUpgrades (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 3145728]`.
	fn schedule_code_upgrade(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `330`
		//  Estimated: `30360`
		// Minimum execution time: 44_644_000 picoseconds.
		Weight::from_parts(44_838_000, 0)
			.saturating_add(Weight::from_parts(0, 30360))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(2_017, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1048576]`.
	fn set_current_head(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_589_000 picoseconds.
		Weight::from_parts(9_930_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(919, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}