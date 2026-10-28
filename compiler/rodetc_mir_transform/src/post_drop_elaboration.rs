use rodetc_const_eval::check_consts;
use rodetc_middle::mir::*;
use rodetc_middle::ty::TyCtxt;

use crate::MirLint;

pub(super) struct CheckLiveDrops;

impl<'tcx> MirLint<'tcx> for CheckLiveDrops {
    fn run_lint(&self, tcx: TyCtxt<'tcx>, body: &Body<'tcx>) {
        check_consts::post_drop_elaboration::check_live_drops(tcx, body);
    }
}
