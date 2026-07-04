//! Identity newtypes and canonical byte encoding.
//!
//! PINNED (P8/P10 support): every actor kind gets its own newtype so the type
//! system enforces PRIMITIVES.md's WitnessRecord invariant that "the witness
//! must be distinguished from the subject being discussed" — a `WitnessId`
//! cannot be passed where a `SubjectId` is expected. A node playing both
//! roles holds both ids.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Deterministic byte encoding for hashing. Field order is fixed by each
/// implementation; strings are length-prefixed so concatenation is
/// unambiguous. This is the only encoding used on determinism-critical
/// hash paths (ids, rank seeds); serde is reserved for interchange.
pub trait CanonicalBytes {
    fn write_canonical(&self, out: &mut Vec<u8>);

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.write_canonical(&mut out);
        out
    }

    /// Domain-separated BLAKE3 digest of the canonical encoding.
    fn digest(&self, domain: &str) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&(domain.len() as u64).to_le_bytes());
        hasher.update(domain.as_bytes());
        hasher.update(&self.canonical_bytes());
        *hasher.finalize().as_bytes()
    }
}

pub(crate) fn write_str(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u64).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

impl CanonicalBytes for str {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        write_str(out, self);
    }
}

impl CanonicalBytes for u64 {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
    }
}

macro_rules! name_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl CanonicalBytes for $name {
            fn write_canonical(&self, out: &mut Vec<u8>) {
                write_str(out, &self.0);
            }
        }
    };
}

name_id!(
    /// A stable referent whose membership is under discussion.
    SubjectId
);
name_id!(
    /// An actor producing observations and witness records.
    WitnessId
);
name_id!(
    /// A candidate for rank-based selection (fanout, rendezvous, witness sets).
    PeerId
);
name_id!(
    /// A named trust root whose standing is always scoped.
    TrustRootId
);
name_id!(
    /// A human or automated operator issuing visible overrides.
    OperatorId
);

macro_rules! hash_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name([u8; 32]);

        impl $name {
            pub fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            pub fn short_hex(&self) -> String {
                self.0[..6].iter().map(|b| format!("{b:02x}")).collect()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.short_hex())
            }
        }

        impl CanonicalBytes for $name {
            fn write_canonical(&self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.0);
            }
        }
    };
}

hash_id!(
    /// Content hash of a claim's canonical bytes.
    ClaimId
);
hash_id!(
    /// Content hash of a witness record's canonical bytes.
    WitnessRecordId
);
hash_id!(
    /// Content hash of a local observation.
    ObservationId
);
hash_id!(
    /// Identity of a preserved piece of unresolved disagreement.
    ResidueId
);
hash_id!(
    /// Identity of an operator override, so an override can always be
    /// distinguished from organic convergence.
    OverrideId
);
