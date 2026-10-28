//! Declares [`rodetc_middle::arena::Arena`], which can allocate values of any
//! `Copy` type, and any `!Copy` type explicitly listed below.

use rodetc_serialize::Decodable;
use rodetc_span::{Span, Spanned};

use crate::mono::MonoItem;
use crate::ty::codec::{RefDecodable, TyDecoder};
use crate::ty::{self, Ty, TyCtxt};

// If a type `T` supported by the arena also needs to support decoding into `&'tcx T`
// backed by an arena allocation (via `RefDecodable`), add it to the list in
// `impl_ref_decodable_into_arena!`.

rodetc_arena::declare_arena! {
    layout: rodetc_abi::LayoutData<rodetc_abi::FieldIdx, rodetc_abi::VariantIdx>,
    proxy_coroutine_layout: rodetc_middle::mir::CoroutineLayout<'tcx>,
    fn_abi: rodetc_target::callconv::FnAbi<'tcx, Ty<'tcx>>,
    adt_def: rodetc_middle::ty::AdtDefData,
    steal_thir: rodetc_data_structures::steal::Steal<rodetc_middle::thir::Thir<'tcx>>,
    steal_mir: rodetc_data_structures::steal::Steal<rodetc_middle::mir::Body<'tcx>>,
    mir: rodetc_middle::mir::Body<'tcx>,
    steal_promoted:
        rodetc_data_structures::steal::Steal<
            rodetc_index::IndexVec<
                rodetc_middle::mir::Promoted,
                rodetc_middle::mir::Body<'tcx>
            >
        >,
    promoted:
        rodetc_index::IndexVec<
            rodetc_middle::mir::Promoted,
            rodetc_middle::mir::Body<'tcx>
        >,
    typeck_results: rodetc_middle::ty::TypeckResults<'tcx>,
    borrowck_result:
        rodetc_data_structures::fx::FxIndexMap<
            rodetc_hir::def_id::LocalDefId,
            rodetc_middle::ty::DefinitionSiteHiddenType<'tcx>,
        >,
    resolver:
        rodetc_data_structures::steal::Steal<
            rodetc_middle::middle::resolve::ResolverAstLowering<'tcx>
        >,
    index_ast:
        Option<rodetc_data_structures::steal::Steal<(
            std::sync::Arc<rodetc_middle::middle::resolve::ResolverAstLowering<'tcx>>,
            rodetc_middle::middle::resolve::AstOwner
        )>>,
    crate_alone: rodetc_data_structures::steal::Steal<rodetc_ast::Crate>,
    crate_for_resolver: rodetc_data_structures::steal::Steal<(rodetc_ast::Crate, rodetc_ast::AttrVec)>,
    resolutions: rodetc_middle::middle::resolve::ResolverGlobalCtxt,
    const_allocs: rodetc_middle::mir::interpret::Allocation,
    region_scope_tree: rodetc_middle::middle::region::ScopeTree,
    // Required for the incremental on-disk cache
    mir_keys: rodetc_hir::def_id::DefIdSet,
    dropck_outlives:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx,
                rodetc_middle::traits::query::DropckOutlivesResult<'tcx>
            >
        >,
    normalize_canonicalized_projection:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx,
                rodetc_middle::traits::query::NormalizationResult<'tcx>
            >
        >,
    implied_outlives_bounds:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx,
                Vec<rodetc_middle::traits::query::OutlivesBound<'tcx>>
            >
        >,
    mir_borrowck_implied_outlives_bounds:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx,
                rodetc_middle::traits::query::MirBorrowckImpliedOutlivesBounds<'tcx>
            >
        >,
    dtorck_constraint: rodetc_middle::traits::query::DropckConstraint<'tcx>,
    candidate_step: rodetc_middle::traits::query::CandidateStep<'tcx>,
    autoderef_bad_ty: rodetc_middle::traits::query::MethodAutoderefBadTy<'tcx>,
    query_region_constraints: rodetc_middle::infer::canonical::QueryRegionConstraints<'tcx>,
    type_op_subtype:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx, ()>
        >,
    type_op_normalize_poly_fn_sig:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx, rodetc_middle::ty::PolyFnSig<'tcx>>
        >,
    type_op_normalize_fn_sig:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx, rodetc_middle::ty::FnSig<'tcx>>
        >,
    type_op_normalize_clause:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx, rodetc_middle::ty::Clause<'tcx>>
        >,
    type_op_normalize_ty:
        rodetc_middle::infer::canonical::Canonical<'tcx,
            rodetc_middle::infer::canonical::QueryResponse<'tcx, Ty<'tcx>>
        >,
    inspect_probe: rodetc_middle::traits::solve::inspect::Probe<TyCtxt<'tcx>>,
    effective_visibilities: rodetc_middle::middle::privacy::EffectiveVisibilities,
    upvars_mentioned: rodetc_data_structures::fx::FxIndexMap<rodetc_hir::HirId, rodetc_hir::Upvar>,
    dyn_compatibility_violations: rodetc_middle::traits::DynCompatibilityViolation,
    codegen_unit: rodetc_middle::mono::CodegenUnit<'tcx>,
    attribute: rodetc_attr_ir::Attribute,
    name_set: rodetc_data_structures::unord::UnordSet<rodetc_span::Symbol>,
    autodiff_item: rodetc_attr_ir::AutoDiffItem,
    ordered_name_set: rodetc_data_structures::fx::FxIndexSet<rodetc_span::Symbol>,
    stable_order_of_exportable_impls:
        rodetc_data_structures::fx::FxIndexMap<rodetc_hir::def_id::DefId, usize>,

    // Note that this deliberately duplicates items in the `rodetc_hir::arena`,
    // since we need to allocate this type on both the `rodetc_hir` arena
    // (during lowering) and the `rodetc_middle` arena (for decoding MIR)
    asm_template: rodetc_ast::InlineAsmTemplatePiece,
    used_trait_imports: rodetc_data_structures::unord::UnordSet<rodetc_hir::def_id::LocalDefId>,
    is_late_bound_map: rodetc_data_structures::fx::FxIndexSet<rodetc_hir::ItemLocalId>,
    impl_source: rodetc_middle::traits::ImplSource<'tcx, ()>,

    dep_kind_vtable: rodetc_middle::dep_graph::DepKindVTable<'tcx>,

    trait_impl_trait_tys:
        rodetc_data_structures::unord::UnordMap<
            rodetc_hir::def_id::DefId,
            rodetc_middle::ty::EarlyBinder<'tcx, Ty<'tcx>>
        >,
    external_constraints: rodetc_middle::traits::solve::ExternalConstraintsData<TyCtxt<'tcx>>,
    doc_link_resolutions: rodetc_middle::middle::resolve::DocLinkResMap,
    stripped_cfg_items: rodetc_attr_ir::StrippedCfgItem,
    mod_child: rodetc_middle::middle::resolve::ModChild,
    features: rodetc_feature::Features,
    specialization_graph: rodetc_middle::traits::specialization_graph::Graph,
    crate_inherent_impls: rodetc_middle::ty::CrateInherentImpls,
    hir_owner_nodes: rodetc_hir::OwnerNodes<'tcx>,
    token_stream: rodetc_ast::tokenstream::TokenStream,
    parenting: rodetc_hir::def_id::LocalDefIdMap<rodetc_hir::ItemLocalId>,
    trait_candidates: rodetc_hir::ItemLocalMap<&'tcx [rodetc_hir::TraitCandidate<'tcx>]>,
    delayed_lints: rodetc_data_structures::steal::Steal<rodetc_hir::lints::DelayedLints>,
}

#[inline]
fn decode_arena_allocatable<'tcx, D, C, T>(decoder: &mut D) -> &'tcx T
where
    D: TyDecoder<'tcx>,
    T: ArenaAllocatable<'tcx, C> + Decodable<D>,
{
    let value: T = Decodable::decode(decoder);
    decoder.interner().arena.alloc(value)
}

#[inline]
fn decode_arena_allocatable_slice<'tcx, D, C, T>(decoder: &mut D) -> &'tcx [T]
where
    D: TyDecoder<'tcx>,
    T: ArenaAllocatable<'tcx, C> + Decodable<D>,
{
    // The decoder for slices must match the decoder for `Vec<T>`,
    // which is a `usize` length followed by that many `T`.
    let len = decoder.read_usize();
    decoder.interner().arena.alloc_from_iter((0..len).map(|_| T::decode(decoder)))
}

macro_rules! impl_ref_decodable_into_arena {
    (
        $(
            $ty:ty,
        )*
    ) => {
        $(
            impl<'tcx> RefDecodable<'tcx> for $ty {
                #[inline]
                fn decode(decoder: &mut impl TyDecoder<'tcx>) -> &'tcx Self {
                    decode_arena_allocatable(decoder)
                }
            }

            impl<'tcx> RefDecodable<'tcx> for [$ty] {
                #[inline]
                fn decode(decoder: &mut impl TyDecoder<'tcx>) -> &'tcx Self {
                    decode_arena_allocatable_slice(decoder)
                }
            }
        )*
    }
}

// For each of these types, implements `RefDecodable` for `T` (and `[T]`) by
// decoding to `T` and then moving the value or values into an arena allocation.
//
// Types in this list must be `ArenaAllocatable`, either because they are `Copy`
// or because they are listed in the `declare_arena!` invocation.
//
// Types in this list must also implement `Decodable<D>` for all `D: TyDecoder<'tcx>`.
impl_ref_decodable_into_arena! {
    // tidy-alphabetical-start
    (rodetc_middle::middle::exported_symbols::ExportedSymbol<'tcx>, rodetc_middle::middle::exported_symbols::SymbolExportInfo),
    (ty::Clause<'tcx>, Span),
    (ty::PolyTraitRef<'tcx>, Span),
    Spanned<MonoItem<'tcx>>,
    rodetc_ast::InlineAsmTemplatePiece,
    rodetc_ast::tokenstream::TokenStream,
    rodetc_attr_ir::Attribute,
    rodetc_data_structures::unord::UnordMap<rodetc_span::def_id::DefId, rodetc_middle::ty::EarlyBinder<'tcx, Ty<'tcx>>>,
    rodetc_data_structures::unord::UnordSet<rodetc_span::def_id::LocalDefId>,
    rodetc_index::IndexVec<rodetc_middle::mir::Promoted, rodetc_middle::mir::Body<'tcx>>,
    rodetc_middle::middle::deduced_param_attrs::DeducedParamAttrs,
    rodetc_middle::mir::Body<'tcx>,
    rodetc_middle::traits::ImplSource<'tcx, ()>,
    rodetc_middle::traits::specialization_graph::Graph,
    rodetc_middle::ty::TypeckResults<'tcx>,
    rodetc_middle::ty::Variance,
    rodetc_span::Ident,
    rodetc_span::Span,
    rodetc_span::def_id::DefId,
    rodetc_span::def_id::LocalDefId,
    // tidy-alphabetical-end
}
