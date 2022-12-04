# Testing in Boa

There are two primary methods of testing used in Boa: Boa's units tests and tc39's [Test262](test262).

## Unit Test Overview

These are the tests written in the Boa's test modules.

To run the unit tests, you simply navigate to the Boa directory on your machine and run `cargo test`

Helpful note: use `cargo test -- --nocapture` to view any print statements 

## tc39's Test262 Overview

tc39's Test262 is the ECMAScript conformance test suite. As the the previous 
statement may imply, this test suite determines Boa's current conformance to 
the ECMAScript language specification.

Boa's conformance results are updated and posted to Boa's [website](conformance)
on each merge.

[test262]: https://github.com/tc39/test262
[conformance]: https://boa-dev.github.io/boa/test262/