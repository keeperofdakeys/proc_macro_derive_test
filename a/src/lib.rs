#![feature(proc_macro)]

#[macro_use]
extern crate b;

trait Test {
  fn test<T: ::std::iter::Iterator>(T);
}

#[derive(Test)]
struct A();
