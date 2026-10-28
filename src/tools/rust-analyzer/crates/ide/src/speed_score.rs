//! Rodet: the speed score of a file, from 0 to 100.
//!
//! A quick look at the code for patterns that are known to make programs
//! slower: copying or allocating inside loops, searching lists in loops,
//! printing in loops, taking `&String` instead of `&str`, and so on. Every
//! finding takes points off and says how to fix it. It reads the code only;
//! it doesn't run anything, so it finds likely problems, not measured ones.
use hir::Semantics;
use ide_db::{FileId, RootDatabase};
use syntax::{
    AstNode, SyntaxNode, TextRange,
    ast::{self, HasArgList, HasLoopBody, HasName},
};

/// The score of a file and the findings behind it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SpeedScore {
    /// 100 minus the penalties, never below 0.
    pub score: u32,
    pub findings: Vec<SpeedFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeedFinding {
    pub range: TextRange,
    /// Points taken off.
    pub penalty: u32,
    /// What is slow.
    pub message: String,
    /// How to make it faster.
    pub fix: String,
}

pub(crate) fn speed_score(db: &RootDatabase, file_id: FileId) -> SpeedScore {
    let sema = Semantics::new(db);
    let file = sema.parse_guess_edition(file_id);
    let mut findings = Vec::new();
    for node in file.syntax().descendants() {
        check_node(&sema, &node, &mut findings);
    }
    findings.sort_by_key(|finding| finding.range.start());
    let penalty: u32 = findings.iter().map(|finding| finding.penalty).sum();
    SpeedScore { score: 100u32.saturating_sub(penalty), findings }
}

fn check_node(sema: &Semantics<'_, RootDatabase>, node: &SyntaxNode, acc: &mut Vec<SpeedFinding>) {
    let mut add = |range: TextRange, penalty: u32, message: &str, fix: &str| {
        acc.push(SpeedFinding {
            range,
            penalty,
            message: message.to_owned(),
            fix: fix.to_owned(),
        })
    };

    if let Some(call) = ast::MethodCallExpr::cast(node.clone()) {
        let Some(name) = call.name_ref() else { return };
        let name = name.text();
        let range = call.syntax().text_range();
        let in_loop = in_loop(call.syntax());
        match &*name {
            "clone" if in_loop => add(
                range,
                4,
                "`clone()` inside a loop copies the value every time around",
                "borrow it (`&value`) or clone once before the loop",
            ),
            "to_string" | "to_owned" if in_loop => add(
                range,
                2,
                "a new String is allocated every time around the loop",
                "keep `&str` if you only read it, or build it once before the loop",
            ),
            "contains" if in_loop && receiver_is_list(sema, &call) => add(
                range,
                4,
                "`contains` on a list checks every element, inside a loop that is n × m steps",
                "put the items in a `HashSet` once, then `contains` is instant",
            ),
            "sort" | "sort_unstable" | "sort_by" | "sort_by_key" if in_loop => add(
                range,
                3,
                "sorting inside a loop sorts the whole list every time around",
                "sort once after the loop, or use a `BinaryHeap` / `BTreeSet`",
            ),
            "nth" if in_loop && receiver_is_method(&call, "chars") => add(
                range,
                3,
                "`chars().nth(i)` walks the text from the start every time",
                "loop over `chars()` directly, or collect into a `Vec<char>` once",
            ),
            "len" | "iter" | "into_iter" if receiver_is_collect(&call) => add(
                range,
                3,
                "`collect()` builds a new list only to use it once",
                "use `.count()`, or keep working with the iterator without `collect()`",
            ),
            "count" if receiver_is_method(&call, "iter") => add(
                range,
                1,
                "`iter().count()` walks every element",
                "use `.len()`, it is instant",
            ),
            "push" | "push_str" if in_loop && grows_new_collection(&call) => add(
                range,
                2,
                "the list grows inside a loop and is moved to bigger memory several times",
                "create it with `Vec::with_capacity(n)` / `String::with_capacity(n)`",
            ),
            _ => {}
        }
        return;
    }

    if let Some(call) = ast::CallExpr::cast(node.clone()) {
        if !in_loop(call.syntax()) {
            return;
        }
        let Some(ast::Expr::PathExpr(callee)) = call.expr() else { return };
        let Some(path) = callee.path() else { return };
        let text = path.syntax().text().to_string();
        let range = call.syntax().text_range();
        if text.ends_with("Regex::new") {
            add(
                range,
                5,
                "a regular expression is compiled every time around the loop",
                "compile it once before the loop (or in a `static` with `LazyLock`)",
            );
        } else if text == "String::from" {
            add(
                range,
                2,
                "a new String is allocated every time around the loop",
                "keep `&str` if you only read it, or build it once before the loop",
            );
        }
        return;
    }

    if let Some(mac) = ast::MacroCall::cast(node.clone()) {
        if !in_loop(mac.syntax()) {
            return;
        }
        let Some(name) = mac.path().and_then(|path| path.segment()).and_then(|s| s.name_ref())
        else {
            return;
        };
        let range = mac.syntax().text_range();
        match &*name.text() {
            "out" | "println" | "print" | "eprintln" | "eprint" => add(
                range,
                2,
                "printing inside a loop locks the terminal every time around",
                "lock it once (`let mut o = std::io::stdout().lock();` then `writeln!(o, ...)`), or collect the text and print once",
            ),
            "format" => add(
                range,
                2,
                "`format!` allocates a new String every time around the loop",
                "write into one String with `write!(text, ...)`",
            ),
            "vec" => add(
                range,
                2,
                "a new list is allocated every time around the loop",
                "create it once before the loop and `clear()` it each time",
            ),
            _ => {}
        }
        return;
    }

    if let Some(param) = ast::Param::cast(node.clone()) {
        let Some(ast::Type::RefType(ref_ty)) = param.ty() else { return };
        if ref_ty.mut_token().is_some() {
            return;
        }
        let Some(ast::Type::PathType(inner)) = ref_ty.ty() else { return };
        let Some(segment) = inner.path().and_then(|path| path.segment()) else { return };
        let Some(name) = segment.name_ref() else { return };
        let (message, fix) = match &*name.text() {
            "String" => ("`&String` points to a pointer to the text", "take `&str`: faster, and works with `\"text\"` too"),
            "Vec" => ("`&Vec<T>` points to a pointer to the items", "take `&[T]`: faster, and works with arrays too"),
            "Box" => ("`&Box<T>` points to a pointer to the value", "take `&T`"),
            _ => return,
        };
        add(param.syntax().text_range(), 1, message, fix);
        return;
    }

    if let Some(for_expr) = ast::ForExpr::cast(node.clone())
        && for_expr.syntax().ancestors().skip(1).any(|it| is_loop_body_of(&it, for_expr.syntax()))
    {
        let range = for_expr.for_token().map_or(for_expr.syntax().text_range(), |t| t.text_range());
        add(
            range,
            2,
            "a loop inside a loop: n × m steps",
            "fine for small lists; for big ones, look things up in a `HashMap` instead",
        );
    }
}

/// Whether `node` runs once per round of a loop in the same function: it is
/// in the body of `for`, `while` or `loop`, or in a closure given to an
/// iterator method such as `map` or `for_each`.
fn in_loop(node: &SyntaxNode) -> bool {
    node.ancestors().skip(1).any(|ancestor| is_loop_body_of(&ancestor, node))
}

fn is_loop_body_of(ancestor: &SyntaxNode, node: &SyntaxNode) -> bool {
    if ast::Fn::can_cast(ancestor.kind()) {
        return false;
    }
    let body = if let Some(it) = ast::ForExpr::cast(ancestor.clone()) {
        it.loop_body()
    } else if let Some(it) = ast::WhileExpr::cast(ancestor.clone()) {
        it.loop_body()
    } else if let Some(it) = ast::LoopExpr::cast(ancestor.clone()) {
        it.loop_body()
    } else if let Some(closure) = ast::ClosureExpr::cast(ancestor.clone()) {
        return is_iterator_closure(&closure)
            && closure.body().is_some_and(|body| contains(body.syntax(), node));
    } else {
        return false;
    };
    body.is_some_and(|body| contains(body.syntax(), node))
}

fn contains(outer: &SyntaxNode, inner: &SyntaxNode) -> bool {
    outer.text_range().contains_range(inner.text_range())
}

fn is_iterator_closure(closure: &ast::ClosureExpr) -> bool {
    let Some(call) = closure
        .syntax()
        .parent()
        .and_then(|args| args.parent())
        .and_then(ast::MethodCallExpr::cast)
    else {
        return false;
    };
    let Some(name) = call.name_ref() else { return false };
    matches!(
        &*name.text(),
        "map" | "for_each" | "filter" | "filter_map" | "flat_map" | "fold" | "any" | "all"
            | "find" | "find_map" | "position" | "take_while" | "skip_while" | "inspect"
            | "map_while" | "scan" | "try_for_each" | "try_fold" | "retain" | "sort_by"
            | "sort_by_key" | "max_by_key" | "min_by_key"
    )
}

fn receiver_is_method(call: &ast::MethodCallExpr, method: &str) -> bool {
    matches!(
        call.receiver(),
        Some(ast::Expr::MethodCallExpr(inner))
            if inner.name_ref().is_some_and(|name| name.text() == method)
    )
}

fn receiver_is_collect(call: &ast::MethodCallExpr) -> bool {
    receiver_is_method(call, "collect")
}

fn receiver_is_list(sema: &Semantics<'_, RootDatabase>, call: &ast::MethodCallExpr) -> bool {
    let Some(receiver) = call.receiver() else { return false };
    let Some(ty) = sema.type_of_expr(&receiver) else { return false };
    let ty = ty.original.strip_references();
    if ty.is_slice() || ty.as_array(sema.db).is_some() {
        return true;
    }
    ty.as_adt().is_some_and(|adt| {
        matches!(adt.name(sema.db).as_str(), "Vec" | "VecDeque" | "LinkedList")
    })
}

/// Whether `call` (`x.push(..)` in a loop) grows a `Vec::new()` or
/// `String::new()` made in the same function.
fn grows_new_collection(call: &ast::MethodCallExpr) -> bool {
    let Some(ast::Expr::PathExpr(receiver)) = call.receiver() else { return false };
    let Some(name) = receiver.path().and_then(|path| path.as_single_name_ref()) else {
        return false;
    };
    let Some(func) = call.syntax().ancestors().find_map(ast::Fn::cast) else { return false };
    func.syntax().descendants().filter_map(ast::LetStmt::cast).any(|let_stmt| {
        let Some(ast::Pat::IdentPat(pat)) = let_stmt.pat() else { return false };
        let Some(pat_name) = pat.name() else { return false };
        if pat_name.text() != name.text() || let_stmt.syntax().text_range().end() > call.syntax().text_range().start() {
            return false;
        }
        match let_stmt.initializer() {
            Some(ast::Expr::CallExpr(init)) => {
                init.arg_list().is_some_and(|args| args.args().next().is_none())
                    && matches!(init.expr(), Some(ast::Expr::PathExpr(path))
                        if path.syntax().text().to_string().ends_with("::new"))
            }
            Some(ast::Expr::MacroExpr(init)) => init
                .macro_call()
                .and_then(|mac| mac.token_tree())
                .is_some_and(|tt| tt.syntax().text().to_string().trim() == "[]"),
            _ => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::fixture;

    #[track_caller]
    fn check(#[rust_analyzer::rust_fixture] ra_fixture: &str, score: u32, messages: &[&str]) {
        let (analysis, file_id) = fixture::file(ra_fixture);
        let result = analysis.speed_score(file_id).unwrap();
        let found: Vec<_> = result.findings.iter().map(|f| f.message.as_str()).collect();
        for message in messages {
            assert!(
                found.iter().any(|found| found.contains(message)),
                "missing finding {message:?} in {found:#?}"
            );
        }
        assert_eq!(result.findings.len(), messages.len(), "{found:#?}");
        assert_eq!(result.score, score, "{found:#?}");
    }

    #[test]
    fn clean_code_scores_100() {
        check(
            r#"
//- minicore: iterator
fn sum(items: &[u32]) -> u32 {
    let mut total = 0;
    for item in items {
        total += item;
    }
    total
}
"#,
            100,
            &[],
        );
    }

    #[test]
    fn finds_slow_patterns() {
        check(
            r#"
//- minicore: iterator, clone
fn names(list: &Vec<u32>, name: &String) {
    let mut out = Vec::new();
    for item in list {
        let copy = item.clone();
        out.push(copy);
        for _other in list {}
    }
}
"#,
            100 - 1 - 1 - 4 - 2 - 2,
            &["`&Vec<T>`", "`&String`", "`clone()`", "the list grows", "a loop inside a loop"],
        );
    }
}
