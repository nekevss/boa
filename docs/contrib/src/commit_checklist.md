# PR Checklist

Below is a helpful checklist prior to submitting a Pull Request or even committing changes to your branch.

General Checks
 - `cargo fmt` -> Run Rustfmt lints
 - `cargo clippy` -> Run clippy lints
 - `cargo build` -> Double check that your branch builds
 - `cargo test` -> Check that the PR passes all of Boa's tests
 
Do you need to check the ECMAScript test suite?
 - `cargo run --release --bin boa_tester -- run -v 2> error.log` -> Run the ECMAScript test suite

More information on Boa's ECMAScript testing can be found in Testing.

Did you write documentation code examples?
 - `cargo test --doc -- --show-output` -> Check that Boa's docs compile