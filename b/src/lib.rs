#![feature(proc_macro, proc_macro_lib)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Test)]
pub fn test(input: TokenStream) -> TokenStream {
  let source = input.to_string();
  let ast = syn::parse_macro_input(&source).unwrap();
  let name = &ast.ident;
  let expanded = quote!(
    #ast

    impl Test for #name {
      fn test<T: ::std:iter::Iterator>(T) {
        println!("{}", T.iter().next().unwarp())
      }
    }
  );
  expanded.to_string().parse().unwrap()
}
