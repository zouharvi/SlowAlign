use crate::evaluator::AlgnSoft;
use crate::reader::{Sent, Vocab};
use crate::utils::levenstein_distance;
use std::collections::HashMap;

pub fn levenstein(sents: &[(Sent, Sent)], vocab1: &Vocab, vocab2: &Vocab) -> Vec<AlgnSoft> {
    let vocab1back = vocab1
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<&usize, &String>>();
    let vocab2back = vocab2
        .iter()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<&usize, &String>>();

    let mut scores = sents
        .iter()
        .map(|(s1, s2)| vec![vec![0.0; s1.len()]; s2.len()])
        .collect::<Vec<AlgnSoft>>();
    for (sent_i, (sent1, sent2)) in sents.iter().enumerate() {
        for (pos1, word1) in sent1.iter().enumerate() {
            let word1_str = vocab1back.get(word1).unwrap();
            for (pos2, word2) in sent2.iter().enumerate() {
                let word2_str = vocab2back.get(word2).unwrap();
                scores[sent_i][pos2][pos1] = 1.0
                    - levenstein_distance(word1_str, word2_str)
                        / ((word1_str.len() + word2_str.len()) as f32);
            }
        }
    }

    scores
}

pub fn diagonal(sents: &[(Sent, Sent)]) -> Vec<AlgnSoft> {
    let mut scores = sents
        .iter()
        .map(|(s1, s2)| vec![vec![0.0; s1.len()]; s2.len()])
        .collect::<Vec<AlgnSoft>>();
    for (sent_i, (sent1, sent2)) in sents.iter().enumerate() {
        let sent1_len = sent1.len() as f32;
        let sent2_len = sent2.len() as f32;
        for (pos1, _word1) in sent1.iter().enumerate() {
            for (pos2, _word2) in sent2.iter().enumerate() {
                scores[sent_i][pos2][pos1] =
                    1.0 - ((pos1 as f32) / sent1_len - (pos2 as f32) / sent2_len).abs();
            }
        }
    }

    scores
}
