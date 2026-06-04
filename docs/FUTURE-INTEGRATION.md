# Future Integration: ternary-voting

## Current State
Provides ternary voting (for/against/abstain), quorum systems, weighted voting, Byzantine agreement protocols, liquid democracy with delegation graphs, and Condorcet methods for preference ordering.

## Integration Opportunities

### With ternary-consensus / ternary-room
Room-level decisions require consensus among ensigns and agents. `TernaryVote` maps to: for = proceed, against = block, abstain = no opinion. `QuorumConfig` ensures minimum participation. `ByzantineAgreement` handles the case where some ensigns may be malfunctioning — critical for distributed rooms where not all agents are trustworthy.

### With ternary-cell
Cell grids making collective decisions use voting. When multiple cells produce conflicting predictions, a vote resolves the conflict. `WeightedVoting` where cell weight is proportional to accumulated surprise (high-surprise cells have earned more credibility). `LiquidDemocracy` where cells delegate their vote to neighbors they trust.

### With ternary-swarm
Swarm decisions are inherently voting problems. Each particle votes for its preferred direction. Condorcet methods find the direction that would beat all others in pairwise comparison — the true consensus direction, not just plurality.

## Potential in Mature Systems
In room-as-codespace, rooms vote on shared resource allocation. Byzantine agreement ensures the fleet reaches consensus even when some Codespaces are compromised. Liquid democracy models the natural delegation pattern: agents delegate to ensigns, ensigns delegate to room leaders.

## Cross-Pollination Ideas
- Abstain as a first-class signal: rooms that abstain are signaling indifference, not disagreement
- Condorcet winner as the room most preferred by all agents — optimal meeting room
- Byzantine agreement as the protocol for fleet-wide configuration changes

## Dependencies for Next Steps
- ternary-consensus integration for multi-round voting protocols
- ternary-room needs a `RoomVote` mechanism for resource arbitration
- ternary-cell needs conflict resolution via weighted voting
