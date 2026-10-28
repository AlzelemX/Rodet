//! Rodet: type checking of named arguments, `f(x = 1, y = 2)`.
//! See `rodetc_hir::rodet` for how they are represented.

use std::iter;

use rodetc_errors::ErrorGuaranteed;
use rodetc_hir as hir;
use rodetc_hir::def::DefKind;
use rodetc_hir::rodet::{NamedArgsError, has_named_args, order_named_args, strip_named_args};
use rodetc_macros::Diagnostic;
use rodetc_middle::ty::Ty;
use rodetc_middle::ty::rodet::param_default;
use rodetc_span::{Ident, Span};

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
    /// Handles named arguments and parameter default values.
    ///
    /// Returns the argument expressions in parameter order (the values of
    /// named arguments) and, when some parameters are left out because they
    /// have default values, the input types of the parameters that remain.
    /// Calls without named arguments or left-out parameters are returned
    /// unchanged. On a mistake, reports it, type-checks the values on their
    /// own, and returns the error: the caller should skip the call.
    pub(super) fn order_named_args(
        &self,
        call_expr: &'tcx hir::Expr<'tcx>,
        args: &'tcx [hir::Expr<'tcx>],
        formal_input_tys: &[Ty<'tcx>],
        fn_id: &SplatLoweringInfo<'tcx>,
    ) -> Result<(&'tcx [hir::Expr<'tcx>], Option<Vec<Ty<'tcx>>>), ErrorGuaranteed> {
        let tcx = self.tcx;
        let named = has_named_args(args);
        let too_few = args.len() < formal_input_tys.len();
        if !named && !too_few {
            return Ok((args, None));
        }

        // The `x = value` nodes themselves are never evaluated; give them (and
        // the unresolved `x`) a type so later passes see a fully typed body.
        for arg in args {
            if let hir::ExprKind::Assign(name, _, _) = arg.kind
                && hir::rodet::named_arg(arg).is_some()
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
            // Not a call that can use defaults: the usual argument count error.
            _ if !named => return Ok((args, None)),
            SplatLoweringInfo::Error(_) => return Ok((alloc(strip_named_args(args)), None)),
            _ => {
                let guar = self.dcx().emit_err(NamedArgsUnsupported { span: call_expr.span });
                return Err(self.check_values_after_error(args, guar));
            }
        };

        // For method calls the receiver is not in `args`: drop the `self` name.
        let names = tcx.fn_arg_idents(def_id);
        let names = &names[names.len().saturating_sub(formal_input_tys.len())..];
        let has_default = |index: usize| {
            names[index].is_some_and(|name| param_default(tcx, def_id, name.name).is_some())
        };

        match order_named_args(names, args, has_default) {
            Ok(slots) => {
                let left_out = slots.iter().any(|slot| slot.is_none());
                let tys = left_out.then(|| {
                    iter::zip(&slots, formal_input_tys)
                        .filter(|(slot, _)| slot.is_some())
                        .map(|(_, &ty)| ty)
                        .collect()
                });
                Ok((alloc(slots.into_iter().flatten().collect()), tys))
            }
            // Without named arguments or default values, keep the usual
            // argument count error.
            Err(_) if !named && !(0..names.len()).any(has_default) => Ok((args, None)),
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
