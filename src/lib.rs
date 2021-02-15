use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ExprArray, Result,
};

// Structure storing details about a matrix with the data to be parsed from the tokenstream
#[derive(Debug)]
struct MatrixInput {
    // Token Stream of array
    parsed_arr: proc_macro2::TokenStream,
    dimensions: (usize, usize),
}

impl Parse for MatrixInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let _array = ExprArray::parse(input)?;
        Ok(MatrixInput {
            parsed_arr: quote! {#[0; 10]},
            dimensions: (0, 0),
        })
    }
}

#[proc_macro]
pub fn matrix(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
