//! Witness-summary derivation (P19): quality from observation modes,
//! diversity from vouch-lineage roots, laundering from loud single-lineage
//! clusters — and the advancement gate that makes sockpuppets fail.

use resonant_kernel::epoch::Epoch;
use resonant_kernel::evidence::{
    AssertedState, Claim, Diversity, Observation, ObservationMode, Provenance, Quality, Stance,
    WitnessRecord,
};
use resonant_kernel::id::{PeerId, SubjectId, WitnessId};
use resonant_kernel::scope::ScopeId;
use resonant_kernel::trust::TrustGrade;
use resonant_kernel::witnessing::{clears_advancement_gate, EvidenceBook};

fn scope() -> ScopeId {
    ScopeId::new("room:test")
}

fn join_claim(subject: &str, voucher: &str) -> Claim {
    Claim {
        subject: SubjectId::new(subject),
        asserted: AssertedState::Present,
        scope: scope(),
        provenance: Provenance {
            introducer: PeerId::new(voucher),
        },
        epoch: Epoch(1),
        evidence: vec![],
    }
}

fn witness(by: &str, subject: &str, mode: ObservationMode) -> WitnessRecord {
    let observation = Observation {
        observer: WitnessId::new(by),
        subject: SubjectId::new(subject),
        mode,
        epoch: Epoch(1),
    };
    WitnessRecord {
        witness: WitnessId::new(by),
        subject: SubjectId::new(subject),
        about: None,
        stance: Stance::Corroborate,
        observation: observation.id(),
        mode,
        scope: scope(),
        epoch: Epoch(1),
        trust_context: TrustGrade::new(70),
    }
}

/// Two strong witnesses from distinct vouch lineages beat six weak ones
/// from a single lineage — the trust-laundering shape, derived from raw
/// evidence instead of asserted.
#[test]
fn laundering_is_derived_not_asserted() {
    let mut book = EvidenceBook::new();
    let subject = SubjectId::new("eve");

    // Honest side: bob was vouched by alice, carol by dave — two roots.
    for claim in [
        join_claim("bob", "alice"),
        join_claim("carol", "dave"),
        join_claim("eve", "mallory"),
    ] {
        book.record_claim(&claim);
    }
    book.record_witness(witness("bob", "hon", ObservationMode::DirectContact));
    book.record_witness(witness("carol", "hon", ObservationMode::ChallengeResponse));
    let honest = book.summarize(&scope(), &SubjectId::new("hon"));
    assert_eq!(honest.count, 2);
    assert_eq!(honest.quality, Quality::Strong);
    assert_eq!(
        honest.diversity,
        Diversity::Mixed,
        "two distinct lineage roots"
    );
    assert!(clears_advancement_gate(&honest));

    // Sockpuppet side: mallory vouched p1..p4, and all of them "witness"
    // eve via timeouts and topology hearsay.
    for puppet in ["p1", "p2", "p3", "p4"] {
        book.record_claim(&join_claim(puppet, "mallory"));
    }
    for (puppet, mode) in [
        ("p1", ObservationMode::Timeout),
        ("p2", ObservationMode::TopologyEvidence),
        ("p3", ObservationMode::Timeout),
        ("p4", ObservationMode::TopologyEvidence),
    ] {
        book.record_witness(witness(puppet, "eve", mode));
    }
    let laundered = book.summarize(&scope(), &subject);
    assert_eq!(laundered.count, 4, "louder");
    assert_eq!(laundered.quality, Quality::Weak);
    assert_eq!(
        laundered.diversity,
        Diversity::Laundered,
        "single lineage root behind a loud weak cluster"
    );
    assert!(
        !clears_advancement_gate(&laundered),
        "the advancement gate refuses laundered corroboration however loud it is"
    );
}

/// Lineage roots follow vouch chains, not direct vouchers: a puppet vouched
/// by a puppet still traces to the original operator.
#[test]
fn lineage_follows_chains() {
    let mut book = EvidenceBook::new();
    book.record_claim(&join_claim("p1", "mallory"));
    book.record_claim(&join_claim("p2", "p1"));
    book.record_claim(&join_claim("p3", "p2"));
    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("p3")),
        WitnessId::new("mallory")
    );
    // A root with no recorded voucher is its own lineage root.
    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("mallory")),
        WitnessId::new("mallory")
    );
}

/// Climbing stops below trust roots: in a room where every chain reaches
/// the creator, members vouched directly by the creator are independent
/// lineages, while a clique vouched through one member is a single lineage.
#[test]
fn lineage_stops_below_trust_roots() {
    let mut book = EvidenceBook::new();
    book.mark_trust_root(&scope(), WitnessId::new("alice"));
    // bob and mallory both vouched by the creator; puppets by mallory.
    book.record_claim(&join_claim("bob", "alice"));
    book.record_claim(&join_claim("mallory", "alice"));
    book.record_claim(&join_claim("p1", "mallory"));
    book.record_claim(&join_claim("p2", "p1"));

    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("bob")),
        WitnessId::new("bob")
    );
    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("mallory")),
        WitnessId::new("mallory")
    );
    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("p2")),
        WitnessId::new("mallory")
    );
    // The creator is its own root.
    assert_eq!(
        book.lineage_root(&scope(), &WitnessId::new("alice")),
        WitnessId::new("alice")
    );
}

/// Three or more independent lineages grade as cross-scope corroboration.
#[test]
fn diversity_scales_with_roots() {
    let mut book = EvidenceBook::new();
    for (member, voucher) in [("a1", "rootA"), ("b1", "rootB"), ("c1", "rootC")] {
        book.record_claim(&join_claim(member, voucher));
    }
    for by in ["a1", "b1", "c1"] {
        book.record_witness(witness(by, "subject", ObservationMode::DirectContact));
    }
    let summary = book.summarize(&scope(), &SubjectId::new("subject"));
    assert_eq!(summary.diversity, Diversity::CrossScope);
    assert_eq!(summary.quality, Quality::Strong);
}

/// No evidence at all summarizes honestly: zero count, weak, single-scope.
#[test]
fn empty_book_is_honest() {
    let book = EvidenceBook::new();
    let summary = book.summarize(&scope(), &SubjectId::new("ghost"));
    assert_eq!(summary.count, 0);
    assert_eq!(summary.quality, Quality::Weak);
    assert_eq!(summary.diversity, Diversity::SingleScope);
    assert!(!clears_advancement_gate(&summary));
}
