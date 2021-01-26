/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfAddOffsetsToTxnRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfAddOffsetsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    pub transactional_id: String,

    /// Current producer id in use by the transactional id.
    pub producer_id: i64,

    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,

    /// The unique group identifier.
    pub group_id: String,
}

// -----------------------------------
// KfAddOffsetsToTxnResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfAddOffsetsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or
    /// zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The response error code, or 0 if there was no error.
    pub error_code: ErrorCode,
}

// -----------------------------------
// Implementation - KfAddOffsetsToTxnRequest
// -----------------------------------

impl Request for KfAddOffsetsToTxnRequest {
    const API_KEY: u16 = 25;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 1;
    const DEFAULT_API_VERSION: i16 = 1;

    type Response = KfAddOffsetsToTxnResponse;
}