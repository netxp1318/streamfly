# streamfly

Streamfly aims to be a stream-oriented Pub/Sub framework.

Unlike traditional Pub/Sub systems, instead of transffering data
packets(messages), streamfly focuses on transffering streams, which means users
could publish or subscrbie a stream(Reader & Writer). Then developers can
manipulate these streams in their applications, just like the stdin & stdout.

## Build

- build streamfly cli command

```sh
cargo build
```

- build examples

```sh
cargo build --examples
```

## Run

- start streamfly server

```sh
RUST_LOG=debug ./target/debug/streamfly serve
```

- subscribe a stream

```sh
./target/debug/examples/sub
```

- publish a stream, and then write data

```sh
./target/debug/examples/pub
```
