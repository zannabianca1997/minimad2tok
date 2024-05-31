use minimad::{Alignment, Composite, CompositeStyle, Compound, Line, TableRow, TableRule, Text};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// reexports for common macro usage
pub use minimad;

/**
An adapter to implement `ToTokens` on types defined in the library `minimad`.
The emitted tokens will represent a `'static` version of the text
*/
pub struct Emitter<'s, T>(pub &'s T);

impl ToTokens for Emitter<'_, Text<'_>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(Text { lines }) = self;
        let lines = lines.iter().map(Emitter);
        quote! {
            ::minimad2tok::minimad::Text {
                lines: ::std::vec![
                    #(#lines),*
                ]
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, Line<'_>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(line) = self;
        match line {
            Line::Normal(content) => {
                let content = Emitter(content);
                quote! { ::minimad2tok::minimad::Line::Normal(#content) }
            }
            Line::TableRow(content) => {
                let content = Emitter(content);
                quote! { ::minimad2tok::minimad::Line::TableRow(#content) }
            }
            Line::TableRule(content) => {
                let content = Emitter(content);
                quote! { ::minimad2tok::minimad::Line::TableRule(#content) }
            }
            Line::HorizontalRule => {
                quote! { ::minimad2tok::minimad::Line::HorizontalRule }
            }
            Line::CodeFence(content) => {
                let content = Emitter(content);
                quote! { ::minimad2tok::minimad::Line::CodeFence(#content) }
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, Composite<'_>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(Composite { style, compounds }) = self;
        let style = Emitter(style);
        let compounds = compounds.iter().map(Emitter);
        quote! {
            ::minimad2tok::minimad::Composite {
                style: #style,
                compounds: ::std::vec![
                    #(#compounds),*
                ]
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, CompositeStyle> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(style) = self;
        match style {
            CompositeStyle::Paragraph => {
                quote! { ::minimad2tok::minimad::CompositeStyle::Paragraph }
            }
            CompositeStyle::Header(n) => {
                quote! { ::minimad2tok::minimad::CompositeStyle::Header(#n) }
            }
            CompositeStyle::ListItem(n) => {
                quote! { ::minimad2tok::minimad::CompositeStyle::ListItem(#n) }
            }
            CompositeStyle::Code => {
                quote! { ::minimad2tok::minimad::CompositeStyle::Code }
            }
            CompositeStyle::Quote => {
                quote! { ::minimad2tok::minimad::CompositeStyle::Quote }
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, Compound<'_>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(Compound {
            src,
            bold,
            italic,
            code,
            strikeout,
        }) = self;
        quote! {
            ::minimad2tok::minimad::Compound {
                src: #src,
                bold: #bold,
                italic: #italic,
                code: #code,
                strikeout: #strikeout
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, TableRow<'_>> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(TableRow { cells }) = self;
        let cells = cells.iter().map(Emitter);
        quote! {
            ::minimad2tok::minimad::TableRow {
                cells: ::std::vec![
                    #(#cells),*
                ]
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, TableRule> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(TableRule { cells }) = self;
        let cells = cells.iter().map(Emitter);
        quote! {
            ::minimad2tok::minimad::TableRule {
                cells: ::std::vec![
                    #(#cells),*
                ]
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for Emitter<'_, Alignment> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(alignement) = self;
        match alignement {
            Alignment::Unspecified => quote! {
                ::minimad2tok::minimad::Alignment::Unspecified
            },
            Alignment::Left => quote! {
                ::minimad2tok::minimad::Alignment::Left
            },
            Alignment::Center => quote! {
                ::minimad2tok::minimad::Alignment::Center
            },
            Alignment::Right => quote! {
                ::minimad2tok::minimad::Alignment::Right
            },
        }
        .to_tokens(tokens)
    }
}
