/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Amendments pallet benchmarks

#![cfg(feature = "runtime-benchmarks")]
#![allow(unused)]

use super::*;

use crate::Pallet as Amendments;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{EnsureOrigin, UnfilteredDispatchable};
use frame_system::{Call as SystemCall, RawOrigin as SystemOrigin};
use sp_std::prelude::*;

const MAX_BYTES: u32 = 1_024;

benchmarks! {
	propose {
		let b in 1 .. MAX_BYTES;

		let amendment: T::Amendment = SystemCall::<T>::remark{ remark: vec![1; b as usize] }.into();
		let call = Call::<T>::propose{ amendment: Box::new(amendment) };
		let origin = T::SubmissionOrigin::successful_origin();
	}: { call.dispatch_bypass_filter(origin)? }

	veto {
		let amendment: T::Amendment = SystemCall::<T>::remark{ remark: vec![1; MAX_BYTES as usize] }.into();
		Pallet::<T>::propose(
			SystemOrigin::Root.into(),
			Box::new(amendment)
		)?;

		let call = Call::<T>::veto{ amendment_id: 0 };
		let origin = T::VetoOrigin::successful_origin();
	}: { call.dispatch_bypass_filter(origin)? }

	impl_benchmark_test_suite!(Amendments, crate::tests::new_test_ext(), crate::tests::Test,);

}
