/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

#![cfg(test)]

use super::*;

use frame_support::{
    assert_noop, assert_ok, impl_outer_dispatch, impl_outer_origin, ord_parameter_types,
    parameter_types, weights::Weight,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use parity_scale_codec::Encode;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    DispatchError::BadOrigin,
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}
impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        frame_system::System,
    }
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1_000_000);
}
impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}
parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
    pub const MaxScheduledPerBlock: u32 = 50;
}
impl pallet_scheduler::Config for Test {
    type Event = ();
    type Origin = Origin;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type ScheduleOrigin = EnsureRoot<u64>;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = ();
}

ord_parameter_types! {
    pub const Proposer: u64 = 1;
    pub const Veto: u64 = 2;
    pub const Hacker: u64 = 3;
    pub const BlockDelay: u64 = 10;
}
impl Config for Test {
    type Event = ();
    type Amendment = Call;
    type SubmissionOrigin = EnsureSignedBy<Proposer, u64>;
    type VetoOrigin = EnsureSignedBy<Veto, u64>;
    type Delay = BlockDelay;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
}

type Amendments = Module<Test>;
type Scheduler = pallet_scheduler::Module<Test>;
type System = frame_system::Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

fn make_proposal(value: u64) -> Box<Call> {
    Box::new(Call::System(frame_system::Call::remark(value.encode())))
}

#[test]
fn non_authorized_origin_cannot_trigger_amendment() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Amendments::propose(Origin::signed(Hacker::get()), make_proposal(1)),
            BadOrigin
        );
    })
}

#[test]
fn call_gets_registered_correctly() {
    new_test_ext().execute_with(|| {
        assert_ok!(Amendments::propose(
            Origin::signed(Proposer::get()),
            make_proposal(1)
        ));
    })
}

#[test]
fn non_veto_origin_cannot_veto() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Amendments::veto(Origin::signed(Hacker::get()), 0),
            BadOrigin
        );
    })
}

#[test]
fn veto_proposal_before_delay_expired() {
    new_test_ext().execute_with(|| {
        assert_ok!(Amendments::propose(
            Origin::signed(Proposer::get()),
            make_proposal(1)
        ));

        assert_ok!(Amendments::veto(Origin::signed(Veto::get()), 0));
    })
}
