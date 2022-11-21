use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllAttributes)]
pub fn all_attributes(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let name = &ident;

    if let syn::Data::Enum(dt_enum) = data {
        let mut tokens = Vec::new();

        for variant in &dt_enum.variants {
            let ident = &variant.ident;
            tokens.push(quote! {#name::#ident});
        }
        // Implement AllAttributes for type
        let output = quote! {
                impl #ident {
                pub fn all() -> &'static [Self] {
                    &[
                        #(#tokens,)*
                    ]
                }
            }
        };

        output.into()
    } else {
        panic!("AllAttributes must be derived by a `Enum`")
    }
}
