use crate::evaluator::AlgnGold;
use crate::utils::cli::OptsMain;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Vocab = HashMap<String, usize>;
pub type VocabRev = HashMap<usize, String>;
pub type Sent = Vec<usize>;
pub type SentText = Vec<String>;

pub fn load(
    file1: &str,
    file2: &str,
    start: usize,
    count: usize,
) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    let reader1 = BufReader::new(File::open(&file1).unwrap());
    let reader2 = BufReader::new(File::open(&file2).unwrap());
    let mut sents = Vec::<(Vec<usize>, Vec<usize>)>::new();
    let mut vocab1 = HashMap::<String, usize>::new();
    let mut vocab2 = HashMap::<String, usize>::new();

    for (line1, line2) in reader1.lines().zip(reader2.lines()).skip(start).take(count) {
        let line1 = line1.unwrap();
        let line2 = line2.unwrap();
        let tokens1 = line1
            .split_whitespace()
            .map(|token| {
                let len = vocab1.len();
                *vocab1.entry(token.to_string()).or_insert(len)
            })
            .collect();
        let tokens2 = line2
            .split_whitespace()
            .map(|token| {
                let len = vocab2.len();
                *vocab2.entry(token.to_string()).or_insert(len)
            })
            .collect();
        sents.push((tokens1, tokens2));
    }

    (sents, (vocab1, vocab2))
}

pub fn load_all(file1: &str, file2: &str) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    load(file1, file2, 0, usize::MAX)
}

pub fn load_sent(sents1: &str, sents2: &str) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    let mut sents = Vec::<(Vec<usize>, Vec<usize>)>::new();
    let mut vocab1 = HashMap::<String, usize>::new();
    let mut vocab2 = HashMap::<String, usize>::new();
    let reader1 = sents1.split('\n').collect::<Vec<&str>>();
    let reader2 = sents2.split('\n').collect::<Vec<&str>>();

    for (line1, line2) in reader1.iter().zip(reader2.iter()) {
        let tokens1 = line1
            .split_whitespace()
            .map(|token| {
                let len = vocab1.len();
                *vocab1.entry(token.to_string()).or_insert(len)
            })
            .collect();
        let tokens2 = line2
            .split_whitespace()
            .map(|token| {
                let len = vocab2.len();
                *vocab2.entry(token.to_string()).or_insert(len)
            })
            .collect();
        sents.push((tokens1, tokens2));
    }

    (sents, (vocab1, vocab2))
}

pub fn load_gold(file: &str, substract_one: bool) -> Vec<AlgnGold> {
    let reader = BufReader::new(File::open(&file).unwrap());
    let mut algns = Vec::<AlgnGold>::new();

    for line in reader.lines() {
        let mut sure = HashSet::<(usize, usize)>::new();
        let mut poss = HashSet::<(usize, usize)>::new();
        let line = line.unwrap();
        for token in line.split_whitespace() {
            if token.contains('-') {
                let indicies = token.split('-').collect::<Vec<&str>>();
                assert_eq!(indicies.len(), 2);
                let t1: usize = indicies[0].parse().unwrap();
                let t2: usize = indicies[1].parse().unwrap();
                if substract_one {
                    sure.insert((t1 - 1, t2 - 1));
                } else {
                    sure.insert((t1, t2));
                }
            } else if token.contains('?') {
                let indicies = token.split('?').collect::<Vec<&str>>();
                assert_eq!(indicies.len(), 2);
                let t1: usize = indicies[0].parse().unwrap();
                let t2: usize = indicies[1].parse().unwrap();
                if substract_one {
                    poss.insert((t1 - 1, t2 - 1));
                } else {
                    poss.insert((t1, t2));
                }
            }
        }
        algns.push((sure, poss));
    }

    algns
}

pub fn load_word_probs(file: String) -> (Vec<Vec<f32>>, (Vocab, Vocab)) {
    let reader = BufReader::new(File::open(&file).unwrap());
    let mut vocab1 = HashMap::<String, usize>::new();
    let mut vocab2 = HashMap::<String, usize>::new();
    let mut word_probs_raw = Vec::<(f32, usize, usize)>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let len1 = vocab1.len();
        let w1_i = vocab1.entry(tokens[2].to_string()).or_insert(len1);
        let len2 = vocab2.len();
        let w2_i = vocab2.entry(tokens[3].to_string()).or_insert(len2);
        let prob = tokens[1].parse::<f32>().unwrap();
        word_probs_raw.push((prob, *w1_i, *w2_i));
    }

    let mut word_probs = vec![vec![0.0; vocab1.len()]; vocab2.len()];
    for (prob, w1_i, w2_i) in word_probs_raw {
        word_probs[w2_i][w1_i] = prob;
    }

    (word_probs, (vocab1, vocab2))
}

pub fn load_data(opts: &OptsMain) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    if let (Some(file1), Some(file2)) = (&opts.file1, &opts.file2) {
        load_all(file1, file2)
    } else if let (Some(sent1), Some(sent2)) = (&opts.sent1, &opts.sent2) {
        load_sent(sent1, sent2)
    } else {
        panic!("Either two files or two sentences have to be provided")
    }
}
