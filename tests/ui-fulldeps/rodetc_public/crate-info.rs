//@ run-pass
// Test that users are able to use stable mir APIs to retrieve information of the current crate

//@ ignore-stage1
//@ ignore-cross-compile
//@ ignore-remote
//@ edition: 2021

#![feature(rustc_private)]

extern crate rodetc_hir;
extern crate rodetc_middle;

extern crate rodetc_driver;
extern crate rodetc_interface;
#[macro_use]
extern crate rodetc_public;

use std::assert_matches;
use std::io::Write;
use std::ops::ControlFlow;

use rodetc_hir::def::DefKind;
use rodetc_public::ItemKind;
use rodetc_public::crate_def::CrateDef;
use rodetc_public::mir::mono::Instance;
use rodetc_public::ty::{RigidTy, TyKind};

const CRATE_NAME: &str = "input";

/// This function uses the Stable MIR APIs to get information about the test crate.
fn test_stable_mir() -> ControlFlow<()> {
    // Get the local crate using rodetc_public API.
    let local = rodetc_public::local_crate();
    assert_eq!(&local.name, CRATE_NAME);

    assert_eq!(rodetc_public::entry_fn(), None);

    // Find items in the local crate.
    let items = rodetc_public::all_local_items();
    assert!(get_item(&items, (DefKind::Fn, "input::foo::bar")).is_some());

    // Find the `std` crate and assert that there is only one of it.
    assert!(rodetc_public::find_crates("std").len() == 1);

    let bar = get_item(&items, (DefKind::Fn, "input::bar")).unwrap();
    let body = bar.expect_body();
    assert_eq!(body.locals().len(), 2);
    assert_eq!(body.blocks.len(), 1);
    let block = &body.blocks[0];
    assert_eq!(block.statements.len(), 1);
    match &block.statements[0].kind {
        rodetc_public::mir::StatementKind::Assign(..) => {}
        other => panic!("{other:?}"),
    }
    match &block.terminator.kind {
        rodetc_public::mir::TerminatorKind::Return => {}
        other => panic!("{other:?}"),
    }

    let foo_bar = get_item(&items, (DefKind::Fn, "input::foo_bar")).unwrap();
    let body = foo_bar.expect_body();
    assert_eq!(body.locals().len(), 5);
    assert_eq!(body.blocks.len(), 4);
    let block = &body.blocks[0];
    match &block.terminator.kind {
        rodetc_public::mir::TerminatorKind::Call { .. } => {}
        other => panic!("{other:?}"),
    }

    let types = get_item(&items, (DefKind::Fn, "input::types")).unwrap();
    let body = types.expect_body();
    assert_eq!(body.locals().len(), 6);
    assert_matches!(
        body.locals()[0].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Bool)
    );
    assert_matches!(
        body.locals()[1].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Bool)
    );
    assert_matches!(
        body.locals()[2].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Char)
    );
    assert_matches!(
        body.locals()[3].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Int(
            rodetc_public::ty::IntTy::I32
        ))
    );
    assert_matches!(
        body.locals()[4].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Uint(
            rodetc_public::ty::UintTy::U64
        ))
    );
    assert_matches!(
        body.locals()[5].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Float(
            rodetc_public::ty::FloatTy::F64
        ))
    );

    let drop = get_item(&items, (DefKind::Fn, "input::drop")).unwrap();
    let body = drop.expect_body();
    assert_eq!(body.blocks.len(), 2);
    let block = &body.blocks[0];
    match &block.terminator.kind {
        rodetc_public::mir::TerminatorKind::Drop { .. } => {}
        other => panic!("{other:?}"),
    }

    let assert = get_item(&items, (DefKind::Fn, "input::assert")).unwrap();
    let body = assert.expect_body();
    assert_eq!(body.blocks.len(), 2);
    let block = &body.blocks[0];
    match &block.terminator.kind {
        rodetc_public::mir::TerminatorKind::Assert { .. } => {}
        other => panic!("{other:?}"),
    }

    let monomorphic = get_item(&items, (DefKind::Fn, "input::monomorphic")).unwrap();
    let instance = Instance::try_from(monomorphic.clone()).unwrap();
    for block in instance.body().unwrap().blocks {
        match &block.terminator.kind {
            rodetc_public::mir::TerminatorKind::Call { func, .. } => {
                let TyKind::RigidTy(ty) = func.ty(&body.locals()).unwrap().kind() else {
                    unreachable!()
                };
                let RigidTy::FnDef(def, args) = ty else { unreachable!() };
                let next_func = Instance::resolve(def, &args).unwrap();
                match next_func.body().unwrap().locals()[1].ty.kind() {
                    TyKind::RigidTy(RigidTy::Uint(_)) | TyKind::RigidTy(RigidTy::Tuple(_)) => {}
                    other => panic!("{other:?}"),
                }
            }
            rodetc_public::mir::TerminatorKind::Return => {}
            other => panic!("{other:?}"),
        }
    }

    let foo_const = get_item(&items, (DefKind::Const, "input::FOO")).unwrap();
    // Ensure we don't panic trying to get the body of a constant.
    foo_const.expect_body();

    let locals_fn = get_item(&items, (DefKind::Fn, "input::locals")).unwrap();
    let body = locals_fn.expect_body();
    assert_eq!(body.locals().len(), 4);
    assert_matches!(
        body.ret_local().ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Char)
    );
    assert_eq!(body.arg_locals().len(), 2);
    assert_matches!(
        body.arg_locals()[0].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Int(
            rodetc_public::ty::IntTy::I32
        ))
    );
    assert_matches!(
        body.arg_locals()[1].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Uint(
            rodetc_public::ty::UintTy::U64
        ))
    );
    assert_eq!(body.inner_locals().len(), 1);
    // If conditions have an extra inner local to hold their results
    assert_matches!(
        body.inner_locals()[0].ty.kind(),
        rodetc_public::ty::TyKind::RigidTy(rodetc_public::ty::RigidTy::Bool)
    );

    ControlFlow::Continue(())
}

// Use internal API to find a function in a crate.
fn get_item<'a>(
    items: &'a rodetc_public::CrateItems,
    item: (DefKind, &str),
) -> Option<&'a rodetc_public::CrateItem> {
    items.iter().find(|crate_item| {
        matches!(
            (item.0, crate_item.kind()),
            (DefKind::Fn, ItemKind::Fn) | (DefKind::Const, ItemKind::Const)
        ) && crate_item.name() == item.1
    })
}

/// This test will generate and analyze a dummy crate using the stable mir.
/// For that, it will first write the dummy crate into a file.
/// Then it will create a `RustcPublic` using custom arguments and then
/// it will run the compiler.
fn main() {
    let path = "input.rs";
    generate_input(&path).unwrap();
    let args = &[
        "rustc".to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        path.to_string(),
    ];
    run!(args, test_stable_mir).unwrap();
}

fn generate_input(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write!(
        file,
        r#"
    pub const FOO: u32 = 1 + 2;

    fn generic<T, const U: usize>(t: T) -> [(); U] {{
        _ = t;
        [(); U]
    }}

    pub fn monomorphic() {{
        generic::<(), 5>(());
        generic::<u32, 0>(45);
    }}

    mod foo {{
        pub fn bar(i: i32) -> i64 {{
            i as i64
        }}
    }}

    pub fn bar(x: i32) -> i32 {{
        x
    }}

    pub fn foo_bar(x: i32, y: i32) -> i64 {{
        let x_64 = foo::bar(x);
        let y_64 = foo::bar(y);
        x_64.wrapping_add(y_64)
    }}

    pub fn types(b: bool, _: char, _: i32, _: u64, _: f64) -> bool {{
        b
    }}

    pub fn drop(_: String) {{}}

    pub fn assert(x: i32) -> i32 {{
        x + 1
    }}

    pub fn locals(a: i32, _: u64) -> char {{
        if a > 5 {{
            'a'
        }} else {{
            'b'
        }}
    }}"#
    )?;
    Ok(())
}
