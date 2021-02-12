use crate::evaluator::AlgnGold;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Vocab = HashMap<String, usize>;
pub type VocabRev = HashMap<usize, String>;
pub type Sent = Vec<usize>;

pub fn load(
    file1: String,
    file2: String,
    start: usize,
    count: usize,
) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    // load data
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

pub fn load_all(file1: String, file2: String) -> (Vec<(Sent, Sent)>, (Vocab, Vocab)) {
    load(file1, file2, 0, usize::MAX)
}

pub fn load_gold(file: &str, substract_one: bool) -> Vec<AlgnGold> {
    // load data
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
