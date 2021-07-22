use actix::{Actor, Handler, Message};
use actix::dev::ToEnvelope;

pub use addr::*;
pub use signals::*;

mod addr;
mod signals;

pub trait SignalHandler: Handler<StopSignal> + Handler<TerminateSignal> {}

pub trait ToSignalEnvelope<A>: ToEnvelope<A, StopSignal> + ToEnvelope<A, TerminateSignal>
    where
        A: Actor + SignalHandler,
        <A as Actor>::Context: ToSignalEnvelope<A>,
{}


impl<A, T> ToSignalEnvelope<A> for T
    where
        A: Actor<Context=T> + SignalHandler,
        T: ToEnvelope<A, StopSignal>,
        T: ToEnvelope<A, TerminateSignal>,
        <A as Actor>::Context: ToEnvelope<A, StopSignal>,
        <A as Actor>::Context: ToEnvelope<A, TerminateSignal>
{}