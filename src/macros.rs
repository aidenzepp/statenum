use crate::helper::*;

pub fn _statenum(_args: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);

    // Get the visibility of the enum
    let visibility = &input.vis;

    // Ensure we're working with an enum
    let data = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("statenum can only be used with enums"),
    };

    // Generate a struct and State implementation for each variant
    let structs = data.variants.iter().map(|variant| {
        let struct_name = syn::Ident::new(&format!("{}", variant.ident), variant.ident.span());
        quote! {
            #visibility struct #struct_name;
            impl State for #struct_name {}
        }
    });

    // Define the State trait
    let state_trait = quote! {
        #visibility trait State {}
    };

    // Combine everything into a single TokenStream
    let expanded = quote! {
        #state_trait
        #(#structs)*
    };

    // Done
    TokenStream::from(expanded)
}
