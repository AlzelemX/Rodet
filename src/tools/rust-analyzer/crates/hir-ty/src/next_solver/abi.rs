//! ABI-related things in the next-trait-solver.
use rodetc_abi::ExternAbi;
use rodetc_ast_ir::visit::VisitorResult;
use rodetc_type_ir::{
    FallibleTypeFolder, TypeFoldable, TypeFolder, TypeVisitable, TypeVisitor, error::TypeError,
    relate::Relate,
};

use super::interner::DbInterner;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Safety {
    Unsafe,
    Safe,
}

impl<'db> Relate<DbInterner<'db>> for Safety {
    fn relate<R: rodetc_type_ir::relate::TypeRelation<DbInterner<'db>>>(
        _relation: &mut R,
        a: Self,
        b: Self,
    ) -> rodetc_type_ir::relate::RelateResult<DbInterner<'db>, Self> {
        if a != b {
            Err(TypeError::SafetyMismatch(rodetc_type_ir::error::ExpectedFound::new(a, b)))
        } else {
            Ok(a)
        }
    }
}

impl<'db> rodetc_type_ir::inherent::Safety<DbInterner<'db>> for Safety {
    fn safe() -> Self {
        Self::Safe
    }

    fn is_safe(self) -> bool {
        matches!(self, Safety::Safe)
    }

    fn prefix_str(self) -> &'static str {
        match self {
            Self::Unsafe => "unsafe ",
            Self::Safe => "",
        }
    }

    fn unsafe_mode() -> Self {
        Safety::Unsafe
    }
}

impl<'db> TypeVisitable<DbInterner<'db>> for ExternAbi {
    fn visit_with<V: TypeVisitor<DbInterner<'db>>>(&self, _visitor: &mut V) -> V::Result {
        V::Result::output()
    }
}

impl<'db> TypeFoldable<DbInterner<'db>> for ExternAbi {
    fn try_fold_with<F: FallibleTypeFolder<DbInterner<'db>>>(
        self,
        _folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(self)
    }

    fn fold_with<F: TypeFolder<DbInterner<'db>>>(self, _folder: &mut F) -> Self {
        self
    }
}

impl<'db> Relate<DbInterner<'db>> for ExternAbi {
    fn relate<R: rodetc_type_ir::relate::TypeRelation<DbInterner<'db>>>(
        _relation: &mut R,
        a: Self,
        b: Self,
    ) -> rodetc_type_ir::relate::RelateResult<DbInterner<'db>, Self> {
        if a == b {
            Ok(a)
        } else {
            Err(TypeError::AbiMismatch(rodetc_type_ir::error::ExpectedFound::new(a, b)))
        }
    }
}
