//! Scopes and scope authority.
//!
//! PINNED (P10, closes SPEC_AUDIT High #2 "scope semantics vs topology
//! policy still partially entangled"): the kernel contains no topology
//! types at all. Scope owns *meaning* (`ScopeId`, `ScopeAuthority` in the
//! merge); topology owns *eligibility* and enters only as caller-provided
//! candidate pools with exclusion reasons (`rank` module). Topology
//! literally cannot influence claim meaning because the kernel has no
//! vocabulary for it.

use crate::id::{write_str, CanonicalBytes};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A named boundary of shared meaning. Scoped belief is always keyed by
/// (scope, subject); a claim's scope is part of its meaning.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScopeId(String);

impl ScopeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ScopeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl CanonicalBytes for ScopeId {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        write_str(out, &self.0);
    }
}

/// How much dominance weight a claim's scope carries during merge.
/// Ordered: Local < Regional < Global.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ScopeAuthority {
    Local,
    Regional,
    Global,
}

impl fmt::Display for ScopeAuthority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ScopeAuthority::Local => "local",
            ScopeAuthority::Regional => "regional",
            ScopeAuthority::Global => "global",
        };
        f.write_str(s)
    }
}
