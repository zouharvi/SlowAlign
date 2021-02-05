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
                .map(|(pos2, tgt_probs)| (argmax(tgt_probs), pos2))
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
                .map(|(pos2, tgt_probs)| {
                    tgt_probs
                        .iter()
                        .enumerate()
                        .filter(|(_pos1, prob)| **prob >= threshold)
                        .map(|(pos1, _)| (pos1, pos2))
                        .collect::<Vec<(usize, usize)>>()
                })
                .flatten()
                .collect()
        })
        .collect()
}

pub fn a4_threshold_dynamic(alignment_probs: &[AlgnSoft], alpha: f32) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            sent_prob
                .iter()
                .enumerate()
                .map(|(pos2, src_probs)| {
                    let threshold = alpha * src_probs.iter().cloned().fold(f32::NAN, f32::max);

                    src_probs
                        .iter()
                        .enumerate()
                        .filter(|(_pos1, prob)| **prob >= threshold)
                        .map(|(pos1, _)| (pos1, pos2))
                        .collect::<Vec<(usize, usize)>>()
                })
                .flatten()
                .collect()
        })
        .collect()
}

pub fn a3_threshold_dynamic(alignment_probs: &[AlgnSoft], alpha: f32) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            let mut sent_prob_rev = vec![vec![0.0; sent_prob.len()]; sent_prob[0].len()];
            for (pos2, src_probs) in sent_prob.iter().enumerate() {
                for (pos1, prob) in src_probs.iter().enumerate() {
                    sent_prob_rev[pos1][pos2] = *prob;
                }
            }

            sent_prob_rev
                .iter()
                .enumerate()
                .map(|(pos1, tgt_probs)| {
                    let threshold = alpha * tgt_probs.iter().cloned().fold(f32::NAN, f32::max);

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