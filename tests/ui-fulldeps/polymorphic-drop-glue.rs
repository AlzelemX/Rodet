//@ run-pass
//@ run-flags: --sysroot {{sysroot-base}} --edition=2021
//@ ignore-stage1 (requires matching sysroot built with in-tree compiler)
//@ ignore-cross-compile
//@ ignore-remote
//@ ignore-backends: gcc
#![feature(rustc_private)]

extern crate rodetc_attr_ir;
extern crate rodetc_driver;
extern crate rodetc_hir;
extern crate rodetc_interface;
extern crate rodetc_middle;
extern crate rodetc_mir_transform;
extern crate rodetc_span;

use std::process::ExitCode;

use rodetc_driver::Compilation;
use rodetc_attr_ir::lang_items::LangItem;
use rodetc_hir::def::DefKind;
use rodetc_interface::interface::Compiler;
use rodetc_middle::ty::{self, TyCtxt};
use rodetc_span::DUMMY_SP;

fn main() -> ExitCode {
    rodetc_driver::catch_with_exit_code(move || {
        std::fs::write(
            "drop_glue_polymorphic_const_generic_input.rs",
            r#"
                struct Unit;

                struct ContainsArray<const K: usize> {
                    array: [Unit; K],
                    make_non_trivial_drop: Box<u32>,
                }

                fn foo(_: ContainsArray<42>) {}
            "#,
        )
        .unwrap();

        let mut args: Vec<_> = std::env::args().collect();
        args.push("--crate-type=lib".to_owned());
        args.push("drop_glue_polymorphic_const_generic_input.rs".to_owned());

        rodetc_driver::compiler_entrypoint(&args, &mut CompilerCalls);
    })
}

struct CompilerCalls;

impl rodetc_driver::Callbacks for CompilerCalls {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        tcx.sess.dcx().abort_if_errors();

        let drop_glue = tcx.require_lang_item(LangItem::DropGlue, DUMMY_SP);
        let contains_array = tcx
            .hir_crate_items(())
            .free_items()
            .map(|id| id.owner_id.to_def_id())
            .find(|&def_id| {
                tcx.def_kind(def_id) == DefKind::Struct
                    && tcx.def_path_str(def_id).ends_with("ContainsArray")
            })
            .unwrap();

        // Regression test for ICE due to the drop glue code not correctly handling generic
        // contexts.
        let ty = tcx.type_of(contains_array).instantiate_identity().skip_norm_wip();
        let typing_env = ty::TypingEnv::post_analysis(tcx, contains_array);
        rodetc_mir_transform::build_drop_shim(tcx, drop_glue, Some(ty), typing_env);

        Compilation::Stop
    }
}
