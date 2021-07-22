//! Manage the lifecycle of an actix actor with its address.
//!
//! If you want to stop/terminate an actor, you call [`ActorContext::stop`](actix::ActorContext::stop) or
//! [`ActorContext::terminate`](actix::ActorContext::terminate) within its execution context.
//!
//! However, sometimes you have access to its address only. This crate add a bunch of methods to the address so that you
//! may [stop](AddrSignalExt::stop) or [terminate](AddrSignalExt::terminate) the actor outside its running context.
//!
//! # Example
//!
//! ```
//! use actix::{Actor, Context};
//! use actix_signal::{AddrSignalExt, SignalHandler};
//! # use actix_signal_derive::SignalHandler;
//!
//! #[derive(SignalHandler)]
//! struct MyActor;
//!
//! impl Actor for MyActor {
//!     type Context = Context<Self>;
//! }
//!
//! # #[actix_rt::test]
//! # async fn test() {
//! let actor = MyActor;
//! let addr = actor.start();
//!
//! addr.stop();        // Stop the actor
//! addr.terminate();   // Terminate the actor
//! # }
//! ```
//!
//! You may also implement handlers by hand if you don't want to use the proc macro or need custom behaviors.
//!
//! ```
//! use actix::{Actor, Context, Handler, ActorContext};
//! use actix_signal::{AddrSignalExt, StopSignal, TerminateSignal};
//!
//! struct MyActor;
//!
//! impl Actor for MyActor {
//!     type Context = Context<Self>;
//! }
//!
//! impl Handler<StopSignal> for MyActor {
//!     type Result = ();
//!
//!     fn handle(&mut self, _msg: StopSignal, ctx: &mut Self::Context) -> Self::Result {
//!         ctx.stop();
//!     }
//! }
//!
//! impl Handler<TerminateSignal> for MyActor {
//!     type Result = ();
//!
//!     fn handle(&mut self, _msg: TerminateSignal, ctx: &mut Self::Context) -> Self::Result {
//!         ctx.terminate();
//!     }
//! }
//!
//! # #[actix_rt::test]
//! # async fn test() {
//! let actor = MyActor;
//! let addr = actor.start();
//!
//! addr.stop();        // Stop the actor
//! addr.terminate();   // Terminate the actor
//! # }
//! ```
//!
//! # Feature flags
//!
//! `derive` - Provide `#[derive(SignalHandler)]` proc-macro.

#[cfg(feature = "actix-signal-derive")]
#[macro_use]
extern crate actix_signal_derive;

use actix::dev::ToEnvelope;
use actix::{Actor, Handler};

#[cfg(feature = "actix-signal-derive")]
pub use actix_signal_derive::*;
pub use addr::*;
pub use signals::*;

mod addr;
pub mod signals;

/// Actors that are able to handle signals.
///
/// This trait is automatically implemented for actors implementing `Handler`s for [`signals`](signals).
/// See top-level document for instructions on implementing these traits.
pub trait SignalHandler: Handler<StopSignal> + Handler<TerminateSignal> {}

impl<T> SignalHandler for T where T: Handler<StopSignal> + Handler<TerminateSignal> {}

/// Execution contexts that are able to handle signals.
///
/// This trait is automatically implemented for contexts of which the associated actors implement `Handler`s for
/// [`signals`](signals).
/// See top-level document for instructions on implementing these traits.
pub trait ToSignalEnvelope<A>: ToEnvelope<A, StopSignal> + ToEnvelope<A, TerminateSignal>
where
    A: Actor + SignalHandler,
    <A as Actor>::Context: ToSignalEnvelope<A>,
{
}

impl<A, T> ToSignalEnvelope<A> for T
where
    A: Actor<Context = T> + SignalHandler,
    T: ToEnvelope<A, StopSignal> + ToEnvelope<A, TerminateSignal>,
    <A as Actor>::Context: ToEnvelope<A, StopSignal> + ToEnvelope<A, TerminateSignal>,
{
}
