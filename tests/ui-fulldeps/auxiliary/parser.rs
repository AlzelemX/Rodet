#![feature(rustc_private)]

extern crate rodetc_ast;
extern crate rodetc_driver;
extern crate rodetc_errors;
extern crate rodetc_parse;
extern crate rodetc_session;
extern crate rodetc_span;

use rodetc_ast::ast::{AttrKind, Attribute, DUMMY_NODE_ID, Expr};
use rodetc_ast::mut_visit::{self, MutVisitor};
use rodetc_ast::node_id::NodeId;
use rodetc_ast::token;
use rodetc_ast::tokenstream::{AttrTokenStream, AttrTokenTree, LazyAttrTokenStream};
use rodetc_errors::Diag;
use rodetc_parse::parser::Recovery;
use rodetc_session::parse::ParseSess;
use rodetc_span::{AttrId, DUMMY_SP, FileName, Span};
use std::sync::Arc;

pub fn parse_expr(psess: &ParseSess, source_code: &str) -> Option<Box<Expr>> {
    let parser = rodetc_parse::unwrap_or_emit_fatal(rodetc_parse::new_parser_from_source_str(
        psess,
        FileName::anon_source_code(source_code),
        source_code.to_owned(),
        rodetc_parse::lexer::StripTokens::Nothing,
    ));

    let mut parser = parser.recovery(Recovery::Forbidden);
    let mut expr = parser.parse_expr().map_err(Diag::cancel).ok()?;
    if parser.token != token::Eof {
        return None;
    }

    Normalize.visit_expr(&mut expr);
    Some(expr)
}

// Erase Span information that could distinguish between identical expressions
// parsed from different source strings.
struct Normalize;

impl MutVisitor for Normalize {
    fn visit_id(&mut self, id: &mut NodeId) {
        *id = DUMMY_NODE_ID;
    }

    fn visit_span(&mut self, span: &mut Span) {
        *span = DUMMY_SP;
    }

    fn visit_attribute(&mut self, attr: &mut Attribute) {
        attr.id = AttrId::from_u32(0);
        if let AttrKind::Normal(normal_attr) = &mut attr.kind {
            if let Some(tokens) = &mut normal_attr.tokens {
                let mut stream = tokens.to_attr_token_stream();
                normalize_attr_token_stream(&mut stream);
                *tokens = LazyAttrTokenStream::new_direct(stream);
            }
        }
        mut_visit::walk_attribute(self, attr);
    }
}

fn normalize_attr_token_stream(stream: &mut AttrTokenStream) {
    Arc::make_mut(&mut stream.0)
        .iter_mut()
        .for_each(normalize_attr_token_tree);
}

fn normalize_attr_token_tree(token: &mut AttrTokenTree) {
    match token {
        AttrTokenTree::Token(token, _spacing) => {
            Normalize.visit_span(&mut token.span);
        }
        AttrTokenTree::Delimited(dspan, _spacing, _delim, stream) => {
            normalize_attr_token_stream(stream);
            Normalize.visit_span(&mut dspan.open);
            Normalize.visit_span(&mut dspan.close);
        }
        AttrTokenTree::AttrsTarget(_) => unimplemented!("AttrTokenTree::AttrsTarget"),
    }
}
