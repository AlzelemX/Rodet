#![deny(unused_must_use)]

use proc_macro2::TokenStream;
use quote::quote;
use synstructure::Structure;

use crate::diagnostics::diagnostic_builder::each_variant;
use crate::diagnostics::error::DiagnosticDeriveError;

/// The central struct for constructing the `into_diag` method from an annotated struct.
pub(crate) struct DiagnosticDerive<'a> {
    structure: Structure<'a>,
}

impl<'a> DiagnosticDerive<'a> {
    pub(crate) fn new(structure: Structure<'a>) -> Self {
        Self { structure }
    }

    pub(crate) fn into_tokens(self) -> TokenStream {
        let DiagnosticDerive { mut structure } = self;
        let implementation = each_variant(&mut structure, |mut builder, variant| {
            let preamble = builder.preamble(variant);
            let body = builder.body(variant);

            let Some(message) = builder.primary_message() else {
                return DiagnosticDeriveError::ErrorHandled.to_compile_error();
            };
            let message = message.diag_message();

            let init = quote! {
                let mut diag = rodetc_errors::Diag::new(
                    dcx,
                    level,
                    #message
                );
            };

            let formatting_init = &builder.formatting_init;
            quote! {
                #init
                #formatting_init
                #preamble
                #body
                diag
            }
        });

        // A lifetime of `'a` causes conflicts, but `_sess` is fine.
        structure.gen_impl(quote! {
            gen impl<'_sess> rodetc_errors::Diagnostic<'_sess> for @Self {
                #[track_caller]
                fn into_diag(
                    self,
                    dcx: rodetc_errors::DiagCtxtHandle<'_sess>,
                    level: rodetc_errors::Level
                ) -> rodetc_errors::Diag<'_sess> {
                    #implementation
                }
            }
        })
    }
}
