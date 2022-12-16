# Experiment with once_cell

Based on this [post](https://users.rust-lang.org/t/is-there-an-easy-way-to-implement-singleton-object-in-rust/72084/4)
and [OnceCell docs](https://docs.rs/once_cell/latest/once_cell/).



## Run:

```
wink@3900x 22-12-16T23:34:49.585Z:~/prgs/rust/myrepos/exper_once_cell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/exper_once_cell`
s=one 1270280638
s=two 1270280638
vec: ["one 1270280638", "two 1270280638"]
use_thing: first=one 1270280638
use_thing: last=one 1270280638
use_two_threads thread: first=one 1270280638
use_two_threads thread: last=two 1270280638
use_two_threads main: first=one 1270280638
use_two_threads main: last=two 1270280638
```

## tests:

```
wink@3900x 22-12-16T23:36:14.626Z:~/prgs/rust/myrepos/exper_once_cell
$ cargo test
   Compiling exper_once_cell v0.1.0 (/home/wink/prgs/rust/myrepos/exper_once_cell)
    Finished test [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests src/lib.rs (target/debug/deps/exper_once_cell-63032b910487194e)

running 2 tests
test tests::test_single_thread ... ok
test tests::test_two_threads ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/exper_once_cell-83e993cc71867fa0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests exper_once_cell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
