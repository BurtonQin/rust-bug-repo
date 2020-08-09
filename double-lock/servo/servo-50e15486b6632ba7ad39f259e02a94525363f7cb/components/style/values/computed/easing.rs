/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Computed types for CSS Easing functions.

use crate::values::computed::{Integer, Number};
use crate::values::generics::easing;

/// A computed timing function.
pub type ComputedTimingFunction = easing::TimingFunction<Integer, Number>;

/// An alias of the computed timing function.
pub type TimingFunction = ComputedTimingFunction;
