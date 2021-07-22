# actix-signal

[![crates.io](https://img.shields.io/crates/v/actix-signal?style=flat-square)](https://crates.io/crates/actix-signal)
[![Documentation](https://img.shields.io/docsrs/actix-signal?style=flat-square)](https://docs.rs/actix-signal)

Manage the lifecycle of an actix actor with its address.

If you want to stop/terminate an actor, you call `ActorContext::stop` or `ActorContext::terminate` within its execution context.

However, sometimes you have access to its address only. This crate adds a bunch of methods to the address so that you
may stop or terminate the actor outside its running context.

## Get Started

Add the following line to your `Cargo.toml`.

```toml
actix-signal = { version = "0.1", features = ["derive"] }
```

## Example

```rust
use actix::{Actor, Context};
use actix_signal::SignalHandler;

#[derive(SignalHandler)]
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

let actor = MyActor;
let addr = actor.start();

addr.stop();        // Stop the actor
addr.terminate();   // Terminate the actor
```

# Feature flags

`derive` - Provide `#[derive(SignalHandler)]` proc-macro.

# License

MIT