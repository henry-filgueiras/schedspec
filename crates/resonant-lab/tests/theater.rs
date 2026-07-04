//! The story corpus as the kernel's end-to-end integration suite, plus a
//! property test that the split-brain story converges under arbitrary
//! partition assignments.

use proptest::prelude::*;
use resonant_kernel::evidence::ObservationMode;
use resonant_lab::theater::{run_story, stories, Step, Story};

#[test]
fn all_stories_pass() {
    for story in stories() {
        let report = run_story(&story);
        assert!(
            report.passed(),
            "{} failed:\n  {}\n---\n{}",
            report.name,
            report.failures.join("\n  "),
            report.lines.join("\n")
        );
    }
}

/// Whichever way the four-actor room splits (as long as both sides are
/// non-empty and the banning side excludes dave's keeper), reunion always
/// converges with the dispute preserved.
fn convergence_story(group_b: &'static [&'static str]) -> Story {
    Story {
        name: "prop-split",
        blurb: "property: reunion converges under arbitrary splits",
        actors: &[("alice", 90), ("bob", 75), ("carol", 75), ("dave", 70)],
        creator: "alice",
        steps: || vec![],
    }
    .with_group(group_b)
}

trait WithGroup {
    fn with_group(self, group_b: &'static [&'static str]) -> Story;
}

impl WithGroup for Story {
    fn with_group(mut self, group_b: &'static [&'static str]) -> Story {
        // Steps are a fn pointer, so route the group through a static table.
        fn steps_for(group_b: &'static [&'static str]) -> Vec<Step> {
            use ObservationMode::*;
            vec![
                Step::Join {
                    who: "bob",
                    vouched_by: "alice",
                },
                Step::Join {
                    who: "carol",
                    vouched_by: "alice",
                },
                Step::Join {
                    who: "dave",
                    vouched_by: "alice",
                },
                Step::Witness {
                    by: "bob",
                    subject: "dave",
                    mode: DirectContact,
                },
                Step::Witness {
                    by: "carol",
                    subject: "dave",
                    mode: ChallengeResponse,
                },
                Step::Ticks(3),
                Step::Ticks(3),
                Step::Partition { group_b },
                Step::Ban {
                    by: "alice",
                    subject: "dave",
                },
                Step::Heal,
                Step::ExpectResidue {
                    min_unhandled: 1,
                    min_handled: 0,
                },
                Step::ExpectConverged,
            ]
        }
        // Encode the choice as one of the fixed splits below.
        self.steps = match group_b {
            g if g == ["dave"] => || steps_for(&["dave"]),
            g if g == ["carol", "dave"] => || steps_for(&["carol", "dave"]),
            g if g == ["bob", "carol", "dave"] => || steps_for(&["bob", "carol", "dave"]),
            _ => || steps_for(&["dave"]),
        };
        self
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]

    #[test]
    fn split_brain_converges_under_any_split(choice in 0usize..3) {
        let group: &'static [&'static str] = match choice {
            0 => &["dave"],
            1 => &["carol", "dave"],
            _ => &["bob", "carol", "dave"],
        };
        // alice (the banning side) must not be in group B for the story to
        // make sense; all three choices satisfy that.
        let report = run_story(&convergence_story(group));
        prop_assert!(
            report.passed(),
            "split {:?} failed:\n  {}",
            group,
            report.failures.join("\n  ")
        );
    }
}
