window.DETERMINISTIC_REUNION_FALLBACK = {
  "deterministic-reunion-clean": {
    "id": "deterministic-reunion-clean",
    "title": "1. Clean reunion",
    "summary": "Both islands drift only slightly. Reunion still performs a merge, but the allowed precedence rules resolve the differences without residue.",
    "difference_from_quorum_lab": "Quorum Lab asks when hidden capability may become observable. Deterministic Reunion Lab asks what happens when two locally coherent membership realities meet again.",
    "notes": [
      "A clean reunion is still a merge, not a magical return to innocence.",
      "The point of the scenario is that deterministic reunion can be quiet without being naive."
    ],
    "allow_operator_override": false,
    "operator_override": null,
    "expected_merge_tensions": [
      "Lumen is accepted on Island B before Island A closes the same witness gap.",
      "Nyx is fresher on Island B but does not materially conflict with Island A."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 31,
        "members": {
          "iona": { "status": "accepted", "epoch": 31, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Seen before the split and reaffirmed by two scopes." },
          "kestrel": { "status": "accepted", "epoch": 31, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Stable trustee membership carried forward cleanly." },
          "lumen": { "status": "provisional", "epoch": 30, "trust_weight": 68, "scope": "regional-a", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Witness gap remained open when the partition landed." },
          "morrow": { "status": "accepted", "epoch": 31, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Locally calm but not over-corroborated." },
          "nyx": { "status": "accepted", "epoch": 31, "trust_weight": 77, "scope": "regional-a", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Operationally present, global corroboration still catching up." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 31,
        "members": {
          "iona": { "status": "accepted", "epoch": 31, "trust_weight": 87, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Same direction as Island A." },
          "kestrel": { "status": "accepted", "epoch": 31, "trust_weight": 82, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "No material split on Kestrel." },
          "lumen": { "status": "accepted", "epoch": 31, "trust_weight": 83, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Island B closed the remaining witness gap." },
          "morrow": { "status": "accepted", "epoch": 31, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm local continuity." },
          "nyx": { "status": "provisional", "epoch": 30, "trust_weight": 66, "scope": "regional-b", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "single-scope" }, "explanation": "Fresh enough to continue service, not enough to close globally." }
        }
      }
    },
    "events": [
      {
        "id": "close-lumen-gap",
        "label": "Island A closes the Lumen witness gap",
        "description": "Island A gets the same missing corroboration Island B already had.",
        "patches": {
          "island_a": {
            "members": {
              "lumen": { "status": "accepted", "epoch": 31, "trust_weight": 85, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Cross-scope corroboration arrives before reunion." }
            },
            "log": "Island A accepts Lumen after the missing corroboration arrives."
          }
        }
      },
      {
        "id": "freshen-nyx",
        "label": "Island B freshens Nyx",
        "description": "Island B closes the same local gap Island A had already tolerated.",
        "patches": {
          "island_b": {
            "members": {
              "nyx": { "status": "accepted", "epoch": 31, "trust_weight": 82, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "A late cross-scope witness closes the remaining doubt." }
            },
            "log": "Island B upgrades Nyx from provisional to accepted."
          }
        }
      }
    ]
  },
  "deterministic-reunion-conflicting-acceptance": {
    "id": "deterministic-reunion-conflicting-acceptance",
    "title": "3. Conflicting acceptance",
    "summary": "Both islands are locally coherent and materially incompatible. Reunion must preserve the conflict instead of flattening it into certainty.",
    "difference_from_quorum_lab": "This artifact is about divergent belief merge. The hard question is not whether a quorum exists, but what a reunited system is willing to remember.",
    "notes": [
      "The conflict is fresh, admissible, and same-epoch on both sides.",
      "This is the canonical residue case."
    ],
    "allow_operator_override": false,
    "operator_override": null,
    "expected_merge_tensions": [
      "Island A accepted Nyx with strong cross-scope support.",
      "Island B quarantined Nyx with equally fresh equivocation evidence."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 58,
        "members": {
          "iona": { "status": "accepted", "epoch": 58, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 58, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "accepted", "epoch": 58, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 58, "trust_weight": 78, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "provisional", "epoch": 57, "trust_weight": 71, "scope": "regional-a", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Nyx was already under active review before the split." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 58,
        "members": {
          "iona": { "status": "accepted", "epoch": 58, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 58, "trust_weight": 83, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "accepted", "epoch": 58, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 58, "trust_weight": 78, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "provisional", "epoch": 57, "trust_weight": 72, "scope": "regional-b", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Nyx was already under active review before the split." }
        }
      }
    },
    "events": [
      {
        "id": "accept-nyx",
        "label": "Island A accepts Nyx",
        "description": "Island A sees enough fresh cross-scope evidence to close the acceptance path.",
        "patches": {
          "island_a": {
            "members": {
              "nyx": { "status": "accepted", "epoch": 58, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Island A sees a clean acceptance path." }
            },
            "log": "Island A accepts Nyx as globally admissible."
          }
        }
      },
      {
        "id": "quarantine-nyx",
        "label": "Island B quarantines Nyx",
        "description": "Island B sees equally fresh equivocation evidence from a high-trust witness set.",
        "patches": {
          "island_b": {
            "members": {
              "nyx": { "status": "quarantined", "epoch": 58, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Equivocation evidence is strong enough to quarantine immediately." }
            },
            "log": "Island B quarantines Nyx after an equivocation report."
          }
        }
      }
    ]
  },
  "deterministic-reunion-epoch-race": {
    "id": "deterministic-reunion-epoch-race",
    "title": "6. Epoch mismatch and revocation race",
    "summary": "Reunion occurs across an epoch boundary while a revocation races a still-coherent acceptance path. Connectivity returns, but truth does not rewind.",
    "difference_from_quorum_lab": "The critical question here is not ceremony admission. It is what a reunited system should do when fresh revocation collides with slightly older acceptance.",
    "notes": [
      "This scenario teaches that newer revocation can dominate while still leaving a visible race scar.",
      "Recontact does not erase the earlier acceptance history."
    ],
    "allow_operator_override": false,
    "operator_override": null,
    "expected_merge_tensions": [
      "Island B still carries a coherent acceptance for Iona at epoch 83.",
      "Island A receives a stronger revocation at epoch 84 just before reunion."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 83,
        "members": {
          "iona": { "status": "accepted", "epoch": 83, "trust_weight": 81, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Iona is stable before the revocation arrives." },
          "kestrel": { "status": "accepted", "epoch": 83, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "accepted", "epoch": 83, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 83, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 83, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 83,
        "members": {
          "iona": { "status": "accepted", "epoch": 83, "trust_weight": 82, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Iona remains coherent on Island B." },
          "kestrel": { "status": "accepted", "epoch": 83, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "accepted", "epoch": 83, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 83, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 83, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      }
    },
    "events": [
      {
        "id": "still-accepted-b",
        "label": "Island B keeps Iona accepted",
        "description": "Island B does not yet see the revocation and continues with a coherent acceptance path.",
        "patches": {
          "island_b": {
            "log": "Island B continues to treat Iona as accepted at epoch 83."
          }
        }
      },
      {
        "id": "revocation-arrives-a",
        "label": "Island A receives a newer revocation for Iona",
        "description": "The revocation lands one epoch later with stronger admissible evidence.",
        "patches": {
          "island_a": {
            "local_epoch": 84,
            "members": {
              "iona": { "status": "removed", "epoch": 84, "trust_weight": 85, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Revocation arrives with newer, stronger evidence." }
            },
            "log": "Island A removes Iona after the revocation lands in epoch 84."
          }
        }
      }
    ]
  },
  "deterministic-reunion-operator-override": {
    "id": "deterministic-reunion-operator-override",
    "title": "5. Visible residue and operator intervention",
    "summary": "Both islands keep a locally coherent position that remains materially unresolved after deterministic merge. The system must surface residue and allow a constrained operator intervention.",
    "difference_from_quorum_lab": "Quorum Lab is about collective presence before revelation. This lab is about what happens after a split when even good merge rules still cannot honestly finish the job alone.",
    "notes": [
      "OperatorOverride is not magic. It is visible, constrained, and leaves a scar in the digest.",
      "The merge remains honest by refusing to flatten this conflict automatically."
    ],
    "allow_operator_override": true,
    "operator_override": {
      "subject_id": "kestrel",
      "status": "quarantined",
      "reason": "Global removal and regional acceptance remain materially unresolved at the same epoch. Quarantine is the least dishonest intervention."
    },
    "expected_merge_tensions": [
      "Island A kept Kestrel active in a regional scope after local repair.",
      "Island B removed Kestrel globally after a policy violation outside Island A's visibility."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 71,
        "members": {
          "iona": { "status": "accepted", "epoch": 71, "trust_weight": 87, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 70, "trust_weight": 78, "scope": "regional-a", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Still active regionally before the later conflict lands." },
          "lumen": { "status": "accepted", "epoch": 71, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 71, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 71, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 71,
        "members": {
          "iona": { "status": "accepted", "epoch": 71, "trust_weight": 87, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 70, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Still active before the violation lands." },
          "lumen": { "status": "accepted", "epoch": 71, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "morrow": { "status": "accepted", "epoch": 71, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 71, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      }
    },
    "events": [
      {
        "id": "regional-keepalive",
        "label": "Island A keeps Kestrel active regionally",
        "description": "A local repair path convinces Island A to keep Kestrel alive in its scope.",
        "patches": {
          "island_a": {
            "members": {
              "kestrel": { "status": "accepted", "epoch": 71, "trust_weight": 81, "scope": "regional-a", "witness_summary": { "count": 2, "quality": "strong", "diversity": "mixed" }, "explanation": "Regional repair keeps Kestrel online inside Island A." }
            },
            "log": "Island A keeps Kestrel active in a regional scope."
          }
        }
      },
      {
        "id": "global-removal",
        "label": "Island B removes Kestrel globally",
        "description": "Island B sees a policy violation outside Island A's visibility and revokes Kestrel.",
        "patches": {
          "island_b": {
            "members": {
              "kestrel": { "status": "removed", "epoch": 71, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "mixed" }, "explanation": "Global scope issues removal, but the witness path is still thin enough that reunion cannot finish the case honestly on its own." }
            },
            "log": "Island B removes Kestrel globally."
          }
        }
      }
    ]
  },
  "deterministic-reunion-stale-witness": {
    "id": "deterministic-reunion-stale-witness",
    "title": "2. Stale witness path",
    "summary": "One island accepts a member through an older and weaker witness path. Reunion should not mistake that for current shared truth.",
    "difference_from_quorum_lab": "Quorum Lab is about plural presence before revelation. This lab is about what survives comparison when two witness histories recontact one another.",
    "notes": [
      "The stale path is locally coherent enough to fool a naive reunion.",
      "The deterministic merge should preserve the downgrade and explain why."
    ],
    "allow_operator_override": false,
    "operator_override": null,
    "expected_merge_tensions": [
      "Island B accepted Morrow based on an older epoch and weaker witness diversity.",
      "Island A only reaches provisional status with fresher evidence."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 42,
        "members": {
          "iona": { "status": "accepted", "epoch": 42, "trust_weight": 87, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Steady gateway membership." },
          "kestrel": { "status": "accepted", "epoch": 42, "trust_weight": 83, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "No meaningful drift on Kestrel." },
          "lumen": { "status": "accepted", "epoch": 42, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Lumen stays calm through the split." },
          "morrow": { "status": "unknown", "epoch": 42, "trust_weight": 0, "scope": "regional-a", "witness_summary": { "count": 0, "quality": "weak", "diversity": "single-scope" }, "explanation": "No fresh admissible evidence yet." },
          "nyx": { "status": "accepted", "epoch": 42, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Operationally stable." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 42,
        "members": {
          "iona": { "status": "accepted", "epoch": 42, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "No drift on Iona." },
          "kestrel": { "status": "accepted", "epoch": 42, "trust_weight": 82, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "No drift on Kestrel." },
          "lumen": { "status": "accepted", "epoch": 42, "trust_weight": 78, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Lumen remains settled." },
          "morrow": { "status": "unknown", "epoch": 42, "trust_weight": 0, "scope": "regional-b", "witness_summary": { "count": 0, "quality": "weak", "diversity": "single-scope" }, "explanation": "No fresh admissible evidence yet." },
          "nyx": { "status": "accepted", "epoch": 42, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Operationally stable." }
        }
      }
    },
    "events": [
      {
        "id": "stale-acceptance",
        "label": "Island B accepts Morrow through a stale witness path",
        "description": "The evidence is old and narrow, but enough for Island B to accept locally during the split.",
        "patches": {
          "island_b": {
            "members": {
              "morrow": { "status": "accepted", "epoch": 40, "trust_weight": 58, "scope": "regional-b", "witness_summary": { "count": 3, "quality": "weak", "diversity": "single-scope" }, "explanation": "Three same-scope witnesses reinforce an older introduction." }
            },
            "log": "Island B accepts Morrow based on an older witness path."
          }
        }
      },
      {
        "id": "fresh-provisional",
        "label": "Island A reaches only provisional confidence on Morrow",
        "description": "Island A gets fresher evidence, but not enough to close the path all the way to accepted.",
        "patches": {
          "island_a": {
            "members": {
              "morrow": { "status": "provisional", "epoch": 42, "trust_weight": 74, "scope": "global", "witness_summary": { "count": 2, "quality": "strong", "diversity": "cross-scope" }, "explanation": "The evidence is fresher and better, but still incomplete." }
            },
            "log": "Island A keeps Morrow provisional pending one more independent witness."
          }
        }
      }
    ]
  },
  "deterministic-reunion-trust-laundering": {
    "id": "deterministic-reunion-trust-laundering",
    "title": "4. Trust laundering attempt",
    "summary": "One island accumulates a large but low-quality reinforcing witness cluster. Reunion should not mistake raw count for trustworthy convergence.",
    "difference_from_quorum_lab": "This artifact is about merge semantics after divergence. The relevant question is whether corroboration quality can dominate count during repair.",
    "notes": [
      "The danger is not just disagreement. The danger is counterfeit agreement.",
      "This scenario exists to make witness laundering feel operationally obvious."
    ],
    "allow_operator_override": false,
    "operator_override": null,
    "expected_merge_tensions": [
      "Island B has more witnesses on Lumen, but they are low-quality and same-scope.",
      "Island A has fewer witnesses, but the set is better and more diverse."
    ],
    "subjects": [
      { "id": "iona", "label": "Iona", "role": "gateway", "domain": "north" },
      { "id": "kestrel", "label": "Kestrel", "role": "trustee", "domain": "north" },
      { "id": "lumen", "label": "Lumen", "role": "relay", "domain": "south" },
      { "id": "morrow", "label": "Morrow", "role": "witness", "domain": "west" },
      { "id": "nyx", "label": "Nyx", "role": "operator", "domain": "south" }
    ],
    "initial": {
      "policy_name": "Deterministic Reunion Baseline",
      "policy_summary": "Fresh admissible evidence dominates stale, trust weight and corroboration quality dominate raw count, and material conflict survives as residue.",
      "island_a": {
        "label": "Island A",
        "scope": "regional-a",
        "local_epoch": 64,
        "members": {
          "iona": { "status": "accepted", "epoch": 64, "trust_weight": 87, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 64, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "unknown", "epoch": 64, "trust_weight": 0, "scope": "regional-a", "witness_summary": { "count": 0, "quality": "weak", "diversity": "single-scope" }, "explanation": "No fresh admissible view yet." },
          "morrow": { "status": "accepted", "epoch": 64, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 64, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      },
      "island_b": {
        "label": "Island B",
        "scope": "regional-b",
        "local_epoch": 64,
        "members": {
          "iona": { "status": "accepted", "epoch": 64, "trust_weight": 86, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "kestrel": { "status": "accepted", "epoch": 64, "trust_weight": 84, "scope": "global", "witness_summary": { "count": 3, "quality": "strong", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "lumen": { "status": "unknown", "epoch": 64, "trust_weight": 0, "scope": "regional-b", "witness_summary": { "count": 0, "quality": "weak", "diversity": "single-scope" }, "explanation": "No fresh admissible view yet." },
          "morrow": { "status": "accepted", "epoch": 64, "trust_weight": 79, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." },
          "nyx": { "status": "accepted", "epoch": 64, "trust_weight": 80, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "Calm continuity." }
        }
      }
    },
    "events": [
      {
        "id": "laundered-acceptance",
        "label": "Island B accepts Lumen through a laundered witness cluster",
        "description": "Many low-trust witnesses reinforce each other inside one scope.",
        "patches": {
          "island_b": {
            "members": {
              "lumen": { "status": "accepted", "epoch": 64, "trust_weight": 57, "scope": "regional-b", "witness_summary": { "count": 6, "quality": "weak", "diversity": "laundered" }, "explanation": "Six low-trust witnesses amplify the same narrow path." }
            },
            "log": "Island B treats a reinforcing local cluster as sufficient for acceptance."
          }
        }
      },
      {
        "id": "better-provisional",
        "label": "Island A reaches a better but still provisional view",
        "description": "Island A has fewer witnesses, but they are better and more diverse.",
        "patches": {
          "island_a": {
            "members": {
              "lumen": { "status": "provisional", "epoch": 64, "trust_weight": 72, "scope": "global", "witness_summary": { "count": 2, "quality": "mixed", "diversity": "cross-scope" }, "explanation": "The path is better, but still not closed enough for accepted." }
            },
            "log": "Island A refuses to accept Lumen on raw count alone."
          }
        }
      }
    ]
  }
};
