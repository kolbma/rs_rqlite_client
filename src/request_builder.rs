//! Trait [`RequestBuilder`] is implemented to build a _runnable_ HTTP request

use crate::query::{Query, State};

/// Trait [`RequestBuilder`] is implemented to build a _runnable_ HTTP request
pub trait RequestBuilder<T: State> {
    /// Run the __request__
    ///
    /// # Errors
    ///
    /// May error with [`crate::error::Error`]
    ///
    fn run(&self, query: &Query<T>) -> crate::response::Result;
}
