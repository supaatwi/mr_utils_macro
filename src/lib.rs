mod to_vec;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToVec, attributes(to_vec))]
pub fn to_vec_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Call implementation from to_vec module
    to_vec::impl_to_vec(&input)
}

