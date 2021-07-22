use actix::{Actor, Context, Running};
use actix_signal::AddrSignalExt;
use actix_signal_derive::SignalHandler;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;

#[derive(SignalHandler)]
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

trait MyTrait {}

struct MyType;

impl MyTrait for MyType {}

#[derive(SignalHandler)]
struct MyActorWithGenerics<T>(PhantomData<T>);

impl<T> MyActorWithGenerics<T> {
    fn new(_data: T) -> Self {
        Self(PhantomData)
    }
}

impl<T: 'static + MyTrait + Unpin> Actor for MyActorWithGenerics<T> {
    type Context = Context<Self>;
}

#[actix_rt::test]
async fn test_actor() {
    let addr = MyActor.start();
    addr.stop();
    let addr = MyActor.start();
    addr.terminate();
}

#[actix_rt::test]
async fn test_actor_with_generis() {
    let addr = MyActorWithGenerics::new(MyType).start();
    addr.stop();
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ActorState {
    Running,
    Terminated,
    Stopping,
    Stopped,
}

impl From<u8> for ActorState {
    fn from(u: u8) -> Self {
        match u {
            0 => Self::Running,
            1 => Self::Terminated,
            2 => Self::Stopping,
            3 => Self::Stopped,
            _ => unreachable!(),
        }
    }
}

impl From<ActorState> for u8 {
    fn from(s: ActorState) -> Self {
        match s {
            ActorState::Running => 0,
            ActorState::Terminated => 1,
            ActorState::Stopping => 2,
            ActorState::Stopped => 3,
        }
    }
}

#[derive(Debug)]
struct ActorStateContainer(AtomicU8);

impl ActorStateContainer {
    fn new() -> Self {
        Self(AtomicU8::new(ActorState::Running.into()))
    }
    fn set_stopping(&self) {
        self.0.store(ActorState::Stopping.into(), SeqCst);
    }
    fn set_stopped(&self) {
        self.0.fetch_add(1, SeqCst);
    }
    fn get(&self) -> ActorState {
        self.0.load(SeqCst).into()
    }
}

#[derive(SignalHandler)]
struct DummyActor(Rc<ActorStateContainer>);

impl Actor for DummyActor {
    type Context = Context<Self>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.0.set_stopping();
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.0.set_stopped();
    }
}

#[actix_rt::test]
async fn test_actor_state() {
    let state = Rc::new(ActorStateContainer::new());
    let actor = DummyActor(state.clone());
    let addr = actor.start();
    addr.stop();
    actix_rt::time::sleep(Duration::from_micros(100)).await;
    assert_eq!(state.get(), ActorState::Stopped);

    let state = Rc::new(ActorStateContainer::new());
    let actor = DummyActor(state.clone());
    let addr = actor.start();
    addr.terminate();
    actix_rt::time::sleep(Duration::from_micros(100)).await;
    assert_eq!(state.get(), ActorState::Terminated);
}
