//! Repeated constants used around pathfinder

use crate::core::StarknetBlockHash;

/// User agent used in http clients
pub const USER_AGENT: &str = concat!(
    "starknet-pathfinder/",
    env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT")
);

pub const TESTNET_GENESIS_HASH: StarknetBlockHash = StarknetBlockHash(crate::starkhash!(
    "07d328a71faf48c5c3857e99f20a77b18522480956d1cd5bff1ff2df3c8b427b"
));

pub const MAINNET_GENESIS_HASH: StarknetBlockHash = StarknetBlockHash(crate::starkhash!(
    "047C3637B57C2B079B93C61539950C17E868A28F46CDEF28F88521067F21E943"
));

pub const INTEGRATION_GENESIS_HASH: StarknetBlockHash = StarknetBlockHash(crate::starkhash!(
    "03ae41b0f023e53151b0c8ab8b9caafb7005d5f41c9ab260276d5bdc49726279"
));
