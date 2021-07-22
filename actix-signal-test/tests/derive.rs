use actix::{Actor, Context};
use actix_signal::AddrSignalExt;
use actix_signal_derive::SignalHandler;
use std::marker::PhantomData;

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
