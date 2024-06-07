// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Tool for creating the genesis block.

use std::{collections::hash_map::DefaultHasher, marker::PhantomData, sync::Arc};

use sc_client_api::{backend::Backend, BlockImportOperation};
use sc_executor::RuntimeVersionOf;
use sp_core::storage::{well_known_keys, StateVersion, Storage};
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, Header as HeaderT, Zero},
	BuildStorage,
};
use sp_runtime::OpaqueExtrinsic;
use codec::{Encode, Decode};
use hex_literal::hex;

/// Return the state version given the genesis storage and executor.
pub fn resolve_state_version_from_wasm<E>(
	storage: &Storage,
	executor: &E,
) -> sp_blockchain::Result<StateVersion>
where
	E: RuntimeVersionOf,
{
	if let Some(wasm) = storage.top.get(well_known_keys::CODE) {
		let mut ext = sp_state_machine::BasicExternalities::new_empty(); // just to read runtime version.

		let code_fetcher = sp_core::traits::WrappedRuntimeCode(wasm.as_slice().into());
		let runtime_code = sp_core::traits::RuntimeCode {
			code_fetcher: &code_fetcher,
			heap_pages: None,
			hash: {
				use std::hash::{Hash, Hasher};
				let mut state = DefaultHasher::new();
				wasm.hash(&mut state);
				state.finish().to_le_bytes().to_vec()
			},
		};
		let runtime_version = RuntimeVersionOf::runtime_version(executor, &mut ext, &runtime_code)
			.map_err(|e| sp_blockchain::Error::VersionInvalid(e.to_string()))?;
		Ok(runtime_version.state_version())
	} else {
		Err(sp_blockchain::Error::VersionInvalid(
			"Runtime missing from initial storage, could not read state version.".to_string(),
		))
	}
}

/// Create a genesis block, given the initial storage.
pub fn construct_genesis_block<Block: BlockT>(
	state_root: Block::Hash,
	state_version: StateVersion,
) -> Block {
    // Genesis remark ext
    let remark_ext = hex!("39028400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d018eed6c7362d2979ef958ec8cbe6761e22d7526b736e20f85bb15901a8d8fbc39682bf991118abaae3ef1783f3a7420b816da1a8c29b1e60be3a950bb4f6c1086040000000000008c546869732069732067656e657369732072656d61726b207472616e73616374696f6e21");
    // Genesis DA ext
    let da_ext = hex!("59028400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0198ff827195df473a04f7465760c2bc87950e1b8962a974e772ae884022d66b354e347feb071a744c3e8f5b90bde02324f8d2453395cc26e660ee41410437a68844000000001d01ac48692c20686f70652074686973204441207478206c616e647320696e2067656e6573697320626c6f636b21");

    // Add extrinsics to be included in the genesis block
    let extrinsics = vec![
        OpaqueExtrinsic::from_bytes(&remark_ext).expect("We know what we're doing!"),
        OpaqueExtrinsic::from_bytes(&da_ext).expect("We know what we're doing!"),
    ];

    // Convert OpaqueExtrinsic to Block::Extrinsic
    let block_extrinsics: Vec<Block::Extrinsic> = extrinsics.into_iter()
        .map(|ext| Decode::decode(&mut &ext.encode()[..]).expect("Extrinsic conversion failed"))
        .collect();

    // Collect encoded extrinsics
    let extrinsics_encoded: Vec<Vec<u8>> = block_extrinsics.iter().map(Encode::encode).collect();
    
    // Compute the extrinsics root
    let extrinsics_root = <<Block as BlockT>::Header as HeaderT>::Hashing::ordered_trie_root(
        extrinsics_encoded,
        state_version,
    );

    // Construct the genesis block
    Block::new(
        <<Block as BlockT>::Header as HeaderT>::new(
            Zero::zero(),
            extrinsics_root,
            state_root,
            Default::default(),
            Default::default(),
        ),
        block_extrinsics,
    )
}

/// Trait for building the genesis block.
pub trait BuildGenesisBlock<Block: BlockT> {
	/// The import operation used to import the genesis block into the backend.
	type BlockImportOperation;

	/// Returns the built genesis block along with the block import operation
	/// after setting the genesis storage.
	fn build_genesis_block(self) -> sp_blockchain::Result<(Block, Self::BlockImportOperation)>;
}

/// Default genesis block builder in Substrate.
pub struct GenesisBlockBuilder<Block: BlockT, B, E> {
	genesis_storage: Storage,
	commit_genesis_state: bool,
	backend: Arc<B>,
	executor: E,
	_phantom: PhantomData<Block>,
}

impl<Block: BlockT, B: Backend<Block>, E: RuntimeVersionOf> GenesisBlockBuilder<Block, B, E> {
	/// Constructs a new instance of [`GenesisBlockBuilder`].
	pub fn new(
		build_genesis_storage: &dyn BuildStorage,
		commit_genesis_state: bool,
		backend: Arc<B>,
		executor: E,
	) -> sp_blockchain::Result<Self> {
		let genesis_storage =
			build_genesis_storage.build_storage().map_err(sp_blockchain::Error::Storage)?;
		Ok(Self {
			genesis_storage,
			commit_genesis_state,
			backend,
			executor,
			_phantom: PhantomData::<Block>,
		})
	}
}

impl<Block: BlockT, B: Backend<Block>, E: RuntimeVersionOf> BuildGenesisBlock<Block>
	for GenesisBlockBuilder<Block, B, E>
{
	type BlockImportOperation = <B as Backend<Block>>::BlockImportOperation;

	fn build_genesis_block(self) -> sp_blockchain::Result<(Block, Self::BlockImportOperation)> {
		let Self { genesis_storage, commit_genesis_state, backend, executor, _phantom } = self;

		let genesis_state_version = resolve_state_version_from_wasm(&genesis_storage, &executor)?;
		let mut op = backend.begin_operation()?;
		let state_root =
			op.set_genesis_state(genesis_storage, commit_genesis_state, genesis_state_version)?;
		let genesis_block = construct_genesis_block::<Block>(state_root, genesis_state_version);

		Ok((genesis_block, op))
	}
}
