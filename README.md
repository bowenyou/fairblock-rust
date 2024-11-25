# Fairblock rust

`crates` contains generated clients for keyshare, pep modules.

`bin` provides an example of how to use the crates.

## Instructions to update

1. Replace the files in the `proto/fairyring` directory.
2. Inside the `proto` directory, run `buf generate`.
3. In `fairblock-proto/src/prost/fairyring.keyshare.rs` add the `grpc` feature.

```rust
#[cfg(feature = "grpc")]
include!("fairyring.keyshare.tonic.rs");
```

4. In `fairblock-proto/src/prost/fairyring.pep.rs` add the `grpc` feature.

```rust
#[cfg(feature = "grpc")]
include!("fairyring.pep.tonic.rs");
```

5. Run the tests in `bin` to see if they pass.
