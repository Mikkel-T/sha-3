# SHA-3

This is a Rust implementation of the SHA-3 and SHAKE cryptographic hash functions as specified in [FIPS 202](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf).

The code is in no way optimized for performance, but rather for clarity and simplicity with comments describing most of the steps in the algorithm with references to the specification.

## Running the code

An example of the usage of the SHA-3 and SHAKE functions can be found in the "sha3" crate under `examples/main.rs`, this can be run with the following command:

```bash
cargo run --example main
```

To run the unit tests, use the following command:

```bash
cargo test
```

To benchmark the functions with an empty input, use the following command:

```bash
cargo bench
```
