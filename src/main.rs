// Copyright 2017-2019 Parity Technologies (UK) Ltd.
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

//! Dothereum CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

#[macro_use]
extern crate rust_embed;

pub use sc_cli::{VersionInfo, IntoExit, error};

fn main() -> Result<(), cli::error::Error> {
	let version = VersionInfo {
		name: "Dothereum Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "dothereum",
		author: "Schoedon, Akinfiev, Pickle",
		description: "Dothereum",
		support_url: "https://dothereum.net",
	};

	cli::run(std::env::args(), cli::Exit, version)
}
