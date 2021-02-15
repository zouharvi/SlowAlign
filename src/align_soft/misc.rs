use crate::evaluator::AlgnSoft;
use crate::reader::{Sent, Vocab};
use crate::utils::{levenstein_score, transpose, writer};

/**
 * Soft alignment based on levenstein distance between token forms:
 * 1 - lev(x,y)/(|x|+|y|)
 **/
pub fn levenstein(sents: &[(Sent, Sent)], vocab1: &Vocab, vocab2: &Vocab) -> Vec<AlgnSoft> {
    eprintln!("Computing levenstein distance");

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
                scores[sent_i][pos2][pos1] = levenstein_score(word1_str, word2_str);
            }
        }
    }

    scores
}

/**
 * Soft alignment based on word position in the sentence:
 * 1 - ||x|/|X| - |y|/|Y||
 **/
pub fn diagonal(sents: &[(Sent, Sent)]) -> Vec<AlgnSoft> {
    eprintln!("Computing diagonals");

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

/**
 * Blurs soft alignment with a filter:
 * [0,     alpha,     0]
 * [alpha, 1-4*alpha, 0]
 * [0,     alpha,     0]
 * First and last rows/columns are omitted from this filter.
 * The overall mass therefore changes slightly.
 **/
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

/**
 * Creates soft alignment from pre-trained word translation probabilities.
 * The direction is swapped and output alignments have therefore a different shape.
 * Normalized on source token (sent1).
 **/
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

/**
 * Creates soft alignment from pre-trained word translation probabilities.
 * Normalized from target token (sent2).
 **/
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

/**
 * Weighted merge of two soft alignments. Panic checks on dimensionality fit.
 **/
pub fn merge_sum(
    alignment1: &[AlgnSoft],
    alignment2: &[AlgnSoft],
    weight1: f32,
) -> Vec<Vec<Vec<f32>>> {
    let weight2 = 1.0 - weight1;
    assert_eq!(alignment1.len(), alignment2.len());
    let mut scores = alignment1
        .iter()
        .zip(alignment2)
        .map(|(sent_a1, sent_a2)| {
            vec![
                vec![
                    0.0;
                    {
                        assert_eq!(sent_a1[0].len(), sent_a2[0].len());
                        sent_a2[0].len()
                    }
                ];
                {
                    assert_eq!(sent_a1.len(), sent_a2.len());
                    sent_a1.len()
                }
            ]
        })
        .collect::<Vec<AlgnSoft>>();

    for (sent_i, (sent_a1, sent_a2)) in alignment1.iter().zip(alignment2).enumerate() {
        for (pos2, (tgtprobs1, tgtprobs2)) in sent_a1.iter().zip(sent_a2).enumerate() {
            for (pos1, (prob1, prob2)) in tgtprobs1.iter().zip(tgtprobs2).enumerate() {
                scores[sent_i][pos2][pos1] = weight1 * prob1 + weight2 * prob2;
            }
        }
    }

    scores
}
