```
cd a; cargo build
   Compiling b v0.1.0 (file:///tmp/test/b)
   Compiling a v0.1.0 (file:///tmp/test/a)
error: custom derive attribute panicked
  --> src/lib.rs:10:10
   |
10 | #[derive(Test)]
   |          ^^^^
   |
   = help: message: called `Result::unwrap()` on an `Err` value: LexError { _inner: () }

error: Could not compile `a`.

To learn more, run the command again with --verbose.
```
