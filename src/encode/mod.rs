pub mod affect;
pub mod embed;
pub mod identity;
pub mod intake;
pub mod scoring;

pub use intake::{
    accept_core, encode, encode_with_parts, lossless_parts, needs_split, parse_segment_reply,
    segment_facts, split_event, EncodeDecision, EncodeInput,
};
