// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_im_online`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_im_online
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_im_online`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_im_online::WeightInfo for WeightInfo<T> {
	// Storage: Session Validators (r:1 w:0)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: ImOnline ReceivedHeartbeats (r:1 w:1)
	// Storage: ImOnline AuthoredBlocks (r:1 w:0)
	// Storage: ImOnline Keys (r:1 w:0)
	/// The range of component `k` is `[1, 1000]`.
	/// The range of component `e` is `[1, 100]`.
	fn validate_unsigned_and_then_heartbeat(k: u32, e: u32, ) -> Weight {
		// Minimum execution time: 97_738 nanoseconds.
		Weight::from_ref_time(77_543_087)
			// Standard Error: 395
			.saturating_add(Weight::from_ref_time(25_104).saturating_mul(k.into()))
			// Standard Error: 3_987
			.saturating_add(Weight::from_ref_time(318_463).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
