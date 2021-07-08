// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Information about the networking, for diagnostic purposes.
//!
//! **Warning**: These APIs are not stable.

use libp2p::{core::ConnectedPoint, Multiaddr};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, time::Duration};

/// Returns general information about the networking.
///
/// Meant for general diagnostic purposes.
///
/// **Warning**: This API is not stable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkState {
	/// PeerId of the local substrate.
	pub peer_id: String,
	/// List of addresses the substrate is currently listening on.
	pub listened_addresses: HashSet<Multiaddr>,
	/// List of addresses the substrate knows it can be reached as.
	pub external_addresses: HashSet<Multiaddr>,
	/// List of substrate we're connected to.
	pub connected_peers: HashMap<String, Peer>,
	/// List of substrate that we know of but that we're not connected to.
	pub not_connected_peers: HashMap<String, NotConnectedPeer>,
	/// State of the peerset manager.
	pub peerset: serde_json::Value,
}

/// Part of the `NetworkState` struct. Unstable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
	/// How we are connected to the substrate.
	pub endpoint: PeerEndpoint,
	/// Node information, as provided by the substrate itself. Can be empty if not known yet.
	pub version_string: Option<String>,
	/// Latest ping duration with this substrate.
	pub latest_ping_time: Option<Duration>,
	/// List of addresses known for this substrate.
	pub known_addresses: HashSet<Multiaddr>,
}

/// Part of the `NetworkState` struct. Unstable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotConnectedPeer {
	/// List of addresses known for this substrate.
	pub known_addresses: HashSet<Multiaddr>,
	/// Node information, as provided by the substrate itself, if we were ever connected to this substrate.
	pub version_string: Option<String>,
	/// Latest ping duration with this substrate, if we were ever connected to this substrate.
	pub latest_ping_time: Option<Duration>,
}

/// Part of the `NetworkState` struct. Unstable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PeerEndpoint {
	/// We are dialing the given address.
	Dialing(Multiaddr),
	/// We are listening.
	Listening {
		/// Local address of the connection.
		local_addr: Multiaddr,
		/// Address data is sent back to.
		send_back_addr: Multiaddr,
	},
}

impl From<ConnectedPoint> for PeerEndpoint {
	fn from(endpoint: ConnectedPoint) -> Self {
		match endpoint {
			ConnectedPoint::Dialer { address } =>
				PeerEndpoint::Dialing(address),
			ConnectedPoint::Listener { local_addr, send_back_addr } =>
				PeerEndpoint::Listening {
					local_addr,
					send_back_addr
				}
		}
	}
}
