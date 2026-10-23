//! Rust++: type checking of named arguments, `f(x = 1, y = 2)`.
//! See `rustc_hir::rustpp` for how they are represented.

use rustc_errors::ErrorGuaranteed;
use rustc_hir as hir;
use rustc_hir::def::DefKind;
use rustc_hir::rustpp::{NamedArgsError, has_named_args, order_named_args, strip_named_args};
use rustc_macros::Diagnostic;
use rustc_span::{Ident, Span};

use crate::FnCtxt;
use crate::callee::SplatLoweringInfo;

#[derive(Diagnostic)]
#[diag("named arguments can only be used when calling a function or method directly")]
struct NamedArgsUnsupported {
    #[primary_span]
    span: Span,
}

#[derive(Diagnostic)]
#[diag("positional argument after a named argument")]
#[help("put positional arguments first, or name this one too")]
struct PositionalAfterNamed {
    #[primary_span]
    span: Span,
}

#[derive(Diagnostic)]
#[diag("this function takes fewer arguments than were given")]
struct TooManyPositional {
    #[primary_span]
    #[label("unexpected argument")]
    span: Span,
}

#[derive(Diagnostic)]
#[diag("this function has no parameter named `{$name}`")]
struct UnknownNamedArg {
    #[primary_span]
    #[label("unknown parameter")]
    span: Span,
    name: Ident,
}

#[derive(Diagnostic)]
#[diag("parameter `{$name}` is given more than once")]
struct NamedArgAlreadyGiven {
    #[primary_span]
    span: Span,
    name: Ident,
}

#[derive(Diagnostic)]
#[diag("missing argument for parameter `{$name}`")]
struct MissingNamedArg {
    #[primary_span]
    #[label("add `{$name} = ...`")]
    span: Span,
    name: String,
}

impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
    /// If the call uses named arguments, returns the argument expressions in
    /// parameter order (the values of named arguments); otherwise returns
    /// `args` unchanged. On a mistake, reports it, type-checks the values on
    /// their own, and returns the error: the caller should skip the call.
    pub(super) fn order_named_args(
        &self,
        call_expr: &'tcx hir::Expr<'tcx>,
        args: &'tcx [hir::Expr<'tcx>],
        n_inputs: usize,
        fn_id: &SplatLoweringInfo<'tcx>,
    ) -> Result<&'tcx [hir::Expr<'tcx>], ErrorGuaranteed> {
        if !has_named_args(args) {
            return Ok(args);
        }
        let tcx = self.tcx;

        // The `x = value` nodes themselves are never evaluated; give them (and
        // the unresolved `x`) a type so later passes see a fully typed body.
        for arg in args {
            if let hir::ExprKind::Assign(name, _, _) = arg.kind
                && hir::rustpp::named_arg(arg).is_some()
            {
                self.write_ty(arg.hir_id, tcx.types.unit);
                self.write_ty(name.hir_id, tcx.types.unit);
            }
        }

        let alloc = |exprs: Vec<&'tcx hir::Expr<'tcx>>| -> &'tcx [hir::Expr<'tcx>] {
            tcx.arena.dropless.alloc_from_iter(exprs.into_iter().copied())
        };

        let def_id = match *fn_id {
            SplatLoweringInfo::FnDef(def_id)
                if matches!(tcx.def_kind(def_id), DefKind::Fn | DefKind::AssocFn) =>
            {
                def_id
            }
            SplatLoweringInfo::Error(_) => return Ok(alloc(strip_named_args(args))),
            _ => {
                let guar = self.dcx().emit_err(NamedArgsUnsupported { span: call_expr.span });
                return Err(self.check_values_after_error(args, guar));
            }
        };

        // For method calls the receiver is not in `args`: drop the `self` name.
        let names = tcx.fn_arg_idents(def_id);
        let names = &names[names.len().saturating_sub(n_inputs)..];

        match order_named_args(names, args) {
            Ok(ordered) => Ok(alloc(ordered)),
            Err(err) => {
                let guar = match err {
                    NamedArgsError::PositionalAfterNamed { span } => {
                        self.dcx().emit_err(PositionalAfterNamed { span })
                    }
                    NamedArgsError::TooManyPositional { span } => {
                        self.dcx().emit_err(TooManyPositional { span })
                    }
                    NamedArgsError::UnknownName { name } => {
                        self.dcx().emit_err(UnknownNamedArg { span: name.span, name })
                    }
                    NamedArgsError::AlreadyGiven { name } => {
                        self.dcx().emit_err(NamedArgAlreadyGiven { span: name.span, name })
                    }
                    NamedArgsError::Missing { name, index } => {
                        let name = name.map_or_else(|| format!("#{}", index + 1), |n| n.to_string());
                        let span = tcx.sess.source_map().end_point(call_expr.span);
                        self.dcx().emit_err(MissingNamedArg { span, name })
                    }
                };
                Err(self.check_values_after_error(args, guar))
            }
        }
    }

    /// Type-checks the argument values on their own, so their own errors are
    /// still reported and every expression gets a type.
    fn check_values_after_error(
        &self,
        args: &'tcx [hir::Expr<'tcx>],
        guar: ErrorGuaranteed,
    ) -> ErrorGuaranteed {
        self.set_tainted_by_errors(guar);
        for value in strip_named_args(args) {
            self.check_expr(value);
        }
        guar
    }
}
