extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Described)]
pub fn described(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Check if the input is a struct
    if let Data::Struct(data) = input.data {
        // Check if the struct has named fields
        if let Fields::Named(fields) = data.fields {
            // Iterate over the fields and check if a field named "description" exists
            let has_field = fields.named.iter().any(|field| field.ident.as_ref().map_or(false, |ident| ident == "description"));
            // If the field does not exist, raise a compilation error

            if !has_field {
                return quote! {
                    compile_error!("Field description mast exist in derived struct");
                }.into();
            }
        }
    } else {
        return quote! {
                    compile_error!("Derived type mast be a struct");
                }.into();
    }
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl Described for #name {
            fn description(&self) -> String{
                self.description.clone()
            }
        }
    };
    TokenStream::from(expanded)
}
