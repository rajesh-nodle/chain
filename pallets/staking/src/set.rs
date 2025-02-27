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
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

/// An ordered set backed by `Vec`
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, PartialEq, Eq, Encode, Decode, Clone, scale_info::TypeInfo)]
pub struct OrderedSet<T>(pub Vec<T>);

impl<T> Default for OrderedSet<T> {
	/// Create a default empty set
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl<T: Ord> OrderedSet<T> {
	/// Create a new empty set
	pub fn new() -> Self {
		Self(Vec::new())
	}

	/// Create a set from a `Vec`.
	/// `v` will be sorted and dedup first.
	pub fn from(mut v: Vec<T>) -> Self {
		v.sort();
		v.dedup();
		Self::from_sorted_set(v)
	}

	/// Create a set from a `Vec`.
	/// Assume `v` is sorted and contain unique elements.
	pub fn from_sorted_set(v: Vec<T>) -> Self {
		Self(v)
	}

	/// Insert an element.
	/// Return true if insertion happened.
	pub fn insert(&mut self, value: T) -> bool {
		match self.0.binary_search(&value) {
			Ok(_) => false,
			Err(loc) => {
				self.0.insert(loc, value);
				true
			}
		}
	}

	/// Remove an element.
	/// Return true if removal happened.
	pub fn remove(&mut self, value: &T) -> bool {
		match self.0.binary_search(value) {
			Ok(loc) => {
				self.0.remove(loc);
				true
			}
			Err(_) => false,
		}
	}

	/// Return if the set contains `value`
	pub fn contains(&self, value: &T) -> Option<usize> {
		// self.0.binary_search(&value).is_ok()
		match self.0.binary_search(value) {
			Ok(loc) => Some(loc),
			Err(_) => None,
		}
	}

	/// Clear the set
	pub fn clear(&mut self) {
		self.0.clear();
	}
}

impl<T: Ord> From<Vec<T>> for OrderedSet<T> {
	fn from(v: Vec<T>) -> Self {
		Self::from(v)
	}
}
