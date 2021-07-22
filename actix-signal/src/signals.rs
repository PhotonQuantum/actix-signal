//! Signals that can be handled by actors implementing [`SignalHandler`](crate::SignalHandler).
//!
//! Notice that the document is just indicating a set of conventions that should be followed by
//! implementors (and the `#[derive(SignalHandler)]` macro). There's no guarantee that types must
//! work as described below.

use actix::Message;

/// Stops the actor receiving this signal.
///
/// Normally you should use [`AddrSignalExt::stop`](crate::AddrSignalExt::stop) method instead of
/// sending it to actors directly.
#[derive(Debug, Copy, Clone, Message)]
#[rtype("()")]
pub struct StopSignal;

/// Terminates the actor receiving this signal.
///
/// Normally you should use [`AddrSignalExt::terminate`](crate::AddrSignalExt::terminate) method
/// instead of sending it to actors directly.
#[derive(Debug, Copy, Clone, Message)]
#[rtype("()")]
pub struct TerminateSignal;
