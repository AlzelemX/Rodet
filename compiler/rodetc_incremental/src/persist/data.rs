//! The data that we will serialize and deserialize.

use rodetc_macros::{Decodable, Encodable};
use rodetc_middle::dep_graph::{WorkProduct, WorkProductId};

#[derive(Debug, Encodable, Decodable)]
pub(crate) struct SerializedWorkProduct {
    /// node that produced the work-product
    pub id: WorkProductId,

    /// work-product data itself
    pub work_product: WorkProduct,
}
