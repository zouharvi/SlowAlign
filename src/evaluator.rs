use std::collections::HashSet;

pub type AlgnHard = HashSet<(usize, usize)>;
pub type AlgnSoft = Vec<Vec<f32>>;
pub type AlgnGold = (HashSet<(usize, usize)>, HashSet<(usize, usize)>);

pub fn alignment_error_rate(alignment: &[AlgnHard], alignment_gold: &[AlgnGold]) -> f32 {
    let total: f32 = alignment
        .iter()
        .zip(alignment_gold)
        .map(|(algn, (sure, poss))| {
            let a_intersect_s = algn
                .intersection(&sure)
                .collect::<HashSet<&(usize, usize)>>()
                .len() as f32;
            // this may be unnecessarily more computationally expensive, but removes the assumption of disjoint S and P
            let s_union_p = sure
                .union(&poss)
                .copied()
                .collect::<HashSet<(usize, usize)>>();
            let a_intersect_sp = algn
                .intersection(&s_union_p)
                .collect::<HashSet<&(usize, usize)>>()
                .len() as f32;
            if algn.is_empty() & sure.is_empty() {
                1.0
            } else {
                1.0 - (a_intersect_s + a_intersect_sp) / ((algn.len() + sure.len()) as f32)
            }
        })
        .sum();
    total / (alignment_gold.len() as f32)
}
