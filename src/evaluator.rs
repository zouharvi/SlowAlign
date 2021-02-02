use std::collections::HashSet;

pub type AlgnHard = Vec<HashSet<(usize, usize)>>;
pub type AlgnSoft = Vec<Vec<Vec<f32>>>;
pub type AlgnGold = Vec<(HashSet<(usize, usize)>, HashSet<(usize, usize)>)>;

pub fn alignment_error_rate(alignment: AlgnHard, alignment_gold: &AlgnGold) -> f32 {
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
                .map(|x| x.clone())
                .collect::<HashSet<(usize, usize)>>();
            let a_intersect_sp = algn
                .intersection(&s_union_p)
                .collect::<HashSet<&(usize, usize)>>()
                .len() as f32;

            (2.0 * a_intersect_s + a_intersect_sp) / ((algn.len() + sure.len()) as f32)
        })
        .sum();
    total / (alignment.len() as f32)
}
