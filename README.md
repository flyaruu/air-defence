[![Continuous Integration](https://github.com/flyaruu/air-defence/actions/workflows/build.yml/badge.svg)](https://github.com/flyaruu/air-defence/actions/workflows/build.yml)

## Assignment
I hope I did not misunderstand the assignment, there wasn't much to go on. This can obviously be implemented with a single loop through a text file, but given that the assignment asks for a 'simulation' I've role-played a bit and created actual components that communicate by messages.


## Tech stack
I've used Rust, as it is a systems language that seems very suitable for this field, but it is more complex than most other languages would be.

- Clap: A very popular CLI parsing library to easily parse CLI parameters
- Tokio: De-facto standard runtime for async workloads. Obviously this exercise can be done without any threading, but I like the 'message passing' architecture the assignment hints at. At first I started using standard hardware threads, thinking it was simpler, but I did not really pan out.
- Broadcast channels: One of the reasons I switched to tokio was that I could use ergonomic broadcast channels, allowing for more than one subscriber.

## Special attention
- Event driven systems can be notorious to troubleshoot, I've put extra effort in the backtracability
- Error handling. I've tried being robust in error handling, for example unreadable data messages (due to io errors or parse errors) have a variant in downstream messages, so downstream components can respond to these. The only panics I've left in is when the actoal sending/receiving infrastructure fails.


## Testing
I have some unit testing for the parsing and IFF evaluation, the rest is more 'infrastructure', which is more cumbersome to test. (Integration test?)

## To Do
- CI
- ~~Integration test~~
- Run no_alloc
- ~~Add statistics viewer~~
- ~~Configurable buffer size~~

## Building
Make sure you have a recent version of rust installed (check https://rustup.rs/ if unsure)
Building:
```
cargo build
```

Run tests (few unit, one integration)
```
cargo test
```

Build a release version:
```
cargo build --release
```

## Using the cli tool
You can run directly using cargo:
```
```