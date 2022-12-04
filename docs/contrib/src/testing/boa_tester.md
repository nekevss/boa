# Boa Tester Overview

As previously stated in this guide, `boa_tester` is Boa's tool for running 
the test262 ECMAScript conformance test suite. 

Note: by default, `boa_tester` runs tests in parallel using `rayon`'s `par_iter()`. 
This can be turned off with the correct flags.

## CLI flags

`boa_tester` takes two commands: `run` and `compare`

The flags for `boa_tester -- run` are:

 - `--verbose` (`-v`, `-vv`, `-vvv`) -> Determines how verbose the output is
 - `--test262-path` (default: ./test262) -> Path to the test suite
 - `--suite` (`-s`, default: test) -> specific test or test suite to run
 - `--output` (`-o`) -> output folder for results
 - `--disable-parallelism` (`-d`) -> execute tests serially
 - `--ignored` (`-i`, default: test_ignore.toml) -> path to a toml file with case to ignore

The flags for `boa_tester -- compare` are:

 - `--base` -> base results of comparison
 - `--new` -> new results to compare
 - `--markdown` (`-m`) -> whether to use markdown output