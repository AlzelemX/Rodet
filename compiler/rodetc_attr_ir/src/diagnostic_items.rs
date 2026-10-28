use rodetc_data_structures::fx::FxIndexMap;
use rodetc_macros::StableHash;
use rodetc_span::Symbol;
use rodetc_span::def_id::{DefId, DefIdMap};

#[derive(Debug, Default, StableHash)]
pub struct DiagnosticItems {
    #[stable_hash(ignore)]
    pub id_to_name: DefIdMap<Symbol>,
    pub name_to_id: FxIndexMap<Symbol, DefId>,
}
