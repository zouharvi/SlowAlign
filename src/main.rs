mod extractor;
mod ibm1;
mod reader;

fn main() {    
    let (sents, (vocab1, vocab2)) = reader::init();

    let alignment_probs = ibm1::ibm1(&sents, &vocab1, &vocab2);
    let alignment = extractor::a1_argmax(&alignment_probs);

    for sent_align in alignment {
        println!(
            "{}",
            sent_align
                .iter()
                .map(|(pos1, pos2)| format!("{}-{}", pos1, pos2))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}