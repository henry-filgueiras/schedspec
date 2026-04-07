# Quorum Lab

Quorum Lab is a small interactive browser artifact for the **quorum-conditioned observability** idea.

It exists to make one claim legible quickly:

> A threshold ceremony is not mainly a way to hide a key; it is a way to require sufficiently real collective presence before hidden state may enter the observable world.

This artifact is intentionally narrow:

- static, local-only, and easy to host
- conceptual rather than cryptographic
- focused on policy-shaped quorum, topology, epoch freshness, and transcript residue

Files:

- [`lab.html`](lab.html): single-page interactive demo with inline CSS and JavaScript

Use:

1. Open [`lab.html`](lab.html) in a browser.
2. Load a scenario or change node state manually.
3. Attempt reveal and inspect the transcript.

For the published docs site, launch the same artifact from [`lab.html`](lab.html).

For local file browsing, open [`lab.html`](lab.html) directly.

What it demonstrates:

- plain `M-of-N` is often too crude
- stale or revoked participants should not count
- fake diversity should fail policy
- role and domain constraints can make membership causal
- partition healing leaves residue rather than erasing history

Related note:

- [`../quorum-conditioned-observability/README.md`](../quorum-conditioned-observability/README.md)
- [`../quorum-conditioned-observability/note.md`](../quorum-conditioned-observability/note.md)

Non-claim:

This is not a production protocol, not a cryptographic implementation, and not evidence that the repo contains a finished runtime.
