use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    const SENT_COUNT: usize = 100000;
    const E_THREADS: usize = 8;
    const E_CHUNKS: usize = SENT_COUNT / E_THREADS;
    const M_THREADS: usize = 8;
    // load data
    let file1 = File::open(Path::new("data/hansards.e")).unwrap();
    let file2 = File::open(Path::new("data/hansards.f")).unwrap();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    let mut sents = Vec::<(Vec<usize>, Vec<usize>)>::with_capacity(SENT_COUNT);
    let mut vocab1 = HashMap::<String, usize>::new();
    let mut vocab2 = HashMap::<String, usize>::new();
    for (line1, line2) in reader1.lines().zip(reader2.lines()).take(SENT_COUNT) {
        let line1 = line1.unwrap();
        let line2 = line2.unwrap();
        let tokens1 = line1
            .split_whitespace()
            .map(|token| {
                if !vocab1.contains_key(token) {
                    vocab1.insert(token.to_string(), vocab1.keys().len());
                }
                *vocab1.get(token).unwrap()
            })
            .collect();
        let tokens2 = line2
            .split_whitespace()
            .map(|token| {
                if !vocab2.contains_key(token) {
                    vocab2.insert(token.to_string(), vocab2.keys().len());
                }
                *vocab2.get(token).unwrap()
            })
            .collect();
        sents.push((tokens1, tokens2));
    }
    let mut alignment_probs = sents
        .iter()
        .map(|(s1, s2)| vec![vec![1.0 / (s1.len() as f32); s1.len()]; s2.len()])
        .collect::<Vec<Vec<Vec<f32>>>>();
    let v1_len = vocab1.keys().len();
    let v2_len = vocab2.keys().len();
    let m_chunks: usize = v1_len / M_THREADS;
    // EM loop
    for step in 0..5 {
        eprintln!("step {}", step);
        // expectation
        let mut word_probs = vec![vec![0.0; v1_len]; v2_len];
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
            for (chunk_i, alignment_probs_slice) in alignment_probs.chunks_mut(E_CHUNKS).enumerate()
            {
                // shadow variables so that they are taken as immutable references
                let sents = &sents;
                let word_probs = &word_probs;
                scope.spawn(move |_| {
                    // maximization
                    for (sent_i, (s1, s2)) in sents[chunk_i * E_CHUNKS..(chunk_i + 1) * E_CHUNKS]
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

    // compute alignments, complicated way to get argmax
    let alignment = alignment_probs.iter().map(|sent_prob| {
        sent_prob.iter().enumerate().map(|(pos1, tgt_probs)| {
            (
                pos1,
                tgt_probs
                    .iter()
                    .enumerate()
                    .max_by(|(_, value0), (_, value1)| value0.partial_cmp(value1).unwrap())
                    .map(|(idx, _)| idx)
                    .unwrap(),
            )
        })
    });

    for sent_align in alignment {
        println!(
            "{}",
            sent_align
                .map(|(pos1, pos2)| format!("{}-{}", pos1, pos2))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
