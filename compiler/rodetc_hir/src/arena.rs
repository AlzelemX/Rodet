//! Declares an arena that can allocate values of any `Copy` type, and of
//! any `!Copy` type listed below.

rodetc_arena::declare_arena! {
    // HIR types
    asm_template: rodetc_ast::InlineAsmTemplatePiece,
    attribute: rodetc_attr_ir::Attribute,
    owner_info: crate::OwnerInfo<'tcx>,
    macro_def: rodetc_ast::MacroDef,
    delegation_info: crate::DelegationInfo,
}
