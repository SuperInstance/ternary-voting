# ternary-voting

Voting and consensus mechanisms where every vote is ternary: for (+1), against (−1), or abstain (0).

## Why This Exists

Binary voting forces a choice: yes or no. Real decision-making includes "I don't have enough information" or "I disagree with both options." Ternary voting captures abstention as a first-class vote. This crate implements five voting systems — simple tally, quorum-based, weighted, Byzantine agreement, and liquid democracy — plus Condorcet pairwise elections — all operating on ternary values. If you're building governance, consensus, or decision-making systems where neutrality matters, this is the foundation.

## Core Concepts

- **TernaryVote** — For (+1), Against (−1), Abstain (0). Abstain is a deliberate vote, not a missing vote.
- **VoteTally** — Counts votes and computes net support (`for_count − against_count`). Ties produce Abstain.
- **Quorum** — A requirement that both a minimum number of votes and a minimum fraction of eligible voters participate before a result is valid.
- **Weighted voting** — Each voter carries a weight. The result is whichever side has the highest total weight.
- **Byzantine agreement** — Tolerates up to f faulty nodes when n > 3f. Requires >2/3 supermajority to reach consensus.
- **Liquid democracy** — Voters can either vote directly or delegate their vote to another voter. Delegation chains resolve transitively; the final voter casts the accumulated weight.
- **Condorcet method** — Pairwise comparison of all candidates. The Condorcet winner beats every other candidate head-to-head. May not exist (Condorcet paradox).

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-voting = "0.1"
```

```rust
use ternary_voting::*;

fn main() {
    // Simple tally
    let mut tally = VoteTally::new();
    tally.record(TernaryVote::For);
    tally.record(TernaryVote::For);
    tally.record(TernaryVote::Against);
    tally.record(TernaryVote::Abstain);
    println!("Result: {:?}, Net support: {}", tally.result(), tally.net_support());

    // Liquid democracy
    let mut ld = LiquidDemocracy::new(4);
    ld.vote_direct(0, TernaryVote::For);
    ld.delegate(1, 0); // voter 1 delegates to voter 0
    ld.delegate(2, 0);
    ld.vote_direct(3, TernaryVote::Against);
    let result = ld.final_tally();
    println!("Liquid result: {:?} (for={}, against={}, abstain={})",
        result.result(), result.for_count, result.against_count, result.abstain_count);

    // Byzantine agreement (4 nodes, tolerates 1 faulty)
    let ba = ByzantineAgreement::new(4);
    let consensus = ba.consensus_round(&[
        TernaryVote::For, TernaryVote::For, TernaryVote::For, TernaryVote::Against,
    ]);
    println!("Byzantine consensus: {:?}", consensus);
}
```

## API Overview

| Type | Description |
|------|-------------|
| `TernaryVote` | For/Against/Abstain with ternary conversion |
| `VoteTally` | Counter for simple majority voting |
| `QuorumConfig` | Minimum participation requirements (count + fraction) |
| `QuorumSystem` | Vote tally with quorum gating |
| `WeightedVoting` | Weight-based result computation |
| `ByzantineAgreement` | Simplified Byzantine fault-tolerant consensus |
| `LiquidDemocracy` | Delegable voting with transitive resolution |
| `CondorcetElection` | Pairwise comparison election with ranking ballots |

## How It Works

**Simple tally:** Count each vote type, return whichever has the most. Ties produce Abstain.

**Quorum:** Before returning a result, check that `votes_cast >= minimum_votes` AND `votes_cast / eligible_voters >= minimum_fraction`. If quorum fails, return `None` (no valid result).

**Weighted:** Each `(weight, vote)` pair contributes its weight to the respective bucket. Highest total weight wins.

**Byzantine agreement:** Given n nodes, the system tolerates f = (n−1)/3 faulty nodes (requires n > 3f). A consensus round requires >2/3 of nodes to agree. If no supermajority, the round returns `None`.

**Liquid democracy:** Each voter either votes directly or delegates to another voter. Resolution follows delegation chains: if voter A delegates to B who delegates to C who votes directly, then A and B's weight transfers to C. Cycles are broken by the visited-set check. The final tally counts each effective vote with accumulated weight.

**Condorcet:** Each ballot is a full ranking of candidates. For every pair (A, B), count how many voters prefer A over B. A candidate who beats all others pairwise is the Condorcet winner. The method returns `None` when no such candidate exists (the Condorcet paradox: A > B > C > A).

## Known Limitations

- **Byzantine agreement is a single-round simplification.** Real Byzantine protocols (PBFT, etc.) require multiple rounds with cryptographic signatures. This implementation is suitable for testing and education, not production fault tolerance.
- **Liquid democracy delegation chains can stack weight unevenly.** A popular delegate could accumulate hundreds of votes. There's no cap on delegation depth or accumulated weight.
- **Condorcet has no tiebreaking fallback.** When no Condorcet winner exists (cyclic preferences), `winner()` returns `None` with no Schulze or ranked-pairs resolution.
- **`WeightedVoting::add_vote` stores voter_id but the `weighted_result` function ignores it.** The struct-level method doesn't integrate with the static method — an inconsistency in the current API.

## Use Cases

- **DAO governance** — Members vote on proposals with explicit abstention. Quorum prevents low-participation decisions.
- **Multi-node system consensus** — Distributed services reach agreement on configuration changes with Byzantine tolerance for crashed or misbehaving nodes.
- **Collaborative filtering** — Users express positive/negative/neutral preference on items; aggregate ternary tallies drive recommendation scores.

## Ecosystem Context

Part of the SuperInstance ternary crate family. `ternary-voting` is a standalone leaf crate. Its ternary vote values compose with `ternary-cell` for cellular consensus, `ternary-diff` for comparing vote outcomes over time, and `ternary-visualization` for rendering election results.

## License

MIT
