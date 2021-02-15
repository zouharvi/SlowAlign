use crate::utils::transpose;
use crate::utils::argmax;
use crate::evaluator::{AlgnHard, AlgnSoft};

/**
 * Extracts the argmax from the target side.
 **/
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

/**
 * Extracts all alignments with a score of at least some threshold.
 **/
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

/**
 * Extracts all alignments with a score of at least a multiple of the best one on the target side.
 * Treatment for negative scores is not implemented (panic).
 **/
pub fn a3_threshold_dynamic(alignment_probs: &[AlgnSoft], alpha: f32) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            let sent_prob_rev = transpose(sent_prob);
            
            sent_prob_rev
                .iter()
                .enumerate()
                .map(|(pos1, tgt_probs)| {
                    let threshold = alpha * tgt_probs.iter().cloned().fold(f32::NAN, f32::max);
                    assert!(threshold >= 0.0);

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

/**
 * Extracts all alignments with a score of at least a multiple of the best one on the source side.
 * Treatment for negative scores is not implemented (panic).
 **/
pub fn a4_threshold_dynamic(alignment_probs: &[AlgnSoft], alpha: f32) -> Vec<AlgnHard> {
    alignment_probs
        .iter()
        .map(|sent_prob| {
            sent_prob
                .iter()
                .enumerate()
                .map(|(pos2, src_probs)| {
                    let threshold = alpha * src_probs.iter().cloned().fold(f32::NAN, f32::max);
                    assert!(threshold >= 0.0);
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
