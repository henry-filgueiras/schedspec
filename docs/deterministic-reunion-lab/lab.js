(function () {
  const QUALITY_SCORES = { strong: 22, mixed: 10, weak: -8 };
  const DIVERSITY_SCORES = {
    "cross-scope": 14,
    mixed: 6,
    "single-scope": 0,
    laundered: -16
  };
  const SCOPE_SCORES = { global: 12, "regional-a": 4, "regional-b": 4 };
  const STATUS_ORDER = {
    accepted: 5,
    provisional: 4,
    quarantined: 3,
    removed: 2,
    disputed: 1,
    unknown: 0
  };

  const state = {
    scenarioIndex: [],
    scenarioMap: {},
    scenarioId: null,
    scenarioPath: null,
    scenario: null,
    stepIndex: 0,
    reconnected: false,
    overrideApplied: false,
    current: null,
    deterministic: null,
    naiveComparison: null
  };

  const panels = {
    islandA: document.getElementById("island-a-panel"),
    controls: document.getElementById("control-panel"),
    islandB: document.getElementById("island-b-panel")
  };

  configureReferenceLinks();
  bootstrap();

  async function bootstrap() {
    await loadScenarios();
    bindGlobalEvents();
    const defaultScenario = state.scenarioIndex[0];
    if (!defaultScenario) {
      panels.controls.innerHTML = `
        <div class="panel-header">
          <div>
            <h2>No Scenario Data</h2>
            <p>The lab could not load any deterministic reunion scenarios.</p>
          </div>
        </div>
      `;
      return;
    }
    loadScenario(defaultScenario.id);
  }

  function bindGlobalEvents() {
    panels.controls.addEventListener("change", (event) => {
      const select = event.target.closest("[data-scenario-select]");
      if (!select) {
        return;
      }
      loadScenario(select.value);
    });

    panels.controls.addEventListener("click", (event) => {
      const button = event.target.closest("button[data-action]");
      if (!button) {
        return;
      }

      const action = button.dataset.action;
      if (action === "step") {
        stepScenario();
      } else if (action === "reconnect") {
        reconnect();
      } else if (action === "merge") {
        runDeterministic();
      } else if (action === "naive") {
        compareNaive();
      } else if (action === "override") {
        applyOverride();
      } else if (action === "reset") {
        loadScenario(state.scenarioId);
      }
    });
  }

  function configureReferenceLinks() {
    const hosted = window.location.protocol !== "file:";
    document.getElementById("merge-doc-link").href = hosted ? "../MERGE_AND_HEALING/" : "../MERGE_AND_HEALING.md";
    document.getElementById("quorum-lab-link").href = hosted ? "../quorum-lab/" : "../quorum-lab/lab.html";
    document.getElementById("scenario-source-link").href = "../scenarios/index.json";
  }

  async function loadScenarios() {
    const fallbackMap = window.DETERMINISTIC_REUNION_FALLBACK || {};
    try {
      const manifest = await fetchJson("../scenarios/index.json");
      const loaded = await Promise.all(
        manifest.scenarios.map(async (entry) => {
          const scenario = await fetchJson(`../scenarios/${entry.path}`);
          return { entry, scenario };
        })
      );

      state.scenarioIndex = loaded.map(({ entry }) => entry);
      state.scenarioMap = loaded.reduce((acc, { scenario, entry }) => {
        acc[scenario.id] = scenario;
        if (!entry.path) {
          entry.path = `${scenario.id}.json`;
        }
        return acc;
      }, {});
    } catch (_error) {
      const sortedScenarios = Object.values(fallbackMap).sort((a, b) => a.title.localeCompare(b.title));
      state.scenarioIndex = sortedScenarios.map((scenario) => ({
        id: scenario.id,
        title: scenario.title,
        path: `${scenario.id}.json`
      }));
      state.scenarioMap = fallbackMap;
    }
  }

  async function fetchJson(path) {
    const response = await fetch(path);
    if (!response.ok) {
      throw new Error(`Failed to load ${path}`);
    }
    return response.json();
  }

  function loadScenario(scenarioId) {
    const scenario = deepClone(state.scenarioMap[scenarioId]);
    const scenarioEntry = state.scenarioIndex.find((entry) => entry.id === scenarioId);
    state.scenarioId = scenarioId;
    state.scenarioPath = scenarioEntry ? scenarioEntry.path : `${scenarioId}.json`;
    state.scenario = scenario;
    state.stepIndex = 0;
    state.reconnected = false;
    state.overrideApplied = false;
    state.deterministic = null;
    state.naiveComparison = null;
    state.current = materializeCurrentState();
    render();
  }

  function stepScenario() {
    if (!state.scenario || state.reconnected || state.stepIndex >= state.scenario.events.length) {
      return;
    }
    state.stepIndex += 1;
    state.current = materializeCurrentState();
    state.deterministic = null;
    state.naiveComparison = null;
    render();
  }

  function reconnect() {
    if (!state.scenario || state.reconnected) {
      return;
    }
    state.reconnected = true;
    state.deterministic = null;
    state.naiveComparison = null;
    render();
  }

  function runDeterministic() {
    if (!state.reconnected) {
      return;
    }
    state.deterministic = runDeterministicMerge(state.scenario, state.current, state.overrideApplied);
    if (state.naiveComparison) {
      state.naiveComparison = buildNaiveComparison(state.scenario, state.current, state.deterministic);
    }
    render();
  }

  function compareNaive() {
    if (!state.reconnected) {
      return;
    }
    if (!state.deterministic) {
      state.deterministic = runDeterministicMerge(state.scenario, state.current, state.overrideApplied);
    }
    state.naiveComparison = buildNaiveComparison(state.scenario, state.current, state.deterministic);
    render();
  }

  function applyOverride() {
    if (!state.reconnected || !state.scenario.allow_operator_override || !state.deterministic || state.overrideApplied) {
      return;
    }
    state.overrideApplied = true;
    state.deterministic = runDeterministicMerge(state.scenario, state.current, true);
    if (state.naiveComparison) {
      state.naiveComparison = buildNaiveComparison(state.scenario, state.current, state.deterministic);
    }
    render();
  }

  function deepClone(value) {
    return JSON.parse(JSON.stringify(value));
  }

  function materializeCurrentState() {
    const islandA = deepClone(state.scenario.initial.island_a);
    const islandB = deepClone(state.scenario.initial.island_b);
    const logs = { island_a: [], island_b: [] };

    state.scenario.events.slice(0, state.stepIndex).forEach((event) => {
      ["island_a", "island_b"].forEach((islandKey) => {
        const patch = event.patches[islandKey];
        if (!patch) {
          return;
        }
        const island = islandKey === "island_a" ? islandA : islandB;
        if (typeof patch.local_epoch === "number") {
          island.local_epoch = patch.local_epoch;
        }
        if (patch.members) {
          Object.entries(patch.members).forEach(([memberId, memberPatch]) => {
            island.members[memberId] = deepClone(memberPatch);
          });
        }
        if (patch.log) {
          logs[islandKey].push({
            label: event.label,
            description: event.description,
            detail: patch.log
          });
        }
      });
    });

    return {
      island_a: islandA,
      island_b: islandB,
      logs
    };
  }

  function isMeaningful(entry) {
    return Boolean(entry && entry.status && entry.status !== "unknown" && entry.trust_weight > 0);
  }

  function isRestrictive(entry) {
    return Boolean(entry && ["removed", "quarantined"].includes(entry.status));
  }

  function isPermissive(entry) {
    return Boolean(entry && ["accepted", "provisional"].includes(entry.status));
  }

  function entryScore(entry) {
    if (!isMeaningful(entry)) {
      return -999;
    }
    const witness = entry.witness_summary || { count: 0, quality: "weak", diversity: "single-scope" };
    return (
      entry.trust_weight +
      (QUALITY_SCORES[witness.quality] || 0) +
      (DIVERSITY_SCORES[witness.diversity] || 0) +
      (SCOPE_SCORES[entry.scope] || 0) +
      Math.min((witness.count || 0) * 2, 10)
    );
  }

  function choosePermissive(a, b) {
    const scoreA = entryScore(a);
    const scoreB = entryScore(b);
    if (a.epoch > b.epoch && scoreA >= scoreB - 12) {
      return "a";
    }
    if (b.epoch > a.epoch && scoreB >= scoreA - 12) {
      return "b";
    }
    if (scoreA > scoreB + 4) {
      return "a";
    }
    if (scoreB > scoreA + 4) {
      return "b";
    }
    if (a.status === "accepted" && b.status !== "accepted") {
      return "a";
    }
    if (b.status === "accepted" && a.status !== "accepted") {
      return "b";
    }
    return scoreA >= scoreB ? "a" : "b";
  }

  function mergeSubject(subject, entryA, entryB) {
    const hasA = isMeaningful(entryA);
    const hasB = isMeaningful(entryB);
    const scoreA = entryScore(entryA);
    const scoreB = entryScore(entryB);
    const result = {
      subject_id: subject.id,
      subject_label: subject.label,
      source: "none",
      status: "unknown",
      stability: "stable",
      epoch: Math.max(entryA?.epoch || 0, entryB?.epoch || 0),
      trust_weight: Math.max(entryA?.trust_weight || 0, entryB?.trust_weight || 0),
      explanation: "No admissible view survived the reunion.",
      rule_summary: [],
      residue: null
    };

    if (!hasA && !hasB) {
      result.rule_summary.push("No admissible input survived for this subject.");
      return result;
    }

    if (hasA && !hasB) {
      result.source = "island_a";
      result.status = entryA.status;
      result.stability = entryScore(entryA) >= 95 && entryA.status !== "provisional" ? "stable" : "provisional";
      result.epoch = entryA.epoch;
      result.trust_weight = entryA.trust_weight;
      result.explanation = "Only Island A carried an admissible path into reunion.";
      result.rule_summary.push("Single-sided admissible state carried forward.");
      if (result.stability === "provisional") {
        result.residue = {
          subject_label: subject.label,
          detail: "Only one island had enough evidence to speak. Reunion preserves that asymmetry instead of pretending shared certainty."
        };
      }
      return result;
    }

    if (!hasA && hasB) {
      result.source = "island_b";
      result.status = entryB.status;
      result.stability = entryScore(entryB) >= 95 && entryB.status !== "provisional" ? "stable" : "provisional";
      result.epoch = entryB.epoch;
      result.trust_weight = entryB.trust_weight;
      result.explanation = "Only Island B carried an admissible path into reunion.";
      result.rule_summary.push("Single-sided admissible state carried forward.");
      if (result.stability === "provisional") {
        result.residue = {
          subject_label: subject.label,
          detail: "Only one island had enough evidence to speak. Reunion preserves that asymmetry instead of pretending shared certainty."
        };
      }
      return result;
    }

    if (entryA.status === entryB.status) {
      const preferred = scoreA >= scoreB ? entryA : entryB;
      result.source = "both";
      result.status = entryA.status;
      result.stability = entryA.status === "provisional" ? "provisional" : "stable";
      result.epoch = Math.max(entryA.epoch, entryB.epoch);
      result.trust_weight = Math.max(entryA.trust_weight, entryB.trust_weight);
      result.explanation = `Both islands converge on ${entryA.status}.`;
      result.rule_summary.push("Matching local outcomes converge cleanly.");
      result.rule_summary.push(preferred === entryA ? "Island A supplied the stronger retained detail." : "Island B supplied the stronger retained detail.");
      return result;
    }

    const launderingA = entryA?.witness_summary?.diversity === "laundered";
    const launderingB = entryB?.witness_summary?.diversity === "laundered";

    if (launderingA || launderingB) {
      const launderedEntry = launderingA ? entryA : entryB;
      const launderedSource = launderingA ? "Island A" : "Island B";
      const otherEntry = launderingA ? entryB : entryA;
      const otherSource = launderingA ? "Island B" : "Island A";
      if (entryScore(otherEntry) >= entryScore(launderedEntry) - 8) {
        result.source = launderingA ? "island_b" : "island_a";
        result.status = otherEntry.status === "accepted" ? "provisional" : otherEntry.status;
        result.stability = "provisional";
        result.epoch = Math.max(otherEntry.epoch, launderedEntry.epoch);
        result.trust_weight = otherEntry.trust_weight;
        result.explanation = "Low-quality reinforcing witnesses were discounted during reunion.";
        result.rule_summary.push("Trust and corroboration quality dominated raw witness count.");
        result.rule_summary.push(`${otherSource} preserved the more admissible path.`);
        result.residue = {
          subject_label: subject.label,
          detail: `${launderedSource} accumulated a louder but lower-quality witness cluster. The merge retains that laundering attempt as visible residue.`
        };
        return result;
      }
    }

    if (isRestrictive(entryA) || isRestrictive(entryB)) {
      const restrictive = isRestrictive(entryA) ? entryA : entryB;
      const permissive = isRestrictive(entryA) ? entryB : entryA;
      const restrictiveSource = isRestrictive(entryA) ? "island_a" : "island_b";
      const permissiveSource = restrictiveSource === "island_a" ? "island_b" : "island_a";

      if (
        restrictive.epoch === permissive.epoch &&
        restrictive.witness_summary.quality !== "weak" &&
        permissive.witness_summary.quality !== "weak" &&
        Math.abs(scoreA - scoreB) < 14
      ) {
        result.source = "both";
        result.status = "disputed";
        result.stability = "provisional";
        result.explanation = "Fresh admissible evidence remains materially unresolved after the allowed dominance checks.";
        result.rule_summary.push("Same-epoch high-trust conflict survives as residue.");
        result.residue = {
          subject_label: subject.label,
          detail: `Island A says ${entryA.status}. Island B says ${entryB.status}. Both paths remain fresh enough that deterministic reunion refuses to counterfeit certainty.`
        };
        return result;
      }

      if (restrictive.epoch > permissive.epoch && entryScore(restrictive) >= entryScore(permissive) - 6) {
        result.source = restrictiveSource;
        result.status = restrictive.status;
        result.stability = "stable";
        result.epoch = restrictive.epoch;
        result.trust_weight = restrictive.trust_weight;
        result.explanation = "Newer restrictive evidence dominates the older permissive path.";
        result.rule_summary.push("Fresh restrictive evidence dominated older permissive evidence.");
        result.residue = {
          subject_label: subject.label,
          detail: `${permissiveSource === "island_a" ? "Island A" : "Island B"} still carried a coherent permissive path. The race remains visible even though the newer restrictive path wins.`
        };
        return result;
      }

      if (entryScore(restrictive) >= entryScore(permissive) + 14) {
        result.source = restrictiveSource;
        result.status = restrictive.status;
        result.stability = "provisional";
        result.epoch = restrictive.epoch;
        result.trust_weight = restrictive.trust_weight;
        result.explanation = "Restrictive evidence dominates on trust and admissibility, but the opposing path remains visible.";
        result.rule_summary.push("Trust and scope authority favored the restrictive path.");
        result.residue = {
          subject_label: subject.label,
          detail: "The merge chooses the restrictive path, but it keeps the opposing history as residue rather than pretending the split never happened."
        };
        return result;
      }

      result.source = "both";
      result.status = "disputed";
      result.stability = "provisional";
      result.explanation = "Restriction versus acceptance remained too close to resolve honestly.";
      result.rule_summary.push("Conflict survived the deterministic reunion pass.");
      result.residue = {
        subject_label: subject.label,
        detail: `Restriction and acceptance remained materially close for ${subject.label}. The conflict stays visible.`
      };
      return result;
    }

    if (isPermissive(entryA) && isPermissive(entryB)) {
      const chosenKey = choosePermissive(entryA, entryB);
      const chosen = chosenKey === "a" ? entryA : entryB;
      const other = chosenKey === "a" ? entryB : entryA;
      result.source = chosenKey === "a" ? "island_a" : "island_b";
      result.status = chosen.status;
      result.epoch = Math.max(entryA.epoch, entryB.epoch);
      result.trust_weight = chosen.trust_weight;

      if (chosen.status === "provisional" && other.status === "accepted") {
        result.stability = "provisional";
        result.explanation = "Fresher admissible evidence downgrades the older acceptance path.";
        result.rule_summary.push("Freshness dominated older permissive acceptance.");
        result.residue = {
          subject_label: subject.label,
          detail: `${other.status} on the older path remains visible as residue, because recontact did not restore innocence.`
        };
        return result;
      }

      result.stability = chosen.status === "accepted" && entryScore(chosen) >= entryScore(other) + 8 ? "stable" : "provisional";
      result.explanation = "Both islands lean permissive, and the stronger path closes the gap.";
      result.rule_summary.push("Permissive paths converged under freshness and trust weighting.");
      if (result.stability === "provisional") {
        result.residue = {
          subject_label: subject.label,
          detail: "The two islands leaned the same direction, but one path remained weaker enough that the merged result stays provisional."
        };
      }
      return result;
    }

    result.source = scoreA >= scoreB ? "island_a" : "island_b";
    result.status = scoreA >= scoreB ? entryA.status : entryB.status;
    result.stability = "provisional";
    result.epoch = Math.max(entryA.epoch, entryB.epoch);
    result.trust_weight = Math.max(entryA.trust_weight, entryB.trust_weight);
    result.explanation = "The reunion retained the stronger path, but not without visible caution.";
    result.rule_summary.push("Fallback deterministic comparison retained the stronger admissible path.");
    result.residue = {
      subject_label: subject.label,
      detail: "This subject still carried enough disagreement to keep a visible scar in the merged view."
    };
    return result;
  }

  function runDeterministicMerge(scenario, currentState, applyOperatorOverride) {
    const merged = scenario.subjects.map((subject) =>
      mergeSubject(subject, currentState.island_a.members[subject.id], currentState.island_b.members[subject.id])
    );

    if (applyOperatorOverride && scenario.allow_operator_override && scenario.operator_override) {
      const overrideTarget = merged.find((member) => member.subject_id === scenario.operator_override.subject_id);
      if (overrideTarget) {
        overrideTarget.status = scenario.operator_override.status;
        overrideTarget.source = "override";
        overrideTarget.stability = "stable-with-override";
        overrideTarget.explanation = scenario.operator_override.reason;
        overrideTarget.rule_summary.push("OperatorOverride applied visibly.");
        if (overrideTarget.residue) {
          overrideTarget.residue.handled_by_override = true;
          overrideTarget.residue.detail += " OperatorOverride forced a visible intervention instead of silent collapse.";
        } else {
          overrideTarget.residue = {
            subject_label: overrideTarget.subject_label,
            detail: "OperatorOverride forced a visible intervention.",
            handled_by_override: true
          };
        }
      }
    }

    const residues = merged
      .filter((member) => member.residue)
      .map((member) => ({
        subject_label: member.subject_label,
        detail: member.residue.detail,
        handled_by_override: Boolean(member.residue.handled_by_override)
      }));

    const unresolvedResidues = residues.filter((residue) => !residue.handled_by_override);
    const distinctRules = Array.from(new Set(merged.flatMap((member) => member.rule_summary)));
    const anyProvisional = merged.some((member) => member.stability === "provisional" || member.status === "disputed");
    const overallOutcome = applyOperatorOverride
      ? unresolvedResidues.length === 0
        ? "stable-with-override"
        : "provisional-with-override"
      : unresolvedResidues.length === 0 && !anyProvisional
        ? "stable"
        : "provisional";

    return {
      members: merged,
      residues,
      digest: {
        overall_outcome: overallOutcome,
        compared_subjects: scenario.subjects.length,
        inputs: [
          `Island A epoch ${currentState.island_a.local_epoch}`,
          `Island B epoch ${currentState.island_b.local_epoch}`,
          `${state.stepIndex} divergence event(s) replayed`
        ],
        rules_fired: distinctRules,
        unresolved: unresolvedResidues,
        override: applyOperatorOverride ? scenario.operator_override : null
      }
    };
  }

  function runNaiveReunion(scenario, currentState) {
    return scenario.subjects.map((subject) => {
      const entryA = currentState.island_a.members[subject.id];
      const entryB = currentState.island_b.members[subject.id];
      const candidates = [];
      if (isMeaningful(entryA)) {
        candidates.push({ source: "Island A", entry: entryA });
      }
      if (isMeaningful(entryB)) {
        candidates.push({ source: "Island B", entry: entryB });
      }
      if (candidates.length === 0) {
        return {
          subject_label: subject.label,
          status: "unknown",
          source: "none",
          note: "No admissible input."
        };
      }
      candidates.sort((left, right) => {
        if (right.entry.epoch !== left.entry.epoch) {
          return right.entry.epoch - left.entry.epoch;
        }
        if (right.entry.witness_summary.count !== left.entry.witness_summary.count) {
          return right.entry.witness_summary.count - left.entry.witness_summary.count;
        }
        if (STATUS_ORDER[right.entry.status] !== STATUS_ORDER[left.entry.status]) {
          return STATUS_ORDER[right.entry.status] - STATUS_ORDER[left.entry.status];
        }
        return right.entry.trust_weight - left.entry.trust_weight;
      });
      const chosen = candidates[0];
      return {
        subject_label: subject.label,
        status: chosen.entry.status,
        source: chosen.source,
        note: "Latest or loudest path wins. Residue is discarded."
      };
    });
  }

  function buildNaiveComparison(scenario, currentState, deterministic) {
    const naive = runNaiveReunion(scenario, currentState);
    const diffs = scenario.subjects
      .map((subject) => {
        const deterministicMember = deterministic.members.find((member) => member.subject_id === subject.id);
        const naiveMember = naive.find((member) => member.subject_label === subject.label);
        const hidesResidue = Boolean(deterministicMember.residue && !deterministicMember.residue.handled_by_override);
        if (!naiveMember || !deterministicMember) {
          return null;
        }
        if (naiveMember.status !== deterministicMember.status || hidesResidue) {
          return {
            subject_label: subject.label,
            naive_status: naiveMember.status,
            deterministic_status: deterministicMember.status,
            note: hidesResidue
              ? "Naive reunion would erase visible residue."
              : `Naive reunion would choose ${naiveMember.status} from ${naiveMember.source}.`
          };
        }
        return null;
      })
      .filter(Boolean);

    return { naive, diffs };
  }

  function computeIslandDigest(island) {
    return Object.values(island.members).reduce(
      (acc, member) => {
        acc[member.status] = (acc[member.status] || 0) + 1;
        return acc;
      },
      { accepted: 0, provisional: 0, quarantined: 0, removed: 0, unknown: 0 }
    );
  }

  function render() {
    renderIslandPanel("island_a", panels.islandA, "Island A State");
    renderControls();
    renderRightPanel();
  }

  function renderIslandPanel(key, panel, title) {
    const island = state.current[key];
    const digest = computeIslandDigest(island);
    const logItems = state.current.logs[key]
      .map(
        (item) => `
          <article class="event-card">
            <div class="event-top">
              <strong>${item.label}</strong>
              <span class="status-pill status-neutral">${key === "island_a" ? "A" : "B"}</span>
            </div>
            <p>${item.detail}</p>
          </article>
        `
      )
      .join("");

    panel.innerHTML = `
      <div class="panel-header">
        <div>
          <h2>${title}</h2>
          <p>Local membership view, local epoch, and island-specific acceptance logic accumulated during the split.</p>
        </div>
      </div>
      <div class="summary-grid">
        <div class="summary-card">
          <span>Local Epoch</span>
          <strong>${island.local_epoch}</strong>
        </div>
        <div class="summary-card">
          <span>Accepted</span>
          <strong>${digest.accepted || 0}</strong>
        </div>
        <div class="summary-card">
          <span>Provisional</span>
          <strong>${digest.provisional || 0}</strong>
        </div>
        <div class="summary-card">
          <span>Restrictive</span>
          <strong>${(digest.quarantined || 0) + (digest.removed || 0)}</strong>
        </div>
      </div>
      <div class="section-card">
        <strong>Local Digest</strong>
        <div class="chip-row">
          <span class="digest-chip"><strong>scope</strong> ${island.scope}</span>
          <span class="digest-chip"><strong>unknown</strong> ${digest.unknown || 0}</span>
          <span class="digest-chip"><strong>active divergence</strong> ${state.stepIndex}/${state.scenario.events.length}</span>
        </div>
      </div>
      <div class="state-grid">
        ${state.scenario.subjects.map((subject) => renderMemberCard(subject, island.members[subject.id], "local")).join("")}
      </div>
      <div class="section-card" style="margin-top: 16px;">
        <strong>Island Event Log</strong>
        ${logItems || '<div class="empty-state">No divergence events have landed here yet.</div>'}
      </div>
    `;
  }

  function renderControls() {
    const currentEvent = state.scenario.events[state.stepIndex] || null;
    const deterministicReady = state.reconnected && state.deterministic;
    const canOverride =
      deterministicReady &&
      state.scenario.allow_operator_override &&
      !state.overrideApplied &&
      state.deterministic.members.some((member) => member.subject_id === state.scenario.operator_override.subject_id && member.status === "disputed");

    panels.controls.innerHTML = `
      <div class="panel-header">
        <div>
          <h2>Reunion / Merge Control</h2>
          <p>Step divergence while partitioned, reconnect explicitly, then inspect what deterministic reunion will resolve and what it refuses to flatten away.</p>
        </div>
      </div>

      <div class="form-block">
        <label for="scenario-select">Load Scenario</label>
        <select id="scenario-select" class="select" data-scenario-select>
          ${state.scenarioIndex
            .map(
              (entry) => `
                <option value="${entry.id}" ${entry.id === state.scenarioId ? "selected" : ""}>${entry.title}</option>
              `
            )
            .join("")}
        </select>
      </div>

      <div class="scenario-card">
        <strong>${state.scenario.title}</strong>
        <p>${state.scenario.summary}</p>
        <div class="chip-row" style="margin-top: 12px;">
          <span class="chip"><strong>scenario file</strong> <code>${state.scenarioPath}</code></span>
          <span class="chip"><strong>override</strong> ${state.scenario.allow_operator_override ? "available" : "not allowed"}</span>
        </div>
        <p style="margin-top: 12px;">${state.scenario.difference_from_quorum_lab}</p>
      </div>

      <div class="partition-card">
        <strong>Partition State</strong>
        <div class="partition-viz">
          <div class="island-node">
            <p class="eyebrow">Island A</p>
            <strong>${state.current.island_a.scope}</strong>
          </div>
          <div class="bridge ${state.reconnected ? "rejoined" : "split"}"></div>
          <div class="island-node">
            <p class="eyebrow">Island B</p>
            <strong>${state.current.island_b.scope}</strong>
          </div>
        </div>
        <div class="chip-row" style="margin-top: 12px;">
          <span class="chip"><strong>step</strong> ${state.stepIndex}/${state.scenario.events.length}</span>
          <span class="chip"><strong>contact</strong> ${state.reconnected ? "restored" : "partitioned"}</span>
          <span class="chip"><strong>next event</strong> ${currentEvent ? currentEvent.label : "none"}</span>
        </div>
      </div>

      <div class="toolbar">
        <button class="button" data-action="step" ${state.reconnected || state.stepIndex >= state.scenario.events.length ? "disabled" : ""}>Step Drift</button>
        <button class="button warn" data-action="reconnect" ${state.reconnected ? "disabled" : ""}>Reconnect Islands</button>
        <button class="button primary" data-action="merge" ${!state.reconnected ? "disabled" : ""}>Run Deterministic Reunion</button>
        <button class="button" data-action="naive" ${!state.reconnected ? "disabled" : ""}>Compare Naive Reunion</button>
        <button class="button danger" data-action="override" ${!canOverride ? "disabled" : ""}>Apply OperatorOverride</button>
        <button class="button" data-action="reset">Reset / Replay</button>
      </div>

      <div class="section-card">
        <strong>Divergence Timeline</strong>
        <div class="event-list">
          ${state.scenario.events
            .map((event, index) => {
              const applied = index < state.stepIndex;
              return `
                <article class="event-card">
                  <div class="event-top">
                    <strong>${event.label}</strong>
                    <span class="status-pill ${applied ? "status-good" : "status-neutral"}">${applied ? "applied" : "pending"}</span>
                  </div>
                  <p>${event.description}</p>
                </article>
              `;
            })
            .join("")}
        </div>
      </div>

      <div class="difference-card">
        <strong>Expected Merge Tensions</strong>
        <div class="rule-list">
          ${state.scenario.expected_merge_tensions.map((tension) => `<div class="card"><p>${tension}</p></div>`).join("")}
        </div>
      </div>
    `;
  }

  function renderRightPanel() {
    const island = state.current.island_b;
    const digest = computeIslandDigest(island);
    const logItems = state.current.logs.island_b
      .map(
        (item) => `
          <article class="event-card">
            <div class="event-top">
              <strong>${item.label}</strong>
              <span class="status-pill status-neutral">B</span>
            </div>
            <p>${item.detail}</p>
          </article>
        `
      )
      .join("");

    panels.islandB.innerHTML = `
      <div class="panel-header">
        <div>
          <h2>Island B / Merged Outcome</h2>
          <p>Island B keeps its own local coherence. The merged view and RepairDigest appear below only after explicit reunion.</p>
        </div>
      </div>
      <div class="summary-grid">
        <div class="summary-card">
          <span>Local Epoch</span>
          <strong>${island.local_epoch}</strong>
        </div>
        <div class="summary-card">
          <span>Accepted</span>
          <strong>${digest.accepted || 0}</strong>
        </div>
        <div class="summary-card">
          <span>Provisional</span>
          <strong>${digest.provisional || 0}</strong>
        </div>
        <div class="summary-card">
          <span>Restrictive</span>
          <strong>${(digest.quarantined || 0) + (digest.removed || 0)}</strong>
        </div>
      </div>
      <div class="state-grid">
        ${state.scenario.subjects.map((subject) => renderMemberCard(subject, island.members[subject.id], "local")).join("")}
      </div>
      <div class="section-card" style="margin-top: 16px;">
        <strong>Island Event Log</strong>
        ${logItems || '<div class="empty-state">No divergence events have landed here yet.</div>'}
      </div>
      ${renderMergedSections()}
    `;
  }

  function renderMergedSections() {
    if (!state.reconnected) {
      return `
        <div class="section-card" style="margin-top: 16px;">
          <strong>Merged View Pending</strong>
          <div class="empty-state">The islands are still partitioned. Reunion has not started, so no merged membership view should exist yet.</div>
        </div>
      `;
    }

    if (!state.deterministic) {
      return `
        <div class="section-card" style="margin-top: 16px;">
          <strong>RepairDigest Preview</strong>
          <div class="digest-list">
            <div class="digest-card">
              <div class="digest-top">
                <strong>Merge Inputs Ready</strong>
                <span class="status-pill status-neutral">recontacted</span>
              </div>
              <p>The islands have recontacted, but the system has not yet run deterministic reunion. Connectivity is restored; truth is not yet restored.</p>
              <div class="chip-row" style="margin-top: 12px;">
                <span class="digest-chip"><strong>Island A</strong> epoch ${state.current.island_a.local_epoch}</span>
                <span class="digest-chip"><strong>Island B</strong> epoch ${state.current.island_b.local_epoch}</span>
                <span class="digest-chip"><strong>subjects</strong> ${state.scenario.subjects.length}</span>
              </div>
            </div>
          </div>
        </div>
      `;
    }

    const digest = state.deterministic.digest;
    const mergedCards = state.deterministic.members.map((member) => renderMergedMemberCard(member)).join("");
    const residueCards = state.deterministic.residues.length
      ? state.deterministic.residues
          .map(
            (residue) => `
              <article class="residue-card">
                <strong>${residue.subject_label}</strong>
                <p>${residue.detail}</p>
                <div class="chip-row" style="margin-top: 10px;">
                  <span class="chip ${residue.handled_by_override ? "status-good" : "status-warn"}">${residue.handled_by_override ? "handled by override" : "still unresolved"}</span>
                </div>
              </article>
            `
          )
          .join("")
      : '<div class="empty-state">No visible residue remained after deterministic reunion.</div>';

    const diffSection = state.naiveComparison
      ? `
          <div class="section-card" style="margin-top: 16px;">
            <strong>Naive Reunion Comparison</strong>
            ${
              state.naiveComparison.diffs.length
                ? `<div class="diff-list">
                    ${state.naiveComparison.diffs
                      .map(
                        (diff) => `
                          <article class="comparison-card">
                            <strong>${diff.subject_label}</strong>
                            <p>${diff.note}</p>
                            <div class="chip-row" style="margin-top: 10px;">
                              <span class="chip"><strong>naive</strong> ${diff.naive_status}</span>
                              <span class="chip"><strong>deterministic</strong> ${diff.deterministic_status}</span>
                            </div>
                          </article>
                        `
                      )
                      .join("")}
                  </div>`
                : '<div class="empty-state">Naive reunion lands on the same statuses here, but still fails to explain why those statuses are deserved.</div>'
            }
          </div>
        `
      : "";

    return `
      <div class="section-card" style="margin-top: 16px;">
        <strong>Merged Membership View</strong>
        <div class="state-grid">${mergedCards}</div>
      </div>
      <div class="section-card" style="margin-top: 16px;">
        <strong>Visible Residue</strong>
        <div class="residue-list">${residueCards}</div>
      </div>
      <div class="section-card" style="margin-top: 16px;">
        <strong>RepairDigest</strong>
        <div class="digest-list">
          <article class="digest-card">
            <div class="digest-top">
              <strong>Outcome</strong>
              <span class="status-pill ${digest.overall_outcome.includes("stable") ? "status-good" : "status-warn"}">${formatOutcome(digest.overall_outcome)}</span>
            </div>
            <p>What inputs were merged, which rules fired, what remained conflicting, and whether any override occurred.</p>
            <div class="chip-row" style="margin-top: 12px;">
              ${digest.inputs.map((input) => `<span class="digest-chip">${input}</span>`).join("")}
            </div>
          </article>
          <article class="digest-card">
            <strong>Rules Fired</strong>
            <div class="rule-list">
              ${digest.rules_fired.map((rule) => `<div class="card"><p>${rule}</p></div>`).join("")}
            </div>
          </article>
          <article class="digest-card">
            <strong>Unresolved Conflict</strong>
            ${
              digest.unresolved.length
                ? digest.unresolved.map((item) => `<div class="card"><p>${item.subject_label}: ${item.detail}</p></div>`).join("")
                : '<div class="empty-state">No unresolved residue remains.</div>'
            }
          </article>
          <article class="digest-card">
            <strong>OperatorOverride</strong>
            ${
              digest.override
                ? `<p>${digest.override.reason}</p><div class="chip-row" style="margin-top: 12px;"><span class="digest-chip"><strong>subject</strong> ${digest.override.subject_id}</span><span class="digest-chip"><strong>forced status</strong> ${digest.override.status}</span></div>`
                : '<div class="empty-state">No operator override was applied.</div>'
            }
          </article>
        </div>
      </div>
      ${diffSection}
    `;
  }

  function renderMemberCard(subject, member, kind) {
    const statusClass = statusClassName(member.status);
    return `
      <article class="member-card" data-status="${member.status}">
        <div class="member-top">
          <div>
            <strong>${subject.label}</strong>
            <p>${subject.role} / ${subject.domain}</p>
          </div>
          <span class="status-pill ${statusClass}">${member.status}</span>
        </div>
        <div class="member-meta">
          <span>epoch ${member.epoch}</span>
          <span>trust ${member.trust_weight}</span>
          <span>${member.scope}</span>
        </div>
        <div class="chip-row" style="margin-bottom: 10px;">
          <span class="chip"><strong>witnesses</strong> ${member.witness_summary.count}</span>
          <span class="chip"><strong>quality</strong> ${member.witness_summary.quality}</span>
          <span class="chip"><strong>diversity</strong> ${member.witness_summary.diversity}</span>
        </div>
        <p>${member.explanation}</p>
      </article>
    `;
  }

  function renderMergedMemberCard(member) {
    const statusClass = statusClassName(member.status);
    return `
      <article class="member-card" data-status="${member.status}">
        <div class="member-top">
          <div>
            <strong>${member.subject_label}</strong>
            <p>${member.explanation}</p>
          </div>
          <span class="status-pill ${statusClass}">${member.status}</span>
        </div>
        <div class="member-meta">
          <span>epoch ${member.epoch}</span>
          <span>trust ${member.trust_weight}</span>
          <span>${member.source.replace("_", " ")}</span>
          <span>${member.stability}</span>
        </div>
        <div class="rule-list">
          ${member.rule_summary.map((rule) => `<div class="card"><p>${rule}</p></div>`).join("")}
        </div>
      </article>
    `;
  }

  function statusClassName(status) {
    if (status === "accepted") {
      return "status-good";
    }
    if (status === "provisional" || status === "unknown") {
      return "status-warn";
    }
    if (status === "disputed" || status === "removed" || status === "quarantined") {
      return "status-bad";
    }
    return "status-neutral";
  }

  function formatOutcome(outcome) {
    return outcome.replace(/-/g, " ");
  }
})();
