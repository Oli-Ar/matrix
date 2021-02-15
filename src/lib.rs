use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, ExprArray, Result,
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
        let array = ExprArray::parse(input)?;
        let dimensions = check_array_length(&array)?;
        Ok(MatrixInput {
            parsed_arr: quote! {#[0; 10]},
            dimensions,
        })
    }
}

// Ensures each array within the passed in 2d arrays are the same lenght then returns the
// dimensions of the matrix
fn check_array_length(array: &ExprArray) -> Result<(usize, usize)> {
    // Takes in pointer to an expression and returns the ExprArray struct that it should be, if
    // it's not the expected ExprArray struct errors
    fn unwrap_array(expr: &Expr) -> Result<ExprArray> {
        if let Expr::Array(array) = expr {
            return Ok(array.clone());
        }
        Err(Error::new(expr.span(), "Expected 2d array"))
    }

    // Iterates through the passsed in array comparing the current element to the first sub arrays
    // length
    let base_length = (unwrap_array(&array.elems[0])?).elems.len();
    for i in 1..(&array).elems.len() {
        let cur_elem = unwrap_array(&array.elems[i])?;
        if cur_elem.elems.len() != base_length {
            return Err(Error::new(
                array.elems[i].span(),
                "All of the arrays within the matrix must be the same length.",
            ));
        }
    }
    if let Expr::Array(sub_array) = &array.elems[0] {
        return Ok((array.elems.len(), sub_array.elems.len()));
    }
    Err(Error::new(array.span(), "Expected a 2d array"))
}

#[proc_macro]
pub fn matrix(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
