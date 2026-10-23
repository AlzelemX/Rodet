//! Rust++: "return value" inlay hints, a mark after the value a function
//! gives back without `return`:
//! ```no_run
//! fn square(n: i32) -> i32 {
//!     n * n /* ↩ */
//! }
//! ```
//! Inside `if`, `match` and blocks every branch that gives the value is
//! marked.
use syntax::ast::{self, AstNode};

use crate::{
    InlayHint, InlayHintLabel, InlayHintPosition, InlayHintsConfig, InlayKind, InlayTooltip,
    inlay_hints::LazyProperty,
};

/// The mark shown after a returned value.
pub(super) const RETURN_VALUE_MARK: &str = "↩";

pub(super) fn hints(
    acc: &mut Vec<InlayHint>,
    config: &InlayHintsConfig<'_>,
    func: &ast::Fn,
) -> Option<()> {
    if !config.return_value_hints {
        return None;
    }
    // Functions without `-> T` give back `()`: nothing to mark.
    func.ret_type()?.ty()?;
    let tail = func.body()?.tail_expr()?;
    let mut values = Vec::new();
    returned_values(tail, &mut values);
    for value in values {
        acc.push(InlayHint {
            range: value.syntax().text_range(),
            kind: InlayKind::ReturnValue,
            label: InlayHintLabel::simple(
                RETURN_VALUE_MARK,
                Some(LazyProperty::Computed(InlayTooltip::String(
                    "returned value: the last expression of the function, without `;`".to_owned(),
                ))),
                None,
            ),
            text_edit: None,
            position: InlayHintPosition::After,
            pad_left: true,
            pad_right: false,
            resolve_parent: None,
        });
    }
    Some(())
}

/// The expressions that give the value of `expr`: `expr` itself, or for
/// `if`, `match` and blocks, the value of each branch.
fn returned_values(expr: ast::Expr, acc: &mut Vec<ast::Expr>) {
    match expr {
        ast::Expr::IfExpr(it) => {
            if let Some(then) = it.then_branch() {
                block_value(then, acc);
            }
            match it.else_branch() {
                Some(ast::ElseBranch::Block(block)) => block_value(block, acc),
                Some(ast::ElseBranch::IfExpr(elif)) => returned_values(elif.into(), acc),
                None => {}
            }
        }
        ast::Expr::MatchExpr(it) => {
            for arm in it.match_arm_list().into_iter().flat_map(|list| list.arms()) {
                if let Some(value) = arm.expr() {
                    returned_values(value, acc);
                }
            }
        }
        ast::Expr::BlockExpr(block)
            if block.label().is_none()
                && block.async_token().is_none()
                && block.const_token().is_none()
                && block.try_block_modifier().is_none() =>
        {
            block_value(block, acc)
        }
        // `return`, `break`, `continue`, `panic!()`... give no value here.
        ast::Expr::ReturnExpr(_) | ast::Expr::BreakExpr(_) | ast::Expr::ContinueExpr(_) => {}
        other => acc.push(other),
    }
}

fn block_value(block: ast::BlockExpr, acc: &mut Vec<ast::Expr>) {
    if let Some(tail) = block.tail_expr() {
        returned_values(tail, acc);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        InlayHintsConfig,
        inlay_hints::tests::{DISABLED_CONFIG, check_with_config},
    };

    #[track_caller]
    fn check(#[rust_analyzer::rust_fixture] ra_fixture: &str) {
        check_with_config(InlayHintsConfig { return_value_hints: true, ..DISABLED_CONFIG }, ra_fixture);
    }

    #[test]
    fn marks_returned_values() {
        check(
            r#"
fn square(n: i32) -> i32 {
    n * n
  //^^^^^ ↩
}

fn sign(n: i32) -> &'static str {
    if n < 0 {
        "-"
      //^^^ ↩
    } else if n == 0 {
        "0"
      //^^^ ↩
    } else {
        match n {
            1 => "one",
               //^^^^^ ↩
            _ => { "+" }
                 //^^^ ↩
        }
    }
}

fn unit() {
    square(1)
}

fn early(n: i32) -> i32 {
    if n > 0 { return n; }
    0
  //^ ↩
}
"#,
        );
    }
}
