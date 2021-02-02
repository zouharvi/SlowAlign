use crate::utils::{linspace,noparam};

mod evaluator;
mod extractor;
mod ibm1;
mod optimizer;
mod reader;
mod utils;

fn main() {
    let (sents, (vocab1, vocab2)) = reader::load_all();
    let algn_gold = reader::load_gold(100, true);
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

    optimizer::gridsearch(
        &[
            noparam(),
            linspace(0.0, 1.0, 20)
        ],
        vec![
            &|_p: f32| extractor::a1_argmax(&alignment_probs),
            &|p: f32| extractor::a2_threshold(&alignment_probs, p),
        ],
        &algn_gold,
    );
}
