//! A set of traits that define a stable interface to rustc's internals.
//!
//! These traits abstract rustc's internal APIs, allowing rodetc_public to maintain a stable
//! interface regardless of internal compiler changes.

use rodetc_middle::mir::interpret::AllocRange;
use rodetc_middle::ty::Ty;
use rodetc_span::def_id::DefId;

pub trait TyHelpers<'tcx> {
    fn new_foreign(&self, def_id: DefId) -> Ty<'tcx>;
}

pub trait AllocRangeHelpers<'tcx> {
    fn alloc_range(&self, offset: rodetc_abi::Size, size: rodetc_abi::Size) -> AllocRange;
}
