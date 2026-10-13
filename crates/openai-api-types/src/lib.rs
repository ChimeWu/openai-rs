//! Protocol types generated from the pinned OpenAI OpenAPI snapshot.
//!
//! API family modules are opt-in. Enable `full` to compile every generated
//! family and component schema, or enable a family feature for a smaller build.

#![forbid(unsafe_code)]

extern crate self as openai_api_types;

mod generated;

pub use generated::*;
