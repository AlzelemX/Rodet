//@ compile-flags: -Z unstable-options
#![feature(rustc_private)]
#![deny(rustc::non_glob_import_of_type_ir_inherent)]

extern crate rodetc_type_ir;

mod ok {
    use rodetc_type_ir::inherent::*; // OK
    use rodetc_type_ir::inherent::{}; // OK
    use rodetc_type_ir::inherent::{*}; // OK

    fn usage<T: rodetc_type_ir::inherent::SliceLike>() {} // OK
}

mod direct {
    use rodetc_type_ir::inherent::Predicate; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
    use rodetc_type_ir::inherent::{AdtDef, Ty};
    //~^ ERROR non-glob import of `rodetc_type_ir::inherent`
    //~| ERROR non-glob import of `rodetc_type_ir::inherent`
    use rodetc_type_ir::inherent::ParamEnv as _; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
}

mod indirect0 {
    use rodetc_type_ir::inherent; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
    use rodetc_type_ir::inherent as inh; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
    use rodetc_type_ir::{inherent as _}; //~ ERROR non-glob import of `rodetc_type_ir::inherent`

    fn usage0<T: inherent::SliceLike>() {}
    fn usage1<T: inh::SliceLike>() {}
}

mod indirect1 {
    use rodetc_type_ir::inherent::{self}; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
    use rodetc_type_ir::inherent::{self as innate}; //~ ERROR non-glob import of `rodetc_type_ir::inherent`
}

fn main() {}
