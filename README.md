# actix-signal

Manage the lifecycle of an actix actor with its address.

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

derive - Provide `#[derive(SignalHandler)]` proc-macro.

# License

MIT