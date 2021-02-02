use crate::evaluator::AlgnSoft;
use std::collections::HashMap;

fn levenshtein_distance(word1: &str, word2: &str) -> f32 {
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    let word1_length = w1.len() + 1;
    let word2_length = w2.len() + 1;
    let mut matrix = vec![vec![0]];

    for i in 1..word1_length {
        matrix[0].push(i);
    }
    for j in 1..word2_length {
        matrix.push(vec![j]);
    }

    for j in 1..word2_length {
        for i in 1..word1_length {
            let x: usize = if w1[i - 1] == w2[j - 1] {
                matrix[j - 1][i - 1]
            } else {
                1 + std::cmp::min(
                    std::cmp::min(matrix[j][i - 1], matrix[j - 1][i]),
                    matrix[j - 1][i - 1],
                )
            };
            matrix[j].push(x);
        }
    }
    matrix[word2_length - 1][word1_length - 1] as f32
}

pub fn levenstein_align(
    sents: &[(Vec<usize>, Vec<usize>)],
    vocab1: &HashMap<String, usize>,
    vocab2: &HashMap<String, usize>,
) -> Vec<Vec<Vec<f32>>> {
    let vocab1back = vocab1.iter().map(|(k,v)| (v,k)).collect::<HashMap<&usize, &String>>();
    let vocab2back = vocab2.iter().map(|(k,v)| (v,k)).collect::<HashMap<&usize, &String>>();

    let mut scores = sents
        .iter()
        .map(|(s1, s2)| vec![vec![0.0; s1.len()]; s2.len()])
        .collect::<Vec<AlgnSoft>>();
    for (sent_i, (sent1, sent2)) in sents.iter().enumerate() {
        for (pos1, word1) in sent1.iter().enumerate() {
            let word1_str = vocab1back.get(word1).unwrap();
            for (pos2, word2) in sent2.iter().enumerate() {
                let word2_str = vocab2back.get(word2).unwrap();
                scores[sent_i][pos2][pos1] =
                    1.0 - levenshtein_distance(word1_str, word2_str) / ((word1_str.len() + word2_str.len()) as f32);
            }
        }
    }

    scores
}
