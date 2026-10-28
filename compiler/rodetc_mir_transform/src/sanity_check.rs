use rodetc_middle::mir::Body;
use rodetc_middle::ty::TyCtxt;
use rodetc_mir_dataflow::rustc_peek::sanity_check;

pub(super) struct SanityCheck;

impl<'tcx> crate::MirLint<'tcx> for SanityCheck {
    fn run_lint(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
        sanity_check(tcx, body);
    }
}
