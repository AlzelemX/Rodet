use std::any::Any;

use rodetc_data_structures::sync::DynSend;
use rodetc_errors::{Diag, DiagCtxtHandle, Diagnostic, Level};
use rodetc_session::Session;

pub struct DiagAndSess<'sess> {
    pub callback: Box<
        dyn for<'b> FnOnce(DiagCtxtHandle<'b>, Level, &dyn Any) -> Diag<'b> + DynSend + 'static,
    >,
    pub sess: &'sess Session,
}

impl<'a> Diagnostic<'a> for DiagAndSess<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'a>, level: Level) -> Diag<'a> {
        (self.callback)(dcx, level, self.sess)
    }
}
