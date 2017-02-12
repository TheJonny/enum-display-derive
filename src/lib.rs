extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Display)]
pub fn display(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    let gen = match ast.body {
        syn::Body::Enum(ref variants) => {
            let name = &ast.ident;
            impl_display(name, variants)
        }
        _ => panic!("#[derive(Display)] works only on enums"),
    };

    gen.parse().unwrap()
}

fn impl_display(name: &syn::Ident, variants: &[syn::Variant]) -> quote::Tokens {
    let variants = impl_display_for_variants(name, variants);
    quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    #(#variants)*
                }
            }
        }
    }
}

fn impl_display_for_variants(name: &syn::Ident, variants: &[syn::Variant]) -> Vec<quote::Tokens> {
    variants.iter()
        .map(|variant| {
            let id = &variant.ident;
            match variant.data {
                syn::VariantData::Unit => {
                    quote! {
                        #name::#id => {
                            f.write_str(stringify!(#id))
                        }
                    }
                }
                _ => panic!("#[derive(Display)] works only with unit variants"),
            }
        })
        .collect()
}