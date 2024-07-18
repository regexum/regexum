use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Token};
use syn::{LitStr, Variant};

#[derive(Debug, Clone)]
pub struct Set {}

impl Parse for Set {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pattern: String,
}

impl Pattern {
    pub fn pattern(&self) -> &str {
        return &self.pattern;
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pattern: LitStr = input.parse()?;
        Ok(Self {
            pattern: pattern.value(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Attr {
    #[allow(dead_code)]
    Set(Set),
    Pattern(Pattern),
}

pub trait HasAttrs {
    fn paths(&self) -> &'static [&'static str] {
        &[]
    }
    fn attrs(&self) -> &[Attribute];
    fn parse_attributes(&self) -> syn::Result<Vec<Attr>> {
        let mut result = Vec::new();
        let attrs = self
            .attrs()
            .as_ref()
            .iter()
            .filter(|a| self.paths().iter().any(|p| a.path().is_ident(*p)));
        for attr in attrs {
            if attr.path().is_ident("pattern") {
                let punctuated =
                    attr.parse_args_with(Punctuated::<Pattern, Token![,]>::parse_terminated)?;
                for parsed in punctuated {
                    result.push(Attr::Pattern(parsed));
                }
            }
        }
        Ok(result)
    }
}

impl HasAttrs for Variant {
    fn paths(&self) -> &'static [&'static str] {
        return &["pattern"];
    }
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}
