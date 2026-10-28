use std::hash::Hash;

use rodetc_data_structures::unord::UnordMap;
use rodetc_index::{Idx, IndexVec};
use rodetc_middle::ty::{Binder, EarlyBinder, GenericArg, Region};
use rodetc_span::Symbol;

use crate::rmeta::{LazyArray, LazyValue, LocalDefId};

pub(crate) trait ParameterizedOverTcx: 'static {
    type Value<'tcx>;
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Option<T> {
    type Value<'tcx> = Option<T::Value<'tcx>>;
}

impl<A: ParameterizedOverTcx, B: ParameterizedOverTcx> ParameterizedOverTcx for (A, B) {
    type Value<'tcx> = (A::Value<'tcx>, B::Value<'tcx>);
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Vec<T> {
    type Value<'tcx> = Vec<T::Value<'tcx>>;
}

impl<I: Idx + 'static, T: ParameterizedOverTcx> ParameterizedOverTcx for IndexVec<I, T> {
    type Value<'tcx> = IndexVec<I, T::Value<'tcx>>;
}

impl<I: Hash + Eq + 'static, T: ParameterizedOverTcx> ParameterizedOverTcx for UnordMap<I, T> {
    type Value<'tcx> = UnordMap<I, T::Value<'tcx>>;
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for Binder<'static, T> {
    type Value<'tcx> = Binder<'tcx, T::Value<'tcx>>;
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for EarlyBinder<'static, T> {
    type Value<'tcx> = EarlyBinder<'tcx, T::Value<'tcx>>;
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for LazyValue<T> {
    type Value<'tcx> = LazyValue<T::Value<'tcx>>;
}

impl<T: ParameterizedOverTcx> ParameterizedOverTcx for LazyArray<T> {
    type Value<'tcx> = LazyArray<T::Value<'tcx>>;
}

impl ParameterizedOverTcx for Region<'static> {
    type Value<'tcx> = Region<'tcx>;
}

impl ParameterizedOverTcx for GenericArg<'static> {
    type Value<'tcx> = GenericArg<'tcx>;
}

macro_rules! trivially_parameterized_over_tcx {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ParameterizedOverTcx for $ty {
                #[allow(unused_lifetimes)]
                type Value<'tcx> = $ty;
            }
        )*
    }
}

trivially_parameterized_over_tcx! {
    bool,
    u32,
    u64,
    usize,
    std::string::String,
    // tidy-alphabetical-start
    crate::rmeta::AttrFlags,
    crate::rmeta::CrateDep,
    crate::rmeta::CrateHeader,
    crate::rmeta::CrateRoot,
    crate::rmeta::CrateRootUnhashed,
    crate::rmeta::IncoherentImpls,
    crate::rmeta::LocalDefId,
    crate::rmeta::ProcMacroKind,
    crate::rmeta::TraitImpls,
    crate::rmeta::VariantData,
    crate::rmeta::remapped_def_id::RemappedRawDefId,
    rodetc_abi::ReprOptions,
    rodetc_ast::DelimArgs,
    rodetc_attr_ir::Attribute,
    rodetc_attr_ir::ConstStability,
    rodetc_attr_ir::DefaultBodyStability,
    rodetc_attr_ir::Deprecation,
    rodetc_attr_ir::EiiDecl,
    rodetc_attr_ir::EiiImpl,
    rodetc_attr_ir::Stability,
    rodetc_attr_ir::StrippedCfgItem<LocalDefId>,
    rodetc_attr_ir::lang_items::LangItem,
    rodetc_crate_store::ForeignModule,
    rodetc_crate_store::LinkagePreference,
    rodetc_crate_store::NativeLib,
    rodetc_hir::Constness,
    rodetc_hir::CoroutineKind,
    rodetc_hir::Defaultness,
    rodetc_hir::OpaqueTyOrigin<rodetc_hir::def_id::DefId>,
    rodetc_hir::PreciseCapturingArgKind<Symbol, Symbol>,
    rodetc_hir::Safety,
    rodetc_hir::def::DefKind,
    rodetc_hir::def_id::DefId,
    rodetc_hir::def_id::DefIndex,
    rodetc_hir::definitions::DefKey,
    rodetc_index::bit_set::DenseBitSet<u32>,
    rodetc_middle::middle::codegen_fn_attrs::CodegenFnAttrs,
    rodetc_middle::middle::debugger_visualizer::DebuggerVisualizerFile,
    rodetc_middle::middle::deduced_param_attrs::DeducedParamAttrs,
    rodetc_middle::middle::exported_symbols::SymbolExportInfo,
    rodetc_middle::middle::lib_features::FeatureStability,
    rodetc_middle::middle::resolve::AmbigModChild,
    rodetc_middle::middle::resolve::DocLinkResMap,
    rodetc_middle::middle::resolve::ModChild,
    rodetc_middle::middle::resolve_bound_vars::ObjectLifetimeDefault,
    rodetc_middle::mir::ConstQualifs,
    rodetc_middle::mir::ConstValue,
    rodetc_middle::ty::AnonConstKind,
    rodetc_middle::ty::AssocContainer,
    rodetc_middle::ty::AsyncDestructor,
    rodetc_middle::ty::Asyncness,
    rodetc_middle::ty::Destructor,
    rodetc_middle::ty::Generics,
    rodetc_middle::ty::ImplTraitInTraitData,
    rodetc_middle::ty::IntrinsicDef,
    rodetc_middle::ty::RestrictionKind,
    rodetc_middle::ty::TraitDef,
    rodetc_middle::ty::Variance,
    rodetc_middle::ty::Visibility<LocalDefId>,
    rodetc_middle::ty::adjustment::CoerceUnsizedInfo,
    rodetc_middle::ty::fast_reject::SimplifiedType,
    rodetc_session::config::TargetModifier,
    rodetc_session::config::mitigation_coverage::DeniedPartialMitigation,
    rodetc_span::ExpnData,
    rodetc_span::ExpnHash,
    rodetc_span::ExpnId,
    rodetc_span::Ident,
    rodetc_span::SourceFile,
    rodetc_span::Span,
    rodetc_span::Symbol,
    rodetc_span::hygiene::SyntaxContextKey,
    // tidy-alphabetical-end
}

// HACK(compiler-errors): This macro rule can only take a fake path,
// not a real, due to parsing ambiguity reasons.
macro_rules! parameterized_over_tcx {
    ($($( $fake_path:ident )::+ ),+ $(,)?) => {
        $(
            impl ParameterizedOverTcx for $( $fake_path )::+ <'static> {
                type Value<'tcx> = $( $fake_path )::+ <'tcx>;
            }
        )*
    }
}

parameterized_over_tcx! {
    // tidy-alphabetical-start
    crate::rmeta::DefPathHashMapRef,
    rodetc_middle::middle::exported_symbols::ExportedSymbol,
    rodetc_middle::mir::Body,
    rodetc_middle::mir::CoroutineLayout,
    rodetc_middle::mir::interpret::ConstAllocation,
    rodetc_middle::ty::Clause,
    rodetc_middle::ty::Const,
    rodetc_middle::ty::ConstConditions,
    rodetc_middle::ty::FnSig,
    rodetc_middle::ty::GenericClauses,
    rodetc_middle::ty::ImplTraitHeader,
    rodetc_middle::ty::TraitRef,
    rodetc_middle::ty::Ty,
    // tidy-alphabetical-end
}
