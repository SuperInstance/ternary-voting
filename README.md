# ternary-voting

**Voting and consensus mechanisms with ternary values — simple majority, quorum systems, weighted voting, Byzantine agreement, liquid democracy, and Condorcet elections.**

## Background

Consensus is the fundamental problem of distributed computing. From Paxos (Lamport, 1998) to Raft (Ongaro & Ousterhout, 2014) to blockchain consensus (Nakamoto, 2008), the question is the same: how do independent nodes agree on a value? Traditional systems use binary consensus (yes/no), but many decisions benefit from a third option: abstain. Parliamentary systems have long recognized this — votes are "for," "against," or "abstain."

`ternary-voting` implements six consensus and voting mechanisms, all using ternary votes (`For`/`Against`/`Abstain`, mapped to +1/−1/0). From simple tallies to Byzantine fault tolerance to liquid democracy, the crate provides a comprehensive toolkit for collective decision-making in the SuperInstance ecosystem.

## How It Works

### Core Types

`TernaryVote` has three variants: `For` (+1), `Against` (−1), `Abstain` (0). `VoteTally` accumulates counts and computes:

- **`net_support()`** — `for_count − against_count` (abstentions don't affect net)
- **`result()`** — plurality winner among the three categories

### Quorum Systems

`QuorumSystem` enforces participation requirements before a vote is valid:

- **Minimum votes** — absolute count threshold
- **Minimum fraction** — percentage of eligible voters (e.g., 50%+ participation)

A vote with insufficient participation returns `None` (no result), preventing a small minority from making decisions for the whole group.

### Weighted Voting

`WeightedVoting` assigns different weights to different voters. Some voters (e.g., domain experts, high-stakes stakeholders) have more influence. The weighted result sums weights per vote category rather than counting individual votes.

### Byzantine Agreement

`ByzantineAgreement` implements simplified Byzantine fault tolerance:

- **Fault tolerance** — tolerates up to `f` faulty nodes where `n > 3f` (classic BFT bound)
- **Consensus** — requires >2/3 agreement for a decision (protecting against Byzantine adversaries who can send conflicting votes to different nodes)
- **`can_agree()`** — checks if the node count satisfies `n > 3f`

This mirrors the guarantees of PBFT (Castro & Liskov, 1999) and modern BFT protocols like Tendermint.

### Liquid Democracy

`LiquidDemocracy` implements delegative voting:

- **Direct voting** — a voter casts their own vote
- **Delegation** — a voter delegates their vote to another voter (transitively)
- **Delegation resolution** — chains of delegation are resolved to find the final voter and accumulated weight

Delegation cycles are detected and broken. The final tally weights each effective voter by the number of delegations they've accumulated. This combines the knowledge of representative democracy with the flexibility of direct voting.

### Condorcet Elections

`CondorcetElection` implements the Condorcet method for multi-candidate elections:

- **Pairwise comparison** — every candidate is compared head-to-head against every other
- **Condorcet winner** — a candidate who beats all others pairwise (if one exists)
- **Comparison matrix** — full pairwise results as a ternary matrix

The Condorcet paradox (cyclic preferences: A > B > C > A) is handled by returning `None` when no Condorcet winner exists — the classic impossibility result that motivates ranked-choice voting systems.

## Experimental Results

The test suite (20+ tests) validates:

- **Vote roundtrip** — `TernaryVote` ↔ `i8` conversion is lossless
- **Tally mechanics** — simple majority, ties, net support, all-abstain
- **Quorum enforcement** — met/not-met based on both minimum votes and fraction
- **Weighted results** — weights correctly determine outcomes
- **Byzantine tolerance** — `can_agree()` for valid `n`, consensus with sufficient agreement, no consensus with split votes
- **Liquid democracy** — direct voting, delegation chains, cycle detection
- **Condorcet** — clear winner, tied pairwise, Condorcet paradox (no winner), comparison matrix

## Impact

The crate's six mechanisms cover the full spectrum of collective decision-making:

| Mechanism | Best For | Fault Tolerance |
|-----------|----------|----------------|
| Simple tally | Small groups, quick polls | None |
| Quorum | Governance, policy decisions | Low (participation check) |
| Weighted | Stakeholder voting, expert panels | None |
| Byzantine | Distributed consensus | Byzantine (n > 3f) |
| Liquid democracy | Large groups with expertise | None (delegation-based) |
| Condorcet | Multi-candidate elections | None (paradox possible) |

The ternary vote model (for/against/abstain) is strictly more expressive than binary yes/no voting. Abstention carries semantic weight: "I participated but have no preference," which is distinct from non-participation. This distinction matters in quorum systems where participation rate affects result validity.

## Use Cases

1. **Fleet configuration consensus** — Rooms use `ByzantineAgreement` to agree on configuration changes. With `n > 3f`, the system tolerates up to 1/3 faulty (or malicious) rooms while still reaching consensus.

2. **Governance voting** — Fleet-wide policy decisions use `QuorumSystem` with a 50% participation requirement and simple majority. Abstentions count toward quorum but don't affect the outcome.

3. **Expert delegation** — In a large fleet, `LiquidDemocracy` allows rooms to delegate votes to domain experts. A room that doesn't understand storage configuration delegates to a storage-specialist room, combining scalability with expertise.

4. **Multi-candidate leader election** — `CondorcetElection` selects a fleet coordinator. Each room ranks candidates, and the pairwise-comparison matrix identifies the candidate who would beat any other in a head-to-head matchup.

5. **Stake-weighted resource allocation** — `WeightedVoting` allocates resources proportionally to stake or contribution level, ensuring that rooms that contribute more to the fleet have proportionally more say in resource decisions.

## Open Questions

- **Byzantine rounds:** The current Byzantine implementation is a single-round majority vote. Production BFT protocols (PBFT, HotStuff) use multi-round protocols with view changes. Should the crate support multi-round consensus?
- **Vote privacy:** All votes are public. Should the crate integrate with `ternary-zkp` for zero-knowledge voting (prove your vote is valid without revealing which way you voted)?
- **Delegation transparency:** Liquid democracy delegation chains are resolved internally. Should delegations be publicly auditable (who delegated to whom) for transparency and accountability?

## Connection to Oxide Stack

`ternary-voting` is the governance layer:

- **`ternary-channel`** — votes are transported between rooms via channels
- **`ternary-protocol`** — vote serialization for cross-node consensus
- **`ternary-game-theory`** — voting is a game; strategic voting can be analyzed with game-theoretic tools
- **`ternary-event`** — vote events (vote.cast, consensus.reached) for observability
- **`ternary-blockchain`** — Byzantine agreement underpins blockchain consensus
- **`ternary-zkp`** — future zero-knowledge voting proofs

The ternary vote model (for/against/abstain) maps directly to the balanced ternary values used throughout the ecosystem, ensuring that voting results are immediately consumable by other ternary systems.
