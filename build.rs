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

use vergen::{ConstantsFlags, generate_cargo_keys};

const ERROR_MSG: &str = "Failed to generate metadata files";

fn main() {
	generate_cargo_keys(ConstantsFlags::SHA_SHORT).expect(ERROR_MSG);

	build_script_utils::rerun_if_git_head_changed();
}
