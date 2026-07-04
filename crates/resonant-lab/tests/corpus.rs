//! Layer 1: corpus integrity. The six scenario JSONs are the closed
//! conformance corpus; any drift fails loudly here before semantics are
//! even evaluated.

use resonant_lab::scenario::{default_scenarios_dir, load_index, load_scenario};

#[test]
fn corpus_parses_and_is_internally_consistent() {
    let dir = default_scenarios_dir();
    let index = load_index(&dir).expect("index.json parses");
    assert_eq!(index.lab, "deterministic-reunion-lab");
    assert_eq!(
        index.scenarios.len(),
        6,
        "the corpus is exactly six scenarios"
    );

    for entry in &index.scenarios {
        let scenario = load_scenario(&dir, &entry.path)
            .unwrap_or_else(|e| panic!("{} should parse: {e}", entry.path));

        assert_eq!(scenario.id, entry.id, "scenario id matches index id");
        assert_eq!(
            entry.path,
            format!("{}.json", scenario.id),
            "path is the id-stem convention"
        );
        assert_eq!(
            scenario.allow_operator_override,
            scenario.operator_override.is_some(),
            "{}: override presence must match the allow flag",
            scenario.id
        );

        let subject_ids: Vec<&str> = scenario.subjects.iter().map(|s| s.id.as_str()).collect();
        for island in [&scenario.initial.island_a, &scenario.initial.island_b] {
            for member in island.members.keys() {
                assert!(
                    subject_ids.contains(&member.as_str()),
                    "{}: initial member {member} is not a declared subject",
                    scenario.id
                );
            }
        }
        for event in &scenario.events {
            for patch in [&event.patches.island_a, &event.patches.island_b]
                .into_iter()
                .flatten()
            {
                for member in patch.members.iter().flat_map(|m| m.keys()) {
                    assert!(
                        subject_ids.contains(&member.as_str()),
                        "{}: event {} patches unknown subject {member}",
                        scenario.id,
                        event.id
                    );
                }
            }
        }
        if let Some(op) = &scenario.operator_override {
            assert!(
                subject_ids.contains(&op.subject_id.as_str()),
                "{}: override targets unknown subject",
                scenario.id
            );
        }
    }
}
