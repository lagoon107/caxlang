// External imports
use proc_macro::TokenStream as TkStream;
use quote::quote;

/// Display trait impl that simply calls Debug trait for printing.
#[proc_macro_derive(Display)]
pub fn derive_display(item: TkStream) -> TkStream {
    // Parse item to syn item
    let parsed_item = syn::parse_macro_input!(item as syn::DeriveInput);
    let parsed_item_ident = &parsed_item.ident;

    // Return derive impl of Display
    quote!{
        #[automatically_derived]
        impl std::fmt::Display for #parsed_item_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as std::fmt::Debug>::fmt(self, f)
            }
        }
    }.into()
}
