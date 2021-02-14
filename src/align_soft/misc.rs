use crate::evaluator::AlgnSoft;
use crate::reader::{Sent, Vocab};
use crate::utils::{levenstein_distance, transpose, writer};

pub fn levenstein(sents: &[(Sent, Sent)], vocab1: &Vocab, vocab2: &Vocab) -> Vec<AlgnSoft> {
    let vocab1rev = writer::vocab_rev(vocab1);
    let vocab2rev = writer::vocab_rev(vocab2);

    let mut scores = sents
        .iter()
        .map(|(s1, s2)| vec![vec![0.0; s1.len()]; s2.len()])
        .collect::<Vec<AlgnSoft>>();
    for (sent_i, (sent1, sent2)) in sents.iter().enumerate() {
        for (pos1, word1) in sent1.iter().enumerate() {
            let word1_str = vocab1rev.get(word1).unwrap();
            for (pos2, word2) in sent2.iter().enumerate() {
                let word2_str = vocab2rev.get(word2).unwrap();
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

pub fn blur(alignment_probs: &[AlgnSoft], alpha: f32) -> Vec<AlgnSoft> {
    let mut scores = alignment_probs
        .iter()
        .map(|sent| vec![vec![0.0; sent[0].len()]; sent.len()])
        .collect::<Vec<AlgnSoft>>();
    let center_alpha = 1.0 - 4.0 * alpha;
    for (sent_i, sent) in alignment_probs.iter().enumerate() {
        for (pos2, tgt_probs) in sent.iter().enumerate() {
            for (pos1, _prob) in tgt_probs.iter().enumerate() {
                scores[sent_i][pos2][pos1] = {
                    if pos1 == 0
                        || pos2 == 0
                        || pos1 == tgt_probs.len() - 1
                        || pos2 == sent.len() - 1
                    {
                        alignment_probs[sent_i][pos2][pos1]
                    } else {
                        0.0 + alpha * alignment_probs[sent_i][pos2 - 1][pos1]
                            + alpha * alignment_probs[sent_i][pos2 + 1][pos1]
                            + center_alpha * alignment_probs[sent_i][pos2][pos1]
                            + alpha * alignment_probs[sent_i][pos2][pos1 - 1]
                            + alpha * alignment_probs[sent_i][pos2][pos1 + 1]
                    }
                }
            }
        }
    }

    scores
}

pub fn from_dic_rev(
    sents: &[(Sent, Sent)],
    vocab1: &Vocab,
    vocab2: &Vocab,
    dic: &[Vec<f32>],
    dic_vocab1: &Vocab,
    dic_vocab2: &Vocab,
) -> Vec<AlgnSoft> {
    let sents_rev = &sents
        .iter()
        .map(|(s1, s2)| (s2.clone(), s1.clone()))
        .collect::<Vec<(Sent, Sent)>>();
    from_dic(
        sents_rev,
        vocab2,
        vocab1,
        &transpose(dic.to_owned()),
        dic_vocab2,
        dic_vocab1,
    )
}

pub fn from_dic(
    sents: &[(Sent, Sent)],
    vocab1: &Vocab,
    vocab2: &Vocab,
    dic: &[Vec<f32>],
    dic_vocab1: &Vocab,
    dic_vocab2: &Vocab,
) -> Vec<AlgnSoft> {
    let mut scores = sents
        .iter()
        .map(|(sent1, sent2)| vec![vec![0.0; sent1.len()]; sent2.len()])
        .collect::<Vec<AlgnSoft>>();

    let vocab1rev = writer::vocab_rev(vocab1);
    let vocab2rev = writer::vocab_rev(vocab2);

    for (sent_i, (sent1, sent2)) in sents.iter().enumerate() {
        for (pos2, w2_i) in sent2.iter().enumerate() {
            let w2 = vocab2rev.get(w2_i).unwrap();
            for (pos1, w1_i) in sent1.iter().enumerate() {
                let w1 = vocab1rev.get(w1_i).unwrap();
                scores[sent_i][pos2][pos1] = {
                    // set translation probability if both present in the dictionary
                    if dic_vocab2.contains_key(w2) && dic_vocab1.contains_key(w1) {
                        dic[*dic_vocab2.get(w2).unwrap()][*dic_vocab1.get(w1).unwrap()]
                    } else {
                        0.0
                    }
                }
            }
            for (pos1, _) in sent1.iter().enumerate() {
                let sum = scores[sent_i]
                    .iter()
                    .map(|tgt_probs| tgt_probs[pos1])
                    .sum::<f32>();
                for tgt_probs in scores[sent_i].iter_mut() {
                    if sum == 0.0 {
                        tgt_probs[pos1] = 0.0;
                    } else {
                        tgt_probs[pos1] /= sum;
                    }
                }
            }
        }
    }

    scores
}
