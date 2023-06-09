// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Utility functions to interact with Substrate's binary merkle tree.

#![cfg_attr(not(feature = "std"), no_std)]

use merkle_tree_db::IndexTree;
pub use merkle_tree_db::{
	DBValue, HashDBRef, Hasher, IndexTreeDBBuilder, TreeDBBuilder, TreeError, TreeRecorder,
};
use sp_std::vec::Vec;

/// Read a value from binary merkle tree.
pub fn read_child_tree_value<H: Hasher, DB: HashDBRef<H, DBValue>>(
	db: &DB,
	root: &H::Out,
	index: &u64,
	recorder: Option<&mut dyn TreeRecorder<H>>,
) -> Result<Option<Vec<u8>>, TreeError> {
	IndexTreeDBBuilder::<1, H>::new(db, root)
		.expect("Could not create merkle tree")
		.with_optional_recorder(recorder)
		.build()
		.value(index)
		.map(|x| x.map(|val| val.to_vec()))
}

pub fn empty_child_trie_root<H: Hasher>() -> H::Out {
	Default::default()
}
