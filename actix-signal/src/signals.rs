use actix::Message;

#[derive(Debug, Copy, Clone, Message)]
#[rtype("()")]
pub struct StopSignal;

#[derive(Debug, Copy, Clone, Message)]
#[rtype("()")]
pub struct TerminateSignal;