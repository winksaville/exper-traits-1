# exper-traits-1

A first experiment with traits, modeling a crude state machine.

This works because I'm not returning `&'a mut SM` from process:
```
$ cargo run
   Compiling expr-traits-1 v0.1.0 (/home/wink/prgs/rust/myrepos/exper-traits-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/expr-traits-1`
State1: process sm.data1=124 Msg1::f1=123
124
125
``

It's weird I have to do that. It seems to me that after calling
process the borrowed SM should be "dropped" and I shouldn't have
to "return it" :(

## Building and running


## Test

TODO

## Benchmark

TODO

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
