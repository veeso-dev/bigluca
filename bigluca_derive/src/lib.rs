use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(AllVariants)]
pub fn all_attributes(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let name = &ident;

    if let syn::Data::Enum(dt_enum) = data {
        let mut tokens = Vec::new();

        for variant in &dt_enum.variants {
            let ident = &variant.ident;
            tokens.push(quote! {#name::#ident});
        }
        // Implement AllVariants for type
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
        panic!("AllVariants must be derived by a `Enum`")
    }
}

#[proc_macro_derive(ValidateAllPaths)]
pub fn validate_all_paths(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let name = &ident;

    let dt_struct = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("ValidateAllPaths must be derived by a `Struct`"),
    };

    let mut tokens = Vec::new();

    match dt_struct.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            for field in named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let ss = format!("field {} does not exist in {}", field_name, name);
                tokens.push(quote! { if !self.#field_name.exists() { anyhow::bail!(#ss); }});
            }
        }
        _ => panic!("struct {} does not contain named fields", ident),
    }
    // Implement ValidateAllPaths for type
    let output = quote! {
        impl Validate for #name {
            fn validate(&self) -> anyhow::Result<()> {
                #(#tokens)*

                Ok(())
            }
        }
    };

    output.into()
}

#[proc_macro_derive(ValidateAllFields)]
pub fn validate_all_fields(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let name = &ident;

    let dt_struct = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("ValidateAllFields must be derived by a `Struct`"),
    };

    let mut tokens = Vec::new();

    match dt_struct.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => {
            for field in named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                tokens.push(quote! {
                    self.#field_name.validate()?;
                });
            }
        }
        _ => panic!("struct {} does not contain named fields", ident),
    }
    // Implement ValidateAllFields for type
    let output = quote! {
        impl Validate for #name {
            fn validate(&self) -> anyhow::Result<()> {
                #(#tokens)*

                Ok(())
            }
        }
    };

    output.into()
}
