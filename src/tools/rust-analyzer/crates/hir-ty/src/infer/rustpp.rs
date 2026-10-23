//! Rust++: named arguments (`f(x = 1, y = 2)`) and parameter default values
//! (`fn f(x: i32 = 0)`) in calls.
//!
//! Body lowering keeps only the values of named arguments in `args` and
//! records their names (see `ExpressionStore::rustpp_named_args`). Here the
//! arguments are put back in parameter order, and parameters left out
//! because they have a default value are dropped from the expected inputs.
//! The rules are the compiler's (`rustc_hir::rustpp::order_named_args`):
//! positional arguments first, each parameter given once, every parameter
//! given unless it has a default value.

use hir_def::{FunctionId, hir::ExprId, signatures::FunctionSignature};
use hir_expand::name::Name;

use crate::{
    infer::{InferenceContext, expr::ExprIsRead},
    next_solver::Ty,
};

/// The arguments of a call in parameter order.
pub(super) struct OrderedArgs<'db> {
    /// The argument expressions, in the order of the parameters they give.
    pub(super) args: Vec<ExprId>,
    /// The input types of the parameters that are given.
    pub(super) input_tys: Vec<Ty<'db>>,
}

impl<'db> InferenceContext<'db> {
    /// Puts the arguments of `call_expr` in parameter order, for a call to
    /// `callee` (`None` if it is not a function, e.g. a closure).
    /// `input_tys` are the callee's input types without the receiver of a
    /// method call (`is_method`).
    ///
    /// Returns `None` when the call needs nothing special (no named
    /// arguments, no left-out parameters). When the arguments don't fit the
    /// parameters, the values are inferred on their own and an empty call is
    /// returned: the compiler reports the precise error through `cargo check`.
    pub(super) fn rustpp_order_args(
        &mut self,
        call_expr: ExprId,
        callee: Option<FunctionId>,
        args: &[ExprId],
        input_tys: &[Ty<'db>],
        is_method: bool,
    ) -> Option<OrderedArgs<'db>> {
        let names = self.store.rustpp_named_args(call_expr).map(|names| names.to_vec());
        if names.is_none() && args.len() >= input_tys.len() {
            return None;
        }

        let Some(callee) = callee else {
            if names.is_none() {
                return None;
            }
            return Some(self.rustpp_give_up(args));
        };
        let sig = FunctionSignature::of(self.db, callee);
        let skip = usize::from(is_method && sig.has_self_param());
        let params = sig.param_names.get(skip..).unwrap_or_default();
        let defaults = sig.param_defaults.get(skip..).unwrap_or_default();
        if params.len() != input_tys.len() {
            // Variadic functions and other unusual signatures.
            return if names.is_none() { None } else { Some(self.rustpp_give_up(args)) };
        }
        if names.is_none() && !defaults.iter().any(Option::is_some) {
            // Too few arguments and nothing to fill them: the usual error.
            return None;
        }

        match order_args(params, defaults, args, names.as_deref()) {
            Some(slots) => {
                let mut ordered = OrderedArgs { args: Vec::new(), input_tys: Vec::new() };
                for (slot, &ty) in slots.into_iter().zip(input_tys) {
                    if let Some(arg) = slot {
                        ordered.args.push(arg);
                        ordered.input_tys.push(ty);
                    }
                }
                Some(ordered)
            }
            None => Some(self.rustpp_give_up(args)),
        }
    }

    fn rustpp_give_up(&mut self, args: &[ExprId]) -> OrderedArgs<'db> {
        for &arg in args {
            self.infer_expr_no_expect(arg, ExprIsRead::Yes);
        }
        OrderedArgs { args: Vec::new(), input_tys: Vec::new() }
    }
}

/// One slot per parameter: the argument given for it, or `None` for a
/// left-out parameter with a default value. `None` if the arguments don't fit.
fn order_args(
    params: &[Option<Name>],
    defaults: &[Option<Box<str>>],
    args: &[ExprId],
    names: Option<&[Option<Name>]>,
) -> Option<Vec<Option<ExprId>>> {
    let mut slots = vec![None; params.len()];
    let mut seen_named = false;
    let mut next_positional = 0;
    for (index, &arg) in args.iter().enumerate() {
        match names.and_then(|names| names.get(index)).and_then(Option::as_ref) {
            None => {
                if seen_named {
                    return None;
                }
                *slots.get_mut(next_positional)? = Some(arg);
                next_positional += 1;
            }
            Some(name) => {
                seen_named = true;
                let param = params.iter().position(|param| param.as_ref() == Some(name))?;
                if slots[param].replace(arg).is_some() {
                    return None;
                }
            }
        }
    }
    for (slot, default) in slots.iter().zip(defaults) {
        if slot.is_none() && default.is_none() {
            return None;
        }
    }
    Some(slots)
}
