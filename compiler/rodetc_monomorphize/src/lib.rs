// tidy-alphabetical-start
#![feature(file_buffered)]
#![feature(impl_trait_in_assoc_type)]
#![feature(once_cell_get_mut)]
// tidy-alphabetical-end

use rodetc_attr_ir::lang_items::LangItem;
use rodetc_middle::query::TyCtxtAt;
use rodetc_middle::traits;
use rodetc_middle::ty::adjustment::CustomCoerceUnsized;
use rodetc_middle::ty::{self, Ty};
use rodetc_middle::util::Providers;
use rodetc_span::{ErrorGuaranteed, bug};

mod collector;
mod diagnostics;
mod graph_checks;
mod mono_checks;
mod offload;
mod partitioning;
mod util;

// Exposed so `rodetc_codegen_ssa::base::codegen_crate` can trigger the
// host-metadata manifest write.
pub use offload::manifest::write_host_metadata_offload_manifest;

fn custom_coerce_unsize_info<'tcx>(
    tcx: TyCtxtAt<'tcx>,
    source_ty: Ty<'tcx>,
    target_ty: Ty<'tcx>,
) -> Result<CustomCoerceUnsized, ErrorGuaranteed> {
    let trait_ref = ty::TraitRef::new(
        tcx.tcx,
        tcx.require_lang_item(LangItem::CoerceUnsized, tcx.span),
        [source_ty, target_ty],
    );

    match tcx
        .codegen_select_candidate(ty::TypingEnv::fully_monomorphized().as_query_input(trait_ref))
    {
        Ok(traits::ImplSource::UserDefined(traits::ImplSourceUserDefinedData {
            impl_def_id,
            ..
        })) => Ok(tcx.coerce_unsized_info(*impl_def_id)?.custom_kind.unwrap()),
        impl_source => {
            bug!(
                "invalid `CoerceUnsized` from {source_ty} to {target_ty}: impl_source: {:?}",
                impl_source
            );
        }
    }
}

pub fn provide(providers: &mut Providers) {
    partitioning::provide(providers);
    mono_checks::provide(&mut providers.queries);
}
