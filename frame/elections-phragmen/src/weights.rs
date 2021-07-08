// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_elections_phragmen
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.1
//! DATE: 2021-01-20, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/elections-phragmen/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections_phragmen.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn remove_member_wrong_refund() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections_phragmen using the Substrate substrate and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn vote_equal(v: u32, ) -> Weight {
		(45_157_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((399_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vote_more(v: u32, ) -> Weight {
		(69_738_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((450_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn vote_less(v: u32, ) -> Weight {
		(73_955_000 as Weight)
			// Standard Error: 38_000
			.saturating_add((227_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_voter() -> Weight {
		(68_398_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn submit_candidacy(c: u32, ) -> Weight {
		(59_291_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((412_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(55_026_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((207_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_members() -> Weight {
		(77_840_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn renounce_candidacy_runners_up() -> Weight {
		(54_559_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_member_with_replacement() -> Weight {
		(84_311_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn remove_member_wrong_refund() -> Weight {
		(7_677_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 55_000
			.saturating_add((114_815_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 53_000
			.saturating_add((49_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_940_000
			.saturating_add((43_557_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 807_000
			.saturating_add((65_849_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 55_000
			.saturating_add((4_206_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn vote_equal(v: u32, ) -> Weight {
		(45_157_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((399_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn vote_more(v: u32, ) -> Weight {
		(69_738_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((450_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn vote_less(v: u32, ) -> Weight {
		(73_955_000 as Weight)
			// Standard Error: 38_000
			.saturating_add((227_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn remove_voter() -> Weight {
		(68_398_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn submit_candidacy(c: u32, ) -> Weight {
		(59_291_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((412_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		(55_026_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((207_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn renounce_candidacy_members() -> Weight {
		(77_840_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn renounce_candidacy_runners_up() -> Weight {
		(54_559_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_member_with_replacement() -> Weight {
		(84_311_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn remove_member_wrong_refund() -> Weight {
		(7_677_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 55_000
			.saturating_add((114_815_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 53_000
			.saturating_add((49_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_940_000
			.saturating_add((43_557_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 807_000
			.saturating_add((65_849_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 55_000
			.saturating_add((4_206_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}
