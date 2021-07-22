use actix::{Actor, Addr};

use crate::{SignalHandler, StopSignal, TerminateSignal, ToSignalEnvelope};

/// An extension trait for `Addr`s that provides convenient methods to control associated actors' lifecycle.
///
/// The actor must implement the [`SignalHandler`](crate::SignalHandler) trait for these methods to work.
///
/// # Delayed operation
///
/// These methods are different from their context counterparts which have immediate effects. Instead, they are queued
/// to the mailbox, subjecting to message sending and handling delays. Moreover, if the actor stops/terminates/panics
/// before handling the signal, no actual operation will be performed.
///
/// # Signal convention
///
/// There's no guarantee how the actor may react to signals. A misbehaved actor may act completely different from your
/// intention.
///
/// See [`signals`](crate::signals) module for details.
pub trait AddrSignalExt {
    /// Stop the actor associated with the address.
    ///
    /// This method should have the same effect as [ActorContext::stop](actix::ActorContext::stop).
    fn stop(&self);
    /// Terminate the actor associated with the address.
    ///
    /// This method should have the same effect as [ActorContext::terminate](actix::ActorContext::terminate).
    fn terminate(&self);
}

impl<A> AddrSignalExt for Addr<A>
where
    A: Actor + SignalHandler,
    <A as Actor>::Context: ToSignalEnvelope<A>,
{
    fn stop(&self) {
        self.do_send(StopSignal)
    }
    fn terminate(&self) {
        self.do_send(TerminateSignal)
    }
}
