extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn impl_async(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func_code = item.to_string();
    // println!("func_code: {func_code}");

    let func_code = func_code.replace("\n", " ");
    let func_code = func_code.replace("     ", " ");
    let func_code = func_code.replace("    ", " ");
    let func_code = func_code.replace("   ", " ");
    let func_code = func_code.replace("  ", " ");
    let func_code = func_code.replace("  ", " ");
    let func_code = func_code.replace("fn ", "async fn ");
    let func_code = func_code.replace(": std :: io :: Read >", ": tokio::io::AsyncRead + Send + Unpin>");
    let func_code = func_code.replace(".map(| await_ | await_)", ".await");

    println!("func_code: {func_code}");
    // panic!("{func_code}");
    TokenStream::from_str(&func_code).unwrap()
}