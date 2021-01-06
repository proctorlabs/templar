use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// Intended for attribute values, format=> value = "expression"
pub(crate) struct BindingAttrValues {
    pub ident: syn::Ident,
    pub equal: Token![=],
    pub val: syn::Expr,
}

impl Parse for BindingAttrValues {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BindingAttrValues { ident: input.parse()?, equal: input.parse()?, val: input.parse()? })
    }
}

// A group of BindingAttrValues in parenthesis=> (value = item, value2 = item)
pub(crate) struct BindingAttrParens {
    pub parens: syn::token::Paren,
    pub contents: Punctuated<BindingAttrValues, Token![,]>,
}

impl Parse for BindingAttrParens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let parens = parenthesized!(content in input);
        let contents = content.parse_terminated(BindingAttrValues::parse)?;
        Ok(BindingAttrParens { parens, contents })
    }
}
