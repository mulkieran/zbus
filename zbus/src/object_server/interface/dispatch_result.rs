use std::{future::Future, pin::Pin};

use crate::Result;

/// A helper type returned by [`Interface`](`crate::object_server::Interface`) callbacks.
pub enum DispatchResult<'a> {
    /// This interface does not support the given method.
    NotFound,

    /// Retry with [Interface::call_mut](`crate::object_server::Interface::call_mut).
    ///
    /// This is equivalent to NotFound if returned by call_mut.
    RequiresMut,

    /// The method was found and will be completed by running this Future.
    Async(Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>),
}
