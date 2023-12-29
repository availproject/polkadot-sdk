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

use super::*;
use crate::mock::{pallet_dummy::Call, DummyExtension, PreDispatchCount, Runtime, RuntimeCall};
use frame_support::dispatch::DispatchInfo;
use sp_runtime::traits::DispatchTransaction;

#[test]
fn feeless_if_works() {
	let call = RuntimeCall::DummyPallet(Call::<Runtime>::aux { data: 1 });
	SkipCheckIfFeeless::<Runtime, DummyExtension>::from(DummyExtension)
		.validate_and_prepare(Some(0).into(), &call, &DispatchInfo::default(), 0)
		.unwrap();
	assert_eq!(PreDispatchCount::get(), 1);

	let call = RuntimeCall::DummyPallet(Call::<Runtime>::aux { data: 0 });
	SkipCheckIfFeeless::<Runtime, DummyExtension>::from(DummyExtension)
		.validate_and_prepare(Some(0).into(), &call, &DispatchInfo::default(), 0)
		.unwrap();
	assert_eq!(PreDispatchCount::get(), 1);
}

#[test]
fn feeless_on_checkpoint_works() {
	let call = RuntimeCall::DummyPallet(Call::<Runtime>::aux_2 { data: 3 });
	SkipCheckIfFeeless::<Runtime, DummyExtension>::from(DummyExtension)
		.validate_and_prepare(Some(0).into(), &call, &DispatchInfo::default(), 0)
		.unwrap();
	assert_eq!(PreDispatchCount::get(), 1);

	let call = RuntimeCall::DummyPallet(Call::<Runtime>::aux_2 { data: 2 });
	SkipCheckIfFeeless::<Runtime, DummyExtension>::from(DummyExtension)
		.validate_and_prepare(Some(0).into(), &call, &DispatchInfo::default(), 0)
		.unwrap();
	assert_eq!(PreDispatchCount::get(), 1);
}
