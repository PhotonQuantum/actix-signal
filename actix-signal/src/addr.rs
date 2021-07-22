use actix::{Actor, Addr, Context};

use crate::{SignalHandler, StopSignal, ToSignalEnvelope, TerminateSignal};
use actix::dev::ToEnvelope;

pub trait AddrSignalExt {
    fn stop(&self);
    fn terminate(&self);
}

impl<A> AddrSignalExt for Addr<A>
where
    A: SignalHandler + Actor,
    <A as Actor>::Context: ToSignalEnvelope<A>,
{
    fn stop(&self) {
        self.do_send(StopSignal)
    }
    fn terminate(&self) {
        self.do_send(TerminateSignal)
    }
}
