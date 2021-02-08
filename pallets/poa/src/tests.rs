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

use super::*;

use frame_support::{impl_outer_origin, parameter_types};
use sp_core::{crypto::key_types, H256};
use sp_runtime::{
    testing::{Header, UintAuthorityId},
    traits::{BlakeTwo256, ConvertInto, IdentityLookup, OpaqueKeys},
    KeyTypeId, Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
}
impl system::Config for Test {
    type Origin = Origin;
    type Call = ();
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
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}
pub type AuthorityId = u64;
pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AuthorityId> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [KeyTypeId] = &[key_types::DUMMY];

    fn on_new_session<Ks: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AuthorityId, Ks)],
        _queued_validators: &[(AuthorityId, Ks)],
    ) {
    }

    fn on_disabled(_validator_index: usize) {}

    fn on_genesis_session<Ks: OpaqueKeys>(_validators: &[(AuthorityId, Ks)]) {}
}
impl pallet_session::ShouldEndSession<u64> for TestSessionHandler {
    fn should_end_session(_now: u64) -> bool {
        false
    }
}
impl pallet_session::Config for Test {
    type SessionManager = Module<Test>;
    type SessionHandler = TestSessionHandler;
    type ShouldEndSession = TestSessionHandler;
    type NextSessionRotation = ();
    type Event = ();
    type Keys = UintAuthorityId;
    type ValidatorId = <Test as system::Config>::AccountId;
    type ValidatorIdOf = ConvertInto;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = ();
}
impl Config for Test {}

type SessionModule = pallet_session::Module<Test>;
type TestModule = Module<Test>;

pub const VALIDATOR: u64 = 1;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

#[test]
fn validators_update_propagate() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR]);

        SessionModule::rotate_session();
        let queued_keys = SessionModule::queued_keys();
        assert_eq!(queued_keys.len(), 1);
        assert_eq!(queued_keys[0].0, VALIDATOR);

        SessionModule::rotate_session();
        assert_eq!(SessionModule::validators(), vec![VALIDATOR]);
    })
}

#[test]
fn change_members_sorted() {
    new_test_ext().execute_with(|| {
        TestModule::change_members_sorted(&[], &[], &[VALIDATOR]);
        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR]));
    })
}

#[test]
fn new_session_return_members() {
    new_test_ext().execute_with(|| {
        TestModule::initialize_members(&[VALIDATOR]);
        assert_eq!(TestModule::new_session(0), Some(vec![VALIDATOR]));
    })
}
