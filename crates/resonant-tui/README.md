# resonant-tui

The whole split-brain story in one window. Spawns three in-process chat
nodes (alice the creator, bob, carol) over real TCP loopback and renders
them side by side — log pane, standing roster, and residue panel per node,
with a convergence strip showing each node's digest hash turn green when
every replica holds the identical view.

```
cargo run -p resonant-tui
```

| Key | Story beat |
|---|---|
| `F2` | Partition: carol splits off (block lists sever connections) |
| `F3` | alice's island bans carol; carol's island still believes in her |
| `F4` | Heal: redial, digest exchange, deterministic reunion — watch the strip go from DIVERGED to CONVERGED with carol **disputed** and the scar in every residue panel |
| `F5` | Creator override: carol quarantined, residue marked `[handled]`, still visible |
| `Tab` | Move input focus between nodes; type to chat or run any `/command` as that node |
| `Esc` | Quit |

Wait a few seconds after launch for standing to accrue (watch the rosters
walk `introduced → witnessed → provisional → accepted` as witness records
gossip and hysteresis passes), then run the beats in order.
