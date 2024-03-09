use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, parse_macro_input, FnArg, ItemFn, Pat};

#[proc_macro_attribute]
pub fn cache(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(input as ItemFn);

    let inputs = function
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                FnArg::Typed(pat) => match &*pat.pat {
                    Pat::Ident(ident) => Some(quote! { #ident.clone() }),
                    _ => None,
                },
                FnArg::Receiver(_) => None, // Skip 'self'
            }
        })
        .collect::<Vec<_>>();

    let original = function
        .block
        .stmts
        .iter()
        .map(quote::ToTokens::to_token_stream);
    let original = quote! { #(#original)* };

    let stmts = &mut function.block.stmts;
    stmts.clear();
    if inputs.len() == 1 {
        let key = &inputs[0];
        stmts.push(
            parse2(quote! {
                let key = #key;
            })
            .unwrap(),
        );
    } else {
        stmts.push(
            parse2(quote! {
                let key = (#(#inputs),*);
            })
            .unwrap(),
        );
    }

    stmts.push(
        parse2(quote! {
            let value = self.cache
                .get_with(key, async move { #original })
                .await;
        })
        .unwrap(),
    );

    stmts.push(
        parse2(quote! {
            return value;
        })
        .unwrap(),
    );

    TokenStream::from(quote! {
        #function
    })
}
