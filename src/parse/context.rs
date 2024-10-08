// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use thiserror::Error;

use crate::parse::{
    proto::extensions::SimpleExtensionUri, Anchor, Parse,
};

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {
    /// Parse an item with this context.
    ///
    /// See [Parse::parse].
    fn parse<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }
}

/// Parse context errors.
#[derive(Debug, Error, PartialEq)]
pub enum ContextError {
    /// Undefined reference to simple extension.
    #[error("undefined reference to simple extension with anchor `{0}`")]
    UndefinedSimpleExtension(Anchor<SimpleExtensionUri>),

    /// Duplicate anchor for simple extension.
    #[error("duplicate anchor `{0}` for simple extension")]
    DuplicateSimpleExtension(Anchor<SimpleExtensionUri>),

    /// Unsupported simple extension URI.
    #[error("unsupported simple extension URI: {0}")]
    UnsupportedURI(String),
}
