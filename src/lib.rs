mod parse;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ExprArray, Result,
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
        let dimensions = parse::check_array_length(&array)?;
        let parsed_arr = parse::check_valid_input(&array)?;
        Ok(MatrixInput {
            parsed_arr,
            dimensions,
        })
    }
}

#[proc_macro]
pub fn matrix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MatrixInput);
    let output_len = input.dimensions.0 * input.dimensions.1;
    let output_arr = input.parsed_arr;

    let x_dim = input.dimensions.1;
    let y_dim = input.dimensions.0;

    let matrix = Ident::new(&format!("Matrix{}x{}", x_dim, y_dim), Span::call_site());

    TokenStream::from(quote! {{
        use std::fmt::{Display, Formatter, Result};
        #[derive(Debug, Eq, PartialEq)]
        pub struct #matrix([i32; #output_len]);

        // Prints the user inputted array in the same dimensions as the passed in
        impl Display for #matrix {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                for i in 0..#y_dim {
                    if i == 0 { write!(f, "[")? } else { write!(f, " ")? };
                    for j in 0..#x_dim {
                        write!(f, "{:>3},", self.0[#x_dim*i+j])?;
                    }
                    if i == #y_dim-1 { write!(f, "]")? } else { write!(f, "\n")? };
                }
                Ok(())
            }
        }

        impl<U> std::ops::Index<[U; 2]> for #matrix
        where
            U: std::convert::Into<usize> + std::marker::Copy,
            usize: std::ops::Mul<U, Output = usize> + std::ops::Add<U, Output = usize>,
        {
            // TODO: This has to be made more generic if the rest of the macro is made more generic
            type Output = i32;

            fn index(&self, idx: [U; 2]) -> &Self::Output {
                let index = #x_dim * idx[1] + idx[0];
                &self.0[index]
            }
        }

        #matrix(#output_arr)
    }})
}
