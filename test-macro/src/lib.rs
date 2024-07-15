#[proc_macro_derive(Foo, attributes(foo))]
pub fn derive(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}
