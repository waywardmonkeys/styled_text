// Copyright 2025 the Styled Text Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::string::String;
use alloc::sync::Arc;

/// A block of text that will be wrapped by an `AttributedText`.
///
/// TODO: Should this handle mutations to the underlying text and
/// propagating them back to the ranges within the `AttributedText`?
pub trait TextStorage {
    /// The length of the underlying text.
    fn len(&self) -> usize;

    /// Return `true` if the underlying text is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl TextStorage for String {
    fn len(&self) -> usize {
        Self::len(self)
    }
}

impl TextStorage for &str {
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl TextStorage for Arc<str> {
    fn len(&self) -> usize {
        str::len(self)
    }
}
