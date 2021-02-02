use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const MAX_SENT_COUNT: usize = 20000;
pub fn init() -> (
    Vec<(Vec<usize>, Vec<usize>)>,
    (HashMap<String, usize>, HashMap<String, usize>),
) {
    // load data
    let file1 = File::open(Path::new("data/data_ende.en")).unwrap();
    let file2 = File::open(Path::new("data/data_ende.de")).unwrap();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    // TODO: undocumented heuristic allocation
    let mut sents = Vec::<(Vec<usize>, Vec<usize>)>::with_capacity(MAX_SENT_COUNT);
    let mut vocab1 = HashMap::<String, usize>::with_capacity(MAX_SENT_COUNT / 2);
    let mut vocab2 = HashMap::<String, usize>::with_capacity(MAX_SENT_COUNT / 2);

    for (line1, line2) in reader1.lines().zip(reader2.lines()).take(MAX_SENT_COUNT) {
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
