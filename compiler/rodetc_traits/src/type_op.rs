use std::fmt;

use rodetc_infer::infer::TyCtxtInferExt;
use rodetc_infer::infer::canonical::{Canonical, CanonicalQueryInput, QueryResponse};
use rodetc_middle::query::Providers;
use rodetc_middle::traits::query::NoSolution;
use rodetc_middle::ty::{Clause, FnSig, ParamEnvAnd, PolyFnSig, Ty, TyCtxt, TypeFoldable};
use rodetc_span::DUMMY_SP;
use rodetc_trait_selection::infer::InferCtxtBuilderExt;
use rodetc_trait_selection::traits::query::normalize::QueryNormalizeExt;
use rodetc_trait_selection::traits::query::type_op::ascribe_user_type::{
    AscribeUserType, type_op_ascribe_user_type_with_span,
};
use rodetc_trait_selection::traits::query::type_op::normalize::Normalize;
use rodetc_trait_selection::traits::query::type_op::prove_predicate::{
    ProvePredicate, type_op_prove_predicate_with_cause,
};
use rodetc_trait_selection::traits::{Normalized, ObligationCause, ObligationCtxt};

pub(crate) fn provide(p: &mut Providers) {
    *p = Providers {
        type_op_ascribe_user_type,
        type_op_prove_predicate,
        type_op_normalize_ty,
        type_op_normalize_clause,
        type_op_normalize_fn_sig,
        type_op_normalize_poly_fn_sig,
        ..*p
    };
}

fn type_op_ascribe_user_type<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, AscribeUserType<'tcx>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, ()>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, |ocx, key| {
        type_op_ascribe_user_type_with_span(ocx, key, DUMMY_SP)
    })
}

fn type_op_normalize<'tcx, T>(
    ocx: &ObligationCtxt<'_, 'tcx>,
    key: ParamEnvAnd<'tcx, Normalize<'tcx, T>>,
) -> Result<T, NoSolution>
where
    T: fmt::Debug + TypeFoldable<TyCtxt<'tcx>>,
{
    let ParamEnvAnd { param_env, value: Normalize { value } } = key;
    let Normalized { value, obligations } = ocx
        .infcx
        .at(&ObligationCause::dummy(), param_env)
        .query_normalize(value.skip_normalization())?;
    ocx.register_obligations(obligations);
    Ok(value)
}

fn type_op_normalize_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<'tcx, Ty<'tcx>>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, Ty<'tcx>>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, type_op_normalize)
}

fn type_op_normalize_clause<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<'tcx, Clause<'tcx>>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, Clause<'tcx>>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, type_op_normalize)
}

fn type_op_normalize_fn_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<'tcx, FnSig<'tcx>>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, FnSig<'tcx>>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, type_op_normalize)
}

fn type_op_normalize_poly_fn_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, Normalize<'tcx, PolyFnSig<'tcx>>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, PolyFnSig<'tcx>>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, type_op_normalize)
}

fn type_op_prove_predicate<'tcx>(
    tcx: TyCtxt<'tcx>,
    canonicalized: CanonicalQueryInput<'tcx, ParamEnvAnd<'tcx, ProvePredicate<'tcx>>>,
) -> Result<&'tcx Canonical<'tcx, QueryResponse<'tcx, ()>>, NoSolution> {
    tcx.infer_ctxt().enter_canonical_trait_query(&canonicalized, |ocx, key| {
        type_op_prove_predicate_with_cause(ocx, key, ObligationCause::dummy());
        Ok(())
    })
}
