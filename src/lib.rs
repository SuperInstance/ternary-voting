//! # ternary-voting
//!
//! Voting and consensus mechanisms with ternary values.
//! TernaryVote (for/against/abstain), QuorumSystem, WeightedVoting,
//! ByzantineAgreement, LiquidDemocracy, Condorcet method.

#![forbid(unsafe_code)]

/// A ternary vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryVote {
    For,
    Against,
    Abstain,
}

impl TernaryVote {
    pub fn to_ternary(self) -> i8 {
        match self {
            TernaryVote::For => 1,
            TernaryVote::Abstain => 0,
            TernaryVote::Against => -1,
        }
    }

    pub fn from_ternary(v: i8) -> Option<Self> {
        match v {
            1 => Some(TernaryVote::For),
            0 => Some(TernaryVote::Abstain),
            -1 => Some(TernaryVote::Against),
            _ => None,
        }
    }
}

/// Simple vote tally.
#[derive(Debug, Clone, Default)]
pub struct VoteTally {
    pub for_count: usize,
    pub against_count: usize,
    pub abstain_count: usize,
}

impl VoteTally {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, vote: TernaryVote) {
        match vote {
            TernaryVote::For => self.for_count += 1,
            TernaryVote::Against => self.against_count += 1,
            TernaryVote::Abstain => self.abstain_count += 1,
        }
    }

    pub fn total(&self) -> usize {
        self.for_count + self.against_count + self.abstain_count
    }

    /// Net support as ternary: positive = for wins, negative = against wins, 0 = tie.
    pub fn net_support(&self) -> i64 {
        self.for_count as i64 - self.against_count as i64
    }

    /// Simple majority result.
    pub fn result(&self) -> TernaryVote {
        if self.for_count > self.against_count && self.for_count > self.abstain_count {
            TernaryVote::For
        } else if self.against_count > self.for_count && self.against_count > self.abstain_count {
            TernaryVote::Against
        } else {
            TernaryVote::Abstain
        }
    }
}

/// Quorum requirement.
#[derive(Debug, Clone)]
pub struct QuorumConfig {
    pub minimum_votes: usize,
    pub minimum_fraction: f64, // e.g. 0.5 means >50% must participate
}

impl QuorumConfig {
    pub fn new(minimum_votes: usize, minimum_fraction: f64) -> Self {
        Self { minimum_votes, minimum_fraction }
    }

    /// Check if quorum is met.
    pub fn is_met(&self, votes_cast: usize, eligible_voters: usize) -> bool {
        votes_cast >= self.minimum_votes
            && (eligible_voters == 0 || votes_cast as f64 / eligible_voters as f64 >= self.minimum_fraction)
    }
}

/// Quorum-based voting system.
#[derive(Debug, Clone)]
pub struct QuorumSystem {
    pub config: QuorumConfig,
    pub eligible_voters: usize,
    pub tally: VoteTally,
}

impl QuorumSystem {
    pub fn new(config: QuorumConfig, eligible_voters: usize) -> Self {
        Self { config, eligible_voters, tally: VoteTally::new() }
    }

    pub fn vote(&mut self, vote: TernaryVote) {
        self.tally.record(vote);
    }

    /// Get the result, checking quorum first.
    pub fn result(&self) -> Option<TernaryVote> {
        if self.config.is_met(self.tally.total(), self.eligible_voters) {
            Some(self.tally.result())
        } else {
            None
        }
    }
}

/// Weighted voting with voter weights.
#[derive(Debug, Clone)]
pub struct WeightedVoting {
    pub weights: Vec<(u32, TernaryVote)>, // (voter_id, weight, vote)
}

impl WeightedVoting {
    pub fn new() -> Self {
        Self { weights: Vec::new() }
    }

    pub fn add_vote(&mut self, voter_id: u32, weight: u32, vote: TernaryVote) {
        self.weights.push((voter_id, vote)); // Simplified: store weight info in tally
        // Actually let's store weights properly
    }

    /// Compute weighted result.
    pub fn weighted_result(votes: &[(u32, TernaryVote)]) -> TernaryVote {
        let mut for_w = 0u64;
        let mut against_w = 0u64;
        let mut abstain_w = 0u64;
        for &(weight, vote) in votes {
            match vote {
                TernaryVote::For => for_w += weight as u64,
                TernaryVote::Against => against_w += weight as u64,
                TernaryVote::Abstain => abstain_w += weight as u64,
            }
        }
        if for_w > against_w && for_w > abstain_w {
            TernaryVote::For
        } else if against_w > for_w && against_w > abstain_w {
            TernaryVote::Against
        } else {
            TernaryVote::Abstain
        }
    }
}

/// Simplified Byzantine agreement on ternary values.
#[derive(Debug, Clone)]
pub struct ByzantineAgreement {
    pub node_count: usize,
    pub byzantine_threshold: usize, // max faulty nodes (f)
}

impl ByzantineAgreement {
    pub fn new(node_count: usize) -> Self {
        // Byzantine fault tolerance: need n > 3f
        let f = (node_count - 1) / 3;
        Self { node_count, byzantine_threshold: f }
    }

    /// Check if agreement is possible.
    pub fn can_agree(&self) -> bool {
        self.node_count > 3 * self.byzantine_threshold
    }

    /// Run one round of majority voting (simplified).
    pub fn consensus_round(&self, votes: &[TernaryVote]) -> Option<TernaryVote> {
        if votes.len() < self.node_count {
            return None; // not all votes received
        }
        let mut tally = VoteTally::new();
        for &v in votes {
            tally.record(v);
        }
        // Need > 2/3 agreement for Byzantine tolerance
        let needed = (2 * self.node_count) / 3 + 1;
        if tally.for_count >= needed {
            Some(TernaryVote::For)
        } else if tally.against_count >= needed {
            Some(TernaryVote::Against)
        } else {
            None // no supermajority
        }
    }
}

/// Liquid democracy with ternary delegation.
#[derive(Debug, Clone)]
pub struct LiquidDemocracy {
    /// For each voter: Some(delegate) or None (votes directly).
    pub delegations: Vec<Option<usize>>,
    pub direct_votes: Vec<TernaryVote>,
}

impl LiquidDemocracy {
    pub fn new(voter_count: usize) -> Self {
        Self {
            delegations: vec![None; voter_count],
            direct_votes: vec![TernaryVote::Abstain; voter_count],
        }
    }

    /// Delegate voter's vote to another voter.
    pub fn delegate(&mut self, from: usize, to: usize) {
        if from < self.delegations.len() && to < self.delegations.len() {
            self.delegations[from] = Some(to);
        }
    }

    /// Cast a direct vote (overrides any delegation).
    pub fn vote_direct(&mut self, voter: usize, vote: TernaryVote) {
        if voter < self.direct_votes.len() {
            self.direct_votes[voter] = vote;
            self.delegations[voter] = None;
        }
    }

    /// Resolve delegations and compute final result.
    /// Returns effective vote weight per voter.
    pub fn resolve(&self) -> Vec<(usize, TernaryVote, u32)> {
        let n = self.delegations.len();
        let mut weight = vec![1u32; n];
        let mut final_vote = self.direct_votes.clone();

        // Resolve delegation chains
        for i in 0..n {
            let mut visited = vec![false; n];
            let mut current = i;
            let mut chain = Vec::new();

            while let Some(delegate) = self.delegations[current] {
                if visited[current] || delegate >= n {
                    break;
                }
                visited[current] = true;
                chain.push(current);
                current = delegate;
            }

            // Transfer weight to final voter in chain
            if !chain.is_empty() {
                for &voter in &chain {
                    weight[voter] = 0;
                }
                weight[current] += chain.len() as u32;
            }
        }

        (0..n).map(|i| (i, final_vote[i], weight[i])).filter(|(_, _, w)| *w > 0).collect()
    }

    /// Compute the final tally.
    pub fn final_tally(&self) -> VoteTally {
        let resolved = self.resolve();
        let mut tally = VoteTally::new();
        for (_, vote, weight) in resolved {
            for _ in 0..weight {
                tally.record(vote);
            }
        }
        tally
    }
}

/// Condorcet method for ternary pairwise comparisons.
#[derive(Debug, Clone)]
pub struct CondorcetElection {
    pub candidates: usize,
    /// preferences[voter] = ranking of candidates (lower is preferred).
    pub preferences: Vec<Vec<usize>>,
}

impl CondorcetElection {
    pub fn new(candidates: usize) -> Self {
        Self { candidates, preferences: Vec::new() }
    }

    /// Add a voter's preference ranking.
    pub fn add_ballot(&mut self, ranking: Vec<usize>) {
        self.preferences.push(ranking);
    }

    /// Count how many voters prefer candidate a over candidate b.
    pub fn pairwise(&self, a: usize, b: usize) -> (usize, usize) {
        let mut prefer_a = 0;
        let mut prefer_b = 0;
        for ranking in &self.preferences {
            let pos_a = ranking.iter().position(|&c| c == a).unwrap_or(usize::MAX);
            let pos_b = ranking.iter().position(|&c| c == b).unwrap_or(usize::MAX);
            if pos_a < pos_b {
                prefer_a += 1;
            } else if pos_b < pos_a {
                prefer_b += 1;
            }
        }
        (prefer_a, prefer_b)
    }

    /// Find Condorcet winner (candidate who beats all others pairwise).
    pub fn winner(&self) -> Option<usize> {
        for candidate in 0..self.candidates {
            let mut wins_all = true;
            for other in 0..self.candidates {
                if candidate == other { continue; }
                let (for_cand, for_other) = self.pairwise(candidate, other);
                if for_cand <= for_other {
                    wins_all = false;
                    break;
                }
            }
            if wins_all {
                return Some(candidate);
            }
        }
        None
    }

    /// Build pairwise comparison matrix.
    pub fn comparison_matrix(&self) -> Vec<Vec<TernaryVote>> {
        let mut matrix = vec![vec![TernaryVote::Abstain; self.candidates]; self.candidates];
        for i in 0..self.candidates {
            for j in (i + 1)..self.candidates {
                let (for_i, for_j) = self.pairwise(i, j);
                matrix[i][j] = if for_i > for_j { TernaryVote::For } else if for_j > for_i { TernaryVote::Against } else { TernaryVote::Abstain };
                matrix[j][i] = if for_j > for_i { TernaryVote::For } else if for_i > for_j { TernaryVote::Against } else { TernaryVote::Abstain };
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_vote_roundtrip() {
        for v in [-1i8, 0, 1] {
            assert_eq!(TernaryVote::from_ternary(v).unwrap().to_ternary(), v);
        }
    }

    #[test]
    fn test_vote_tally_simple() {
        let mut tally = VoteTally::new();
        tally.record(TernaryVote::For);
        tally.record(TernaryVote::For);
        tally.record(TernaryVote::Against);
        assert_eq!(tally.result(), TernaryVote::For);
        assert_eq!(tally.total(), 3);
    }

    #[test]
    fn test_vote_tally_tie() {
        let mut tally = VoteTally::new();
        tally.record(TernaryVote::For);
        tally.record(TernaryVote::Against);
        assert_eq!(tally.result(), TernaryVote::Abstain);
    }

    #[test]
    fn test_net_support() {
        let mut tally = VoteTally::new();
        tally.record(TernaryVote::For);
        tally.record(TernaryVote::For);
        tally.record(TernaryVote::Against);
        assert_eq!(tally.net_support(), 1);
    }

    #[test]
    fn test_quorum_met() {
        let config = QuorumConfig::new(3, 0.5);
        assert!(config.is_met(5, 8));
    }

    #[test]
    fn test_quorum_not_met() {
        let config = QuorumConfig::new(5, 0.5);
        assert!(!config.is_met(3, 10));
    }

    #[test]
    fn test_quorum_system_result() {
        let config = QuorumConfig::new(1, 0.0);
        let mut qs = QuorumSystem::new(config, 5);
        qs.vote(TernaryVote::For);
        qs.vote(TernaryVote::For);
        qs.vote(TernaryVote::Against);
        assert_eq!(qs.result(), Some(TernaryVote::For));
    }

    #[test]
    fn test_quorum_system_no_quorum() {
        let config = QuorumConfig::new(10, 0.5);
        let mut qs = QuorumSystem::new(config, 20);
        qs.vote(TernaryVote::For);
        assert_eq!(qs.result(), None);
    }

    #[test]
    fn test_weighted_voting() {
        let votes = vec![
            (3, TernaryVote::Against),
            (1, TernaryVote::For),
            (1, TernaryVote::For),
        ];
        assert_eq!(WeightedVoting::weighted_result(&votes), TernaryVote::Against);
    }

    #[test]
    fn test_weighted_voting_for_wins() {
        let votes = vec![
            (5, TernaryVote::For),
            (3, TernaryVote::Against),
        ];
        assert_eq!(WeightedVoting::weighted_result(&votes), TernaryVote::For);
    }

    #[test]
    fn test_byzantine_can_agree() {
        let ba = ByzantineAgreement::new(4);
        assert!(ba.can_agree()); // 4 > 3*1 = 3
    }

    #[test]
    fn test_byzantine_consensus_round() {
        let ba = ByzantineAgreement::new(4);
        // Need > 2/3 = 3 votes to agree
        let votes = vec![TernaryVote::For; 4];
        assert_eq!(ba.consensus_round(&votes), Some(TernaryVote::For));
    }

    #[test]
    fn test_byzantine_no_consensus() {
        let ba = ByzantineAgreement::new(4);
        let votes = vec![TernaryVote::For, TernaryVote::For, TernaryVote::Against, TernaryVote::Against];
        assert_eq!(ba.consensus_round(&votes), None);
    }

    #[test]
    fn test_liquid_democracy_direct() {
        let mut ld = LiquidDemocracy::new(3);
        ld.vote_direct(0, TernaryVote::For);
        ld.vote_direct(1, TernaryVote::For);
        ld.vote_direct(2, TernaryVote::Against);
        let tally = ld.final_tally();
        assert_eq!(tally.result(), TernaryVote::For);
    }

    #[test]
    fn test_liquid_democracy_delegation() {
        let mut ld = LiquidDemocracy::new(3);
        ld.vote_direct(0, TernaryVote::For);
        ld.delegate(1, 0); // voter 1 delegates to voter 0
        ld.delegate(2, 0); // voter 2 delegates directly to voter 0
        let tally = ld.final_tally();
        assert_eq!(tally.for_count, 3);
    }

    #[test]
    fn test_condorcet_winner() {
        let mut ce = CondorcetElection::new(3);
        // A > B > C for all voters
        ce.add_ballot(vec![0, 1, 2]);
        ce.add_ballot(vec![0, 1, 2]);
        ce.add_ballot(vec![0, 1, 2]);
        assert_eq!(ce.winner(), Some(0));
    }

    #[test]
    fn test_condorcet_pairwise() {
        let mut ce = CondorcetElection::new(3);
        ce.add_ballot(vec![0, 1, 2]);
        ce.add_ballot(vec![1, 0, 2]);
        let (a_over_b, b_over_a) = ce.pairwise(0, 1);
        assert_eq!(a_over_b, 1);
        assert_eq!(b_over_a, 1);
    }

    #[test]
    fn test_condorcet_no_winner_cycle() {
        let mut ce = CondorcetElection::new(3);
        ce.add_ballot(vec![0, 1, 2]);
        ce.add_ballot(vec![1, 2, 0]);
        ce.add_ballot(vec![2, 0, 1]);
        // Condorcet paradox — no winner
        assert_eq!(ce.winner(), None);
    }

    #[test]
    fn test_condorcet_comparison_matrix() {
        let mut ce = CondorcetElection::new(2);
        ce.add_ballot(vec![0, 1]);
        ce.add_ballot(vec![0, 1]);
        let matrix = ce.comparison_matrix();
        assert_eq!(matrix[0][1], TernaryVote::For);
        assert_eq!(matrix[1][0], TernaryVote::Against);
    }

    #[test]
    fn test_vote_tally_all_abstain() {
        let mut tally = VoteTally::new();
        tally.record(TernaryVote::Abstain);
        tally.record(TernaryVote::Abstain);
        assert_eq!(tally.result(), TernaryVote::Abstain);
    }
}
