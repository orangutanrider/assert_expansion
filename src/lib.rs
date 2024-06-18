use proc_macro::*;

#[proc_macro]
pub fn assert_expansion(input: TokenStream) -> TokenStream {
    todo!()
}

// Get two groups
// Expand macros within the first group
// Get the expansion as a token stream
// Get the second group as a token stream
// Check if both groups equal eachother once converted to strings
// Create a panic!() if they do not