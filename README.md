# Ancestor
Async client/server library in Rust.

## Brief overview
To run server, use the following command:

`cargo run -p server`

To run client:

`cargo run -p client`

To adjust settings, add `-- some_arguments` after startup command.
For help use `-- -h`.

**Important**
Do not start server with `--release` flag, because compiler will optimize fake computation.

Environment:

```
rustc 1.23.0-nightly (45594d5de 2017-11-22)
rustfmt 0.2.16-nightly 
clippy 0.0.174
```
