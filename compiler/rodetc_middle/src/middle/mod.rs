pub mod codegen_fn_attrs;
pub mod dead_code;
pub mod debugger_visualizer;
pub mod deduced_param_attrs;
pub mod dependency_format;
pub mod exported_symbols;
pub mod lang_items;
pub mod lib_features {
    use rodetc_data_structures::unord::UnordMap;
    use rodetc_macros::{BlobDecodable, Encodable, StableHash};
    use rodetc_span::{Span, Symbol};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[derive(StableHash, Encodable, BlobDecodable)]
    pub enum FeatureStability {
        AcceptedSince(Symbol),
        Unstable { old_name: Option<Symbol> },
    }

    #[derive(StableHash, Debug, Default)]
    pub struct LibFeatures {
        pub stability: UnordMap<Symbol, (FeatureStability, Span)>,
    }

    impl LibFeatures {
        pub fn to_sorted_vec(&self) -> Vec<(Symbol, FeatureStability)> {
            self.stability
                .to_sorted_stable_ord()
                .iter()
                .map(|&(&sym, &(stab, _))| (sym, stab))
                .collect()
        }
    }
}
pub mod privacy;
pub mod region;
pub mod resolve;
pub mod resolve_bound_vars;
pub mod stability;
