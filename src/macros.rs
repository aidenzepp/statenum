use crate::helper::*;

struct StatenumArgs {
    name: Option<Ident>,
}

impl Parse for StatenumArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<LitStr>().ok().map(|s| Ident::new(&s.value(), s.span()));
        Ok(StatenumArgs { name })
    }
}

pub fn _statenum(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as StatenumArgs);
    let name = args.name.unwrap_or_else(|| Ident::new("State", Span::call_site()));

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);

    // Get the visibility of the enum
    let visibility = &input.vis;

    // Ensure we're working with an enum
    let data = match input.data {
        Data::Enum(e) => e,
        _ => panic!("statenum can only be used with enums"),
    };

    // Generate a struct and State implementation for each variant
    let structs = data.variants.iter().map(|variant| {
        let struct_name = Ident::new(&format!("{}", variant.ident), variant.ident.span());
        quote! {
            #visibility struct #struct_name;
            impl #name for #struct_name {}
        }
    });

    // Define the State trait
    let state_trait = quote! {
        #visibility trait #name {}
    };

    // Combine everything into a single TokenStream
    let expanded = quote! {
        #state_trait
        #(#structs)*
    };

    // Done
    TokenStream::from(expanded)
}
