use proc_macro::TokenStream;

#[proc_macro]
pub fn matrix(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
