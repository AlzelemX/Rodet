use rodetc_attr_ir::Linkage;
use rodetc_hir::def_id::DefId;
use rodetc_middle::mono::Visibility;
use rodetc_middle::ty::Instance;

pub trait PreDefineCodegenMethods<'tcx> {
    fn predefine_static(
        &mut self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
    fn predefine_fn(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
}
