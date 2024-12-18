# Running

```
cargo run --bin <file_name>
```

or

```
$ cargo build --bin <file_name>
$ ./target/debug/<file_name>
```

# Debugging

Either click the "Debug" annotation above a main function or run the `rust-analyzer: Debug` command. This allows debugging without a launch configuration.

# Testing

```
cargo test --bin <file_name>
```

or, to see console logs in tested code:

```
cargo test --bin vectors -- tests --show-output
```
