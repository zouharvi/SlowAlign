use crate::utils::{linspace, noparam};
use clap::Clap;

mod aligner;
mod extractor;
mod optimizer;
mod utils;

use utils::{cli::Opts, evaluator, reader};

fn main() {
    let opts: Opts = Opts::parse();

    let (sents, (vocab1, vocab2)) = reader::load_all(opts.file1, opts.file2);

    let alignment_probs = match opts.soft.as_str() {
        "ibm1" => aligner::ibm1::ibm1(&sents, &vocab1, &vocab2),
        "levenstein" => aligner::levenstein::levenstein_align(&sents, &vocab1, &vocab2),
        _ => panic!("Unknown soft algorithm"),
    };

    let alignment = match opts.hard.as_str() {
        "argmax" => extractor::a1_argmax(&alignment_probs),
        "basic" => {
            let algn_a1 = extractor::a1_argmax(&alignment_probs);
            let algn_a2 = extractor::a2_threshold(&alignment_probs, 0.2);
            optimizer::intersect_algn(Some(algn_a1), algn_a2).unwrap()
        },
        "search" => {
            let algn_gold = if let Some(file) = opts.gold {
                reader::load_gold(file, 100, true)
            } else {
                panic!("Gold alignments not supplied (only top N are required)")
            };

            let (algn, _params, _aer) = optimizer::gridsearch(
                &[noparam(), linspace(0.0, 1.0, 20)],
                vec![
                    &|_p: f32| extractor::a1_argmax(&alignment_probs),
                    &|p: f32| extractor::a2_threshold(&alignment_probs, p),
                ],
                &algn_gold,
            );
            algn
        }
        _ => panic!("Unknown hard algorithm"),
    };

    // print alignments
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
