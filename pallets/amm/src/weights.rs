// This file is part of HydraDX-node.

// Copyright (C) 2021 Intergalactic Ltd.
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

//! Autogenerated weights for amm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2021-03-03, STEPS: [5, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/hydra-dx
// benchmark
// --chain=dev
// --steps=5
// --repeat=20
// --pallet=amm
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=weights.rs
// --template=.maintain/pallet-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for amm.
pub trait WeightInfo {
fn create_pool() -> Weight;
fn add_liquidity() -> Weight;
fn remove_liquidity() -> Weight;
fn sell() -> Weight;
fn buy() -> Weight;
}

/// Weights for amm using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
		impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
				fn create_pool() -> Weight {
				(250_200_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(11 as Weight))
				.saturating_add(T::DbWeight::get().writes(13 as Weight))
				}
				fn add_liquidity() -> Weight {
				(239_134_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(9 as Weight))
				.saturating_add(T::DbWeight::get().writes(8 as Weight))
				}
				fn remove_liquidity() -> Weight {
				(240_260_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(8 as Weight))
				.saturating_add(T::DbWeight::get().writes(7 as Weight))
				}
				fn sell() -> Weight {
				(169_053_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(5 as Weight))
				.saturating_add(T::DbWeight::get().writes(4 as Weight))
				}
				fn buy() -> Weight {
				(168_649_000 as Weight)
				.saturating_add(T::DbWeight::get().reads(5 as Weight))
				.saturating_add(T::DbWeight::get().writes(4 as Weight))
				}
				}

				// For backwards compatibility and tests
				impl WeightInfo for () {
				fn create_pool() -> Weight {
				(250_200_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(11 as Weight))
				.saturating_add(RocksDbWeight::get().writes(13 as Weight))
				}
				fn add_liquidity() -> Weight {
				(239_134_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(9 as Weight))
				.saturating_add(RocksDbWeight::get().writes(8 as Weight))
				}
				fn remove_liquidity() -> Weight {
				(240_260_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(8 as Weight))
				.saturating_add(RocksDbWeight::get().writes(7 as Weight))
				}
				fn sell() -> Weight {
				(169_053_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(5 as Weight))
				.saturating_add(RocksDbWeight::get().writes(4 as Weight))
				}
				fn buy() -> Weight {
				(168_649_000 as Weight)
				.saturating_add(RocksDbWeight::get().reads(5 as Weight))
				.saturating_add(RocksDbWeight::get().writes(4 as Weight))
				}
				}