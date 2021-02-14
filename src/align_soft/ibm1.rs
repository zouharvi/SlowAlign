use crate::evaluator::AlgnSoft;
use std::cmp;
use std::collections::HashMap;

const E_THREADS: usize = 4;
const M_THREADS: usize = 4;

/**
 * A very simple alignment based on IBM Model 1 (without NULL tokens).
 * Returns alignment probabilities. For translation probabilities see ibm1_raw.
 * Vocabularies are provided for their dimensionality.
 **/
pub fn ibm1(
    sents: &[(Vec<usize>, Vec<usize>)],
    vocab1: &HashMap<String, usize>,
    vocab2: &HashMap<String, usize>,
) -> Vec<AlgnSoft> {
    ibm1_raw(sents, vocab1, vocab2).0
}

/**
 * A very simple alignment based on IBM Model 1 (without NULL tokens).
 * Returns alignment probabilities together with wordtranslation probabilities.
 * Vocabularies are provided for their dimensionality.
 **/
pub fn ibm1_raw(
    sents: &[(Vec<usize>, Vec<usize>)],
    vocab1: &HashMap<String, usize>,
    vocab2: &HashMap<String, usize>,
) -> (Vec<AlgnSoft>, Vec<Vec<f32>>) {
    let mut alignment_probs = sents
        .iter()
        .map(|(s1, s2)| vec![vec![1.0 / (s1.len() as f32); s1.len()]; s2.len()])
        .collect::<Vec<AlgnSoft>>();
    let v1_len = vocab1.len();
    let v2_len = vocab2.len();
    let m_chunks: usize = v1_len / M_THREADS;
    let sent_count: usize = sents.len();
    let e_chunks: usize = sent_count / E_THREADS;
    let mut word_probs = vec![vec![]];
    // EM loop
    for step in 0..5 {
        eprintln!("step {}", step);

        // expectation
        word_probs = vec![vec![0.0; v1_len]; v2_len];
        for ((s1, s2), probs) in sents.iter().zip(alignment_probs.iter()) {
            for (word_tgt, probs_tgt) in s2.iter().zip(probs.iter()) {
                for (word_src, partial_count) in s1.iter().zip(probs_tgt) {
                    word_probs[*word_tgt][*word_src] += *partial_count;
                }
            }
        }

        // thread at least normalization
        let _ = crossbeam::scope(|scope| {
            for word_probs in word_probs.chunks_mut(m_chunks) {
                scope.spawn(move |_| {
                    // normalize rows
                    for word_prob in word_probs.iter_mut() {
                        let sum = word_prob.iter().sum::<f32>();
                        for prob in word_prob.iter_mut() {
                            *prob /= sum;
                        }
                    }
                });
            }
        });

        let _ = crossbeam::scope(|scope| {
            // chop alignment_probs into disjoint sub-slices
            for (chunk_i, alignment_probs_slice) in alignment_probs.chunks_mut(e_chunks).enumerate()
            {
                // shadow variables so that they are taken as immutable references
                let sents = &sents;
                let word_probs = &word_probs;
                scope.spawn(move |_| {
                    // maximization
                    for (sent_i, (s1, s2)) in sents
                        [chunk_i * e_chunks..cmp::min(sent_count, (chunk_i + 1) * e_chunks)]
                        .iter()
                        .enumerate()
                    {
                        let sent_probs = &mut alignment_probs_slice[sent_i];
                        for (pos1, word1) in s1.iter().enumerate() {
                            for (pos2, word2) in s2.iter().enumerate() {
                                sent_probs[pos2][pos1] = word_probs[*word2][*word1];
                            }
                        }
                        // normalize columns
                        for (pos1, _) in s1.iter().enumerate() {
                            let sum = sent_probs
                                .iter()
                                .map(|tgt_probs| tgt_probs[pos1])
                                .sum::<f32>();
                            for tgt_probs in sent_probs.iter_mut() {
                                tgt_probs[pos1] /= sum;
                            }
                        }
                    }
                });
            }
        });
    }

    (alignment_probs, word_probs)
}
