extern crate proc_macro;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Expr};

#[proc_macro]
pub fn coroutine(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parser = Punctuated::<Expr, Comma>::parse_terminated;
    let input_exprs = parser.parse(input).unwrap();
    let mut match_arms = quote!();
    input_exprs.iter().enumerate().for_each(|(i, step_fn)| {
        match_arms.extend(quote!(#i => {#step_fn;},));
    });
    quote! {
        match co.step() {
            #match_arms
            _ => ()
        }
    }
    .into()
}
