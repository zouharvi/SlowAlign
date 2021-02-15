use std::collections::HashSet;

pub type AlgnHard = HashSet<(usize, usize)>;
pub type AlgnSoft = Vec<Vec<f32>>;
pub type AlgnGold = (HashSet<(usize, usize)>, HashSet<(usize, usize)>);

/**
 * Computes AER given a proposed and gold alignment (includes sure and poss).
 * The computation is truncated to the smallest of the two alignments (sentence count wise).
 * 1 - (2*|A*S|+|A*P|/(|A|+|S|))
 **/
pub fn alignment_error_rate(alignment: &[AlgnHard], alignment_gold: &[AlgnGold]) -> f32 {
    let total: f32 = alignment
        .iter()
        .zip(alignment_gold)
        .map(|(algn, (sure, poss))| {
            let a_intersect_s = algn
                .intersection(&sure)
                .collect::<HashSet<&(usize, usize)>>()
                .len() as f32;
            // We compute the AER as 1 - (2*|A*sure|+|A*poss|/(|A|+|sure|)), S=sure, P=sure+poss.
            // The assumption is that sure and poss are disjoint. The following assert can catch assumption violations.
            assert!(sure
                .intersection(&poss)
                .collect::<HashSet<&(usize, usize)>>()
                .is_empty());
            let a_intersect_p = algn
                .intersection(&poss)
                .collect::<HashSet<&(usize, usize)>>()
                .len() as f32;
            if algn.is_empty() & sure.is_empty() {
                1.0
            } else {
                1.0 - (2.0 * a_intersect_s + a_intersect_p) / ((algn.len() + sure.len()) as f32)
            }
        })
        .sum();
    total / (usize::min(alignment.len(), alignment_gold.len()) as f32)
}

/**
 * Reverse a hard alignment.
 **/
pub fn alignment_reverse(alignment: &[AlgnHard]) -> Vec<AlgnHard> {
    alignment
        .iter()
        .map(|sent| sent.iter().map(|(x, y)| (*y, *x)).collect::<AlgnHard>())
        .collect()
}
