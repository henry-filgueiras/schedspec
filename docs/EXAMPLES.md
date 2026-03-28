# ChronOS Examples

These examples are intentionally text-first and semantics-first. They illustrate the kind of workflows ChronOS and `chrono flow` are meant to handle, rather than claiming a finished runtime or frozen syntax.

For the language sketch, see [`LANGUAGE.md`](LANGUAGE.md). For the semantic contract, see [`SPEC.md`](SPEC.md). For the core vocabulary, see [`GLOSSARY.md`](GLOSSARY.md).

## 1. Deployment Rollout

Problem:

- deploy a service across regions
- require an operator gate
- proceed after quorum success
- compensate traffic shift if the rollout later degrades

```chrono
flow global_rollout(service: ServiceId, target: Version) {
  state {
    approved = false
    live_regions: set<Region> = {}
  }

  on start {
    emit effect create_release_record(service, target)
    await operator.approve("release-manager") within 2h
  }

  on operator.approved(_) {
    approved = true
  }

  when approved {
    child deploy_region(service, target, "us-west") as west
    child deploy_region(service, target, "us-east") as east
    child deploy_region(service, target, "eu-central") as eu
    await quorum children completed(status == ok) >= 2 within 30m
    emit effect shift_global_traffic(service, target)
  }

  on child_completed deploy_region(region, status == ok) {
    live_regions += region
  }

  on child_failed deploy_region(region) {
    emit effect page_release_team(service, region)
  }

  on effect_failed shift_global_traffic(error) {
    compensate create_release_record(service, target)
    complete failed
  }
}
```

Why this is a ChronOS problem:

- the rollout must survive process restarts and long waits
- quorum waits and child lineage need durable structure
- operator approval is an auditable event
- traffic shift is an explicit effect
- comparative replay can answer "what would the new quorum rule have done?"

## 2. Approval / Human-in-the-Loop Workflow

Problem:

- route a request to a manager
- allow approval, denial, escalation, or manual operator override
- preserve a durable audit trail

```chrono
flow expense_approval(report: ReportId, owner: UserId, amount: Money) {
  state {
    approver: option<UserId> = none
    decision: option<Decision> = none
  }

  on start {
    emit effect notify_manager(owner, report, amount)
    await operator.action in ["approve", "deny"] within 48h
  }

  on operator.approved(by) {
    approver = some(by)
    decision = some(approve)
    emit effect reimburse(owner, amount)
    complete approved
  }

  on operator.denied(by) {
    approver = some(by)
    decision = some(deny)
    emit effect notify_denial(owner, report)
    complete denied
  }

  on timeout {
    emit effect escalate_to_finance(report)
  }
}
```

Why this is a ChronOS problem:

- the human action is not an out-of-band note; it is part of the history
- deadlines and escalation are durable
- operator override can be modeled without hiding causality

## 3. Payment Authorization with Compensation

Problem:

- authorize payment
- capture funds
- if downstream fulfillment fails, compensate with void or refund semantics

```chrono
flow charge_order(order: OrderId, amount: Money) {
  state {
    authorization_id: option<AuthId> = none
    capture_id: option<CaptureId> = none
  }

  on start {
    emit effect authorize_payment(order, amount)
  }

  on effect_succeeded authorize_payment(auth_id) {
    authorization_id = some(auth_id)
    emit effect capture_payment(auth_id, amount)
  }

  on effect_succeeded capture_payment(capture) {
    capture_id = some(capture)
    child fulfill_order(order) as fulfillment
    await child fulfillment completed within 2h
  }

  on child_failed fulfill_order {
    compensate refund_payment(capture_id)
    complete failed
  }

  on effect_failed capture_payment(error) {
    retry effect capture_payment in exponential(30s, factor: 2, max: 15m) up to 5 times
  }

  on retry_exhausted capture_payment {
    compensate void_authorization(authorization_id)
    complete failed
  }
}
```

Why this is a ChronOS problem:

- external effects have partial progress and nontrivial compensation
- retries must be explicit and auditable
- fulfillment and payment form a parent-child causal tree

## 4. Incident Response / Escalation

Problem:

- open an incident
- gather context
- escalate by severity and elapsed time
- allow operator intervention without losing the record

```chrono
flow incident_response(incident: IncidentId, severity: Severity) {
  state {
    commander: option<UserId> = none
    sev = severity
  }

  on start {
    child gather_logs(incident) as logs
    child identify_recent_deploys(incident) as deploys
    emit effect page_primary_oncall(incident, sev)
    await operator.claim("incident-commander") within 10m
  }

  on operator.claimed(by) {
    commander = some(by)
    await all children completed
  }

  on timeout {
    emit effect escalate_page(incident, sev)
  }

  when sev == critical {
    emit effect create_war_room(incident)
  }
}
```

Why this is a ChronOS problem:

- time-based escalation is native
- child flows gather context in parallel
- operator claim is a first-class event
- replay can explain why escalation occurred when it did

## 5. AI Tool Orchestration

Problem:

- treat agents as durable workflows with explicit audit trails
- gather evidence, run tools, request review, and publish an operator-approved output

```chrono
flow investigate_customer_issue(ticket: TicketId) {
  state {
    summary_ready = false
  }

  on start {
    child collect_ticket_context(ticket) as context
    child search_logs(ticket) as logs
    child query_kb(ticket) as kb
    await all children completed
  }

  on all_children_completed {
    emit effect run_model_summarizer(ticket)
    await effect run_model_summarizer settled within 5m
  }

  on effect_succeeded run_model_summarizer(summary) {
    summary_ready = true
    await operator.approve("support-lead")
  }

  when summary_ready {
    emit effect publish_customer_response(ticket)
  }
}
```

Why this is a ChronOS problem:

- "agents" become inspectable workflows instead of opaque loops
- tool calls are explicit effects
- retries and approvals are auditable
- history can distinguish model output, operator review, and publication

## Cross-Cutting Observations

Across all five examples, the same structural needs recur:

- durable identity
- append-only history
- explicit effects
- replay-safe decisions
- timers and deadlines
- operator actions as events
- child flow trees
- compensation and cancellation

That recurrence is the project's central argument: these are not isolated product features but parts of one temporal execution model.
