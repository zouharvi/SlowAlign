use crate::reader::{Vocab, VocabRev};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn write_dict(
    file: String,
    word_probs: &Vec<Vec<f32>>,
    vocab1: &Vocab,
    vocab2: &Vocab,
    threshold: f32,
) {
    // load data
    let mut file = File::create(&file).unwrap();

    let vocab1rev = vocab_rev(vocab1);
    let vocab2rev = vocab_rev(vocab2);

    for (w2_i, w1_vec) in word_probs.iter().enumerate() {
        let w2 = vocab2rev.get(&w2_i).unwrap();
        for (w1_i, prob) in w1_vec.iter().enumerate() {
            let w1 = vocab1rev.get(&w1_i).unwrap();
            if *prob >= threshold {
                write!(&mut file, "0\t{}\t{}\t{}\n", prob, w1, w2);
            }
        }
    }
}

pub fn vocab_rev(vocab: &Vocab) -> VocabRev {
    vocab
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<HashMap<usize, String>>()
}