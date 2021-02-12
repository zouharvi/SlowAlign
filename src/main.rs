use crate::optimizer::AlignmentPackage;
use crate::evaluator::alignment_error_rate;
use crate::reader::Sent;
use crate::utils::{linspace, pack};
use clap::Clap;

mod align_hard;
mod align_soft;
mod optimizer;
mod utils;
const GOLD_DEV_COUNT: usize = 20;

use utils::{cli::Opts, evaluator, reader};

fn main() {
    let opts: Opts = Opts::parse();

    let (sents, (vocab1, vocab2)) = reader::load_all(opts.file1, opts.file2);

    let alignment = match opts.method.as_str() {
        "static" => align_hard::a1_argmax(&align_soft::merge_sum(
            &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            &align_soft::misc::diagonal(&sents),
            0.4,
        )),
        "levenstein" => align_hard::a2_threshold(
            &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            0.75,
        ),
        "ibm1" => align_hard::a1_argmax(&align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2)),
        "search" => {
            let alignment_dev = &reader::load_gold(
                &opts
                    .gold
                    .clone()
                    .expect("Gold alignment need to be provided for gridsearch"),
                opts.gold_substract_one,
            )[..GOLD_DEV_COUNT];

            let sents_rev = sents
                .iter()
                .map(|(x, y)| (y.clone(), x.clone()))
                .collect::<Vec<(Sent, Sent)>>();

            let package = AlignmentPackage {
                alignment_fwd: &align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2)[..GOLD_DEV_COUNT],
                alignment_rev: &align_soft::ibm1::ibm1(&sents_rev, &vocab2, &vocab1)[..GOLD_DEV_COUNT],
                alignment_diag: &align_soft::misc::diagonal(&sents[..GOLD_DEV_COUNT]),
                alignment_lev: &align_soft::misc::levenstein(&sents[..GOLD_DEV_COUNT], &vocab1, &vocab2),
            };
            let (algn, params, _aer) = optimizer::gridsearch(
                &package,
                &optimizer::extractor_recipes(),
                &alignment_dev,
            );
            algn
        }
        _ => panic!("Unknown hard algorithm"),
    };

    if let Some(file) = opts.gold {
        let alignment_eval = reader::load_gold(&file, opts.gold_substract_one);
        eprintln!("{}, {}", alignment.len(), alignment_eval.len());
        let aer = alignment_error_rate(&alignment, &alignment_eval);
        eprintln!("AER {}\n", aer);
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
