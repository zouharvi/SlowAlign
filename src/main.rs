use crate::utils::linspace;
use clap::Clap;

mod align_hard;
mod align_soft;
mod optimizer;
mod utils;

use utils::{cli::Opts, evaluator, reader};

fn main() {
    let opts: Opts = Opts::parse();

    let (sents, (vocab1, vocab2)) = reader::load_all(opts.file1, opts.file2);

    let alignment_probs = match opts.soft.as_str() {
        "ibm1" => align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2),
        "levenstein" => align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
        _ => panic!("Unknown soft algorithm"),
    };

    let alignment = match opts.hard.as_str() {
        "argmax" => align_hard::a1_argmax(&alignment_probs),
        "basic" => {
            let algn_a1 = align_hard::a1_argmax(&alignment_probs);
            let algn_a2 = align_hard::a2_threshold(&alignment_probs, 0.2);
            optimizer::intersect_algn(Some(algn_a1), algn_a2).unwrap()
        }
        "search" => {
            let algn_gold = if let Some(file) = opts.gold {
                reader::load_gold(file, 100, true)
            } else {
                panic!("Gold alignments not supplied (only top N are required)")
            };

            let alignment_probs_diagonal = align_soft::misc::diagonal(&sents);

            let (algn, _params, _aer) = optimizer::gridsearch(
                &[
                    linspace(0.0, 0.2, 4),
                    linspace(0.8, 1.0, 4),
                    linspace(0.6, 1.0, 4),
                    linspace(0.0, 0.2, 4),
                ],
                vec![
                    &|p: &f32| align_hard::a2_threshold(&alignment_probs, *p),
                    &|p: &f32| align_hard::a3_threshold_dynamic(&alignment_probs, *p),
                    &|p: &f32| align_hard::a2_threshold(&alignment_probs_diagonal, *p),
                    &|p: &f32| {
                        align_hard::a2_threshold(&align_soft::misc::blur(&alignment_probs, *p), 0.1)
                    },
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
