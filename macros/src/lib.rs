#[cfg(test)]
mod test;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;

#[proc_macro_derive(Lex, attributes(regex))]
pub fn derive_lex(item: TokenStream) -> TokenStream {
    derive_lex_entry(item.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn derive_lex_entry(item: TokenStream2) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = syn::parse2(item)?;
    let syn::Data::Enum(ast) = ast.data else {
            return Err(syn::Error::new_spanned(
                ast,
                "union and strict are not supported",
            ))
        };


    todo!()
}

#[cfg(test)]
mod tests {
    use crate::derive_lex_entry;

    #[test]
    fn test() {
        assert!(derive_lex_entry(r#"enum A {#[regex("let")]LET,}"#.parse().unwrap()).is_ok());
    }

    #[test]
    fn union() {
        assert!(derive_lex_entry("union A{}".parse().unwrap()).is_err())
    }
    #[test]
    fn strict() {
        assert!(derive_lex_entry("strict A{}".parse().unwrap()).is_err())
    }
}
