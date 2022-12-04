# Pull Request Checks

So you're ready to submit your pull request! Yay! There are a number of checks we run on any Pull Request that is submitted to Boa.

## ECMAScript official test suite (test262)

During this phase, we determine if the PR completes the ECMAScript test suite without any panics. This check is completed in an ubuntu-linux environment.

## Continuous integration

Boa's Continuous integration test runs a few checks.

### Coverage

During this phase, we determine the code coverage and upload it to Codecov for a report.

### Build and Test (macos-latest)

During this phase, we determine whether the Pull Request builds on the latest version of MacOS and passes Boa's test suite.

### Build and Test (windows-latest)

During this phase, we determine whther the Pull Request builds on the latest version of Windows and
passes Boa's test suite.

### Misc

During this phase, we check for a few things:

 - Lints
    - RustFmt
    - Clippy Lints
 - Documentation compilation
 - boa_examples compilation

# Webassembly demo

During this phase, we ensure that the WebAssembly build still compiles.