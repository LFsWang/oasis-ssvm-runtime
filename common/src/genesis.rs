//! Genesis state.
use std::io::Cursor;
use std::path::Path;

use spec::Spec;
use spec::SpecParams;
use ethereum_types::U256;
use lazy_static::lazy_static;

use crate::BLOCK_GAS_LIMIT;

lazy_static! {
    /// Block gas limit.
    pub static ref GAS_LIMIT: U256 = U256::from(BLOCK_GAS_LIMIT);

    /// Genesis spec.
    pub static ref SPEC: Spec = {
        #[cfg(all(feature = "production-genesis", feature = "benchmarking"))]
        compile_error!("Cannot use \"production-genesis\" and \"benchmarking\" features together!");

        #[cfg(feature = "production-genesis")]
        let spec_json = include_str!("../../resources/genesis/genesis.json");

        #[cfg(feature = "benchmarking")]
        let spec_json = include_str!("../../resources/genesis/genesis_benchmarking.json");

        #[cfg(not(feature = "benchmarking"))]
        let spec_json = include_str!("../../resources/genesis/genesis_testing.json");

        // FIXME Path::new("/tmp/")?
        Spec::load( SpecParams::from_path(Path::new("/tmp/"))  ,Cursor::new(spec_json)).expect("must have a valid genesis spec")
    };
}