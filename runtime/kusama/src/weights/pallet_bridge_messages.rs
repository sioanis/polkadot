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
//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_messages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: unknown [0x3a143f4abdba538105373c7cdb014a50] (r:1 w:0)
	// Storage: unknown [0x8626edfd24ecc30ca2c42535541fa3a7] (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: unknown [0x51f1dbb0462eff18ec87eafdf9569989] (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: BridgePolkadotMessages OutboundMessages (r:0 w:9)
	fn send_minimal_message_worst_case() -> Weight {
		(45_686_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: unknown [0x3a143f4abdba538105373c7cdb014a50] (r:1 w:0)
	// Storage: unknown [0x8626edfd24ecc30ca2c42535541fa3a7] (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: unknown [0x51f1dbb0462eff18ec87eafdf9569989] (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: BridgePolkadotMessages OutboundMessages (r:0 w:9)
	fn send_1_kb_message_worst_case() -> Weight {
		(48_216_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: unknown [0x3a143f4abdba538105373c7cdb014a50] (r:1 w:0)
	// Storage: unknown [0x8626edfd24ecc30ca2c42535541fa3a7] (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: unknown [0x51f1dbb0462eff18ec87eafdf9569989] (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: BridgePolkadotMessages OutboundMessages (r:0 w:9)
	fn send_16_kb_message_worst_case() -> Weight {
		(67_693_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: BridgePolkadotMessages OutboundMessages (r:1 w:1)
	fn maximal_increase_message_fee() -> Weight {
		(3_777_465_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: BridgePolkadotMessages OutboundMessages (r:1 w:1)
	fn increase_message_fee(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	fn receive_single_message_proof() -> Weight {
		(77_857_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	fn receive_two_messages_proof() -> Weight {
		(136_688_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		(83_192_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	fn receive_single_message_proof_1_kb() -> Weight {
		(83_735_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	fn receive_single_message_proof_16_kb() -> Weight {
		(157_986_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages InboundLanes (r:1 w:1)
	fn receive_single_prepaid_message_proof() -> Weight {
		(75_794_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: BridgePolkadotMessages OutboundMessages (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn receive_delivery_proof_for_single_message() -> Weight {
		(47_675_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: BridgePolkadotMessages OutboundMessages (r:2 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		(50_397_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: BridgePolkadotMessages PalletOperatingMode (r:1 w:0)
	// Storage: BridgePolkadotGrandpa ImportedHeaders (r:1 w:0)
	// Storage: BridgePolkadotMessages OutboundLanes (r:1 w:1)
	// Storage: BridgePolkadotMessages OutboundMessages (r:2 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		(64_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
