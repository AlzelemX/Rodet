use rodetc_lint_defs::LintPass;

use crate::context::{EarlyContext, LateContext};

#[macro_export]
macro_rules! late_lint_methods {
    ($macro:path, $args:tt) => (
        // `_post` methods are called *after* recursing into the node.
        $macro!($args, [
            fn check_body(a: &rodetc_hir::Body<'tcx>);
            fn check_body_post(a: &rodetc_hir::Body<'tcx>);
            fn check_crate();
            fn check_crate_post();
            fn check_mod(a: &'tcx rodetc_hir::Mod<'tcx>, b: rodetc_span::def_id::LocalModId);
            fn check_foreign_item(a: &'tcx rodetc_hir::ForeignItem<'tcx>);
            fn check_item(a: &'tcx rodetc_hir::Item<'tcx>);
            fn check_item_post(a: &'tcx rodetc_hir::Item<'tcx>);
            fn check_local(a: &'tcx rodetc_hir::LetStmt<'tcx>);
            fn check_block(a: &'tcx rodetc_hir::Block<'tcx>);
            fn check_block_post(a: &'tcx rodetc_hir::Block<'tcx>);
            fn check_stmt(a: &'tcx rodetc_hir::Stmt<'tcx>);
            fn check_arm(a: &'tcx rodetc_hir::Arm<'tcx>);
            fn check_pat(a: &'tcx rodetc_hir::Pat<'tcx>);
            fn check_lit(hir_id: rodetc_hir::HirId, a: rodetc_hir::Lit, is_negated_pat: bool);
            fn check_expr(a: &'tcx rodetc_hir::Expr<'tcx>);
            fn check_expr_post(a: &'tcx rodetc_hir::Expr<'tcx>);
            fn check_ty(a: &'tcx rodetc_hir::Ty<'tcx, rodetc_hir::AmbigArg>);
            fn check_generic_param(a: &'tcx rodetc_hir::GenericParam<'tcx>);
            fn check_generics(a: &'tcx rodetc_hir::Generics<'tcx>);
            fn check_poly_trait_ref(a: &'tcx rodetc_hir::PolyTraitRef<'tcx>);
            fn check_fn(
                a: rodetc_hir::intravisit::FnKind<'tcx>,
                b: &'tcx rodetc_hir::FnDecl<'tcx>,
                c: &'tcx rodetc_hir::Body<'tcx>,
                d: rodetc_span::Span,
                e: rodetc_span::def_id::LocalDefId);
            fn check_trait_item(a: &'tcx rodetc_hir::TraitItem<'tcx>);
            fn check_impl_item(a: &'tcx rodetc_hir::ImplItem<'tcx>);
            fn check_impl_item_post(a: &'tcx rodetc_hir::ImplItem<'tcx>);
            fn check_field_def(a: &'tcx rodetc_hir::FieldDef<'tcx>);
            fn check_variant(a: &'tcx rodetc_hir::Variant<'tcx>);
            fn check_path(a: &rodetc_hir::Path<'tcx>, b: rodetc_hir::HirId);
            fn check_attribute(a: &'tcx rodetc_attr_ir::Attribute);
            fn check_attributes(a: &'tcx [rodetc_attr_ir::Attribute]);
            fn check_attributes_post(a: &'tcx [rodetc_attr_ir::Attribute]);
        ]);
    )
}

/// Trait for types providing lint checks.
///
/// Each `check` method checks a single syntax node, and should not
/// invoke methods recursively (unlike `Visitor`). By default they
/// do nothing.
///
// FIXME: eliminate the duplication with `Visitor`. But this also
// contains a few lint-specific methods with no equivalent in `Visitor`.
//
macro_rules! declare_late_lint_pass {
    ([], [$(fn $name:ident($($param:ident: $arg:ty),*);)*]) => (
        pub trait LateLintPass<'tcx>: LintPass {
            $(#[inline(always)] fn $name(&mut self, _: &LateContext<'tcx>, $(_: $arg),*) {})*
        }
    )
}

// Declare the `LateLintPass` trait, which contains empty default definitions
// for all the `check_*` methods.
late_lint_methods!(declare_late_lint_pass, []);

#[macro_export]
macro_rules! expand_combined_late_lint_pass_method {
    ([$($pass:ident),*], $self: ident, $name: ident, $params:tt) => ({
        $($self.$pass.$name $params;)*
    })
}

#[macro_export]
macro_rules! expand_combined_late_lint_pass_methods {
    ($passes:tt, [$(fn $name:ident($($param:ident: $arg:ty),*);)*]) => (
        $(fn $name(&mut self, context: &$crate::LateContext<'tcx>, $($param: $arg),*) {
            $crate::expand_combined_late_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
    )
}

/// Combines multiple lints passes into a single lint pass, at compile time,
/// for maximum speed. Each `check_foo` method in `$methods` within this pass
/// simply calls `check_foo` once per `$pass`. Compare with
/// `RuntimeCombinedLateLintPass`, which is similar, but combines lint passes at
/// runtime.
#[macro_export]
macro_rules! declare_combined_late_lint_pass {
    ([$v:vis $name:ident, [$($pass:ident: $constructor:expr,)*]], $methods:tt) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($pass: $pass,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($pass: $constructor,)*
                }
            }

            $v fn lint_vec() -> $crate::LintVec {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$pass::lint_vec());)*
                lints
            }
        }

        impl<'tcx> $crate::LateLintPass<'tcx> for $name {
            $crate::expand_combined_late_lint_pass_methods!([$($pass),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl $crate::LintPass for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }
            fn get_lints(&self) -> LintVec {
                $name::lint_vec()
            }
        }
    )
}

#[macro_export]
macro_rules! early_lint_methods {
    ($macro:path, $args:tt) => (
        // `_post` methods are called *after* recursing into the node.
        $macro!($args, [
            fn check_param(a: &rodetc_ast::Param);
            fn check_ident(a: &rodetc_span::Ident);
            fn check_crate(a: &rodetc_ast::Crate);
            fn check_crate_post(a: &rodetc_ast::Crate);
            fn check_item(a: &rodetc_ast::Item);
            fn check_item_post(a: &rodetc_ast::Item);
            fn check_local(a: &rodetc_ast::Local);
            fn check_block(a: &rodetc_ast::Block);
            fn check_stmt(a: &rodetc_ast::Stmt);
            fn check_arm(a: &rodetc_ast::Arm);
            fn check_pat(a: &rodetc_ast::Pat);
            fn check_pat_post(a: &rodetc_ast::Pat);
            fn check_expr(a: &rodetc_ast::Expr);
            fn check_expr_post(a: &rodetc_ast::Expr);
            fn check_ty(a: &rodetc_ast::Ty);
            fn check_generic_arg(a: &rodetc_ast::GenericArg);
            fn check_generic_param(a: &rodetc_ast::GenericParam);
            fn check_generics(a: &rodetc_ast::Generics);
            fn check_poly_trait_ref(a: &rodetc_ast::PolyTraitRef);
            fn check_fn(
                a: rodetc_ast::visit::FnKind<'_>,
                c: rodetc_span::Span,
                d_: rodetc_ast::NodeId);
            fn check_trait_item(a: &rodetc_ast::AssocItem);
            fn check_trait_item_post(a: &rodetc_ast::AssocItem);
            fn check_impl_item(a: &rodetc_ast::AssocItem);
            fn check_impl_item_post(a: &rodetc_ast::AssocItem);
            fn check_variant(a: &rodetc_ast::Variant);
            fn check_attribute(a: &rodetc_ast::Attribute);
            fn check_attributes(a: &[rodetc_ast::Attribute]);
            fn check_attributes_post(a: &[rodetc_ast::Attribute]);
            fn check_mac_def(a: &rodetc_ast::MacroDef);
            fn check_mac(a: &rodetc_ast::MacCall);
            fn check_where_predicate(a: &rodetc_ast::WherePredicate);
            fn check_where_predicate_post(a: &rodetc_ast::WherePredicate);
        ]);
    )
}

macro_rules! declare_early_lint_pass {
    ([], [$(fn $name:ident($($param:ident: $arg:ty),*);)*]) => (
        pub trait EarlyLintPass: LintPass {
            $(#[inline(always)] fn $name(&mut self, _: &EarlyContext<'_>, $(_: $arg),*) {})*
        }
    )
}

// Declare the `EarlyLintPass` trait, which contains empty default definitions
// for all the `check_*` methods.
early_lint_methods!(declare_early_lint_pass, []);

#[macro_export]
macro_rules! expand_combined_early_lint_pass_method {
    ([$($pass:ident),*], $self: ident, $name: ident, $params:tt) => ({
        $($self.$pass.$name $params;)*
    })
}

#[macro_export]
macro_rules! expand_combined_early_lint_pass_methods {
    ($passes:tt, [$(fn $name:ident($($param:ident: $arg:ty),*);)*]) => (
        $(fn $name(&mut self, context: &$crate::EarlyContext<'_>, $($param: $arg),*) {
            $crate::expand_combined_early_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
    )
}

/// Combines multiple lints passes into a single lint pass, at compile time,
/// for maximum speed. Each `check_foo` method in `$methods` within this pass
/// simply calls `check_foo` once per `$pass`. Compare with
/// `RuntimeCombinedEarlyLintPass`, which is similar, but combines lint passes at
/// runtime.
#[macro_export]
macro_rules! declare_combined_early_lint_pass {
    ([$v:vis $name:ident, [$($pass:ident: $constructor:expr,)*]], $methods:tt) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($pass: $pass,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($pass: $constructor,)*
                }
            }

            $v fn lint_vec() -> $crate::LintVec {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$pass::lint_vec());)*
                lints
            }
        }

        impl $crate::EarlyLintPass for $name {
            $crate::expand_combined_early_lint_pass_methods!([$($pass),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl $crate::LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
            fn get_lints(&self) -> LintVec {
                panic!()
            }
        }
    )
}

/// A lint pass boxed up as a trait object.
pub(crate) type EarlyLintPassObject = Box<dyn EarlyLintPass>;
pub(crate) type LateLintPassObject<'tcx> = Box<dyn LateLintPass<'tcx> + 'tcx>;
