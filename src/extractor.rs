use crate::evaluator::{AlgnHard, AlgnSoft};

fn argmax(probs: &[f32]) -> usize {
    probs
        .iter()
        .enumerate()
        .max_by(|(_, value0), (_, value1)| value0.partial_cmp(value1).unwrap())
        .map(|(idx, _)| idx)
        .unwrap()
}

pub fn a1_argmax(alignment_probs: &[AlgnSoft]) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            sent_prob
                .iter()
                .enumerate()
                .map(|(pos1, tgt_probs)| (pos1, argmax(tgt_probs)))
                .collect()
        })
        .collect()
}

pub fn a2_threshold(alignment_probs: &[AlgnSoft], threshold: f32) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            sent_prob
                .iter()
                .enumerate()
                .map(|(pos1, tgt_probs)| {
                    tgt_probs
                        .iter()
                        .enumerate()
                        .filter(|(_pos2, prob)| **prob >= threshold)
                        .map(|(pos2, _)| (pos1, pos2))
                        .collect::<Vec<(usize, usize)>>()
                })
                .flatten()
                .collect()
        })
        .collect()
}
