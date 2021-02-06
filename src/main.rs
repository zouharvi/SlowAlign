use crate::evaluator::alignment_error_rate;
use crate::reader::Sent;
use crate::utils::cartesian_product;
use crate::utils::{linspace, noparam, pack};
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
        "levenstein" => {
            align_hard::a2_threshold(&align_soft::misc::levenstein(&sents, &vocab1, &vocab2), 0.75)
        }
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
            let alignment_probs_rev =
                &align_soft::ibm1::ibm1(&sents_rev, &vocab2, &vocab1)[..GOLD_DEV_COUNT];
            let alignment_probs_diagonal = align_soft::misc::diagonal(&sents[..GOLD_DEV_COUNT]);
            let alignment_probs_levenstein =
                align_soft::misc::levenstein(&sents[..GOLD_DEV_COUNT], &vocab1, &vocab2);
            let alignment_probs =
                &align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2)[..GOLD_DEV_COUNT];

            let (algn, _params, _aer) = optimizer::gridsearch(
                &[
                    //(pack(&noparam()), optimizer::AlgnMergeAction::INTERSECT, &|p: &[f32]| align_hard::a1_argmax(&alignment_probs),
                    (
                        pack(&linspace(0.0, 0.05, 2)),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| align_hard::a2_threshold(&alignment_probs, p[0]),
                    ),
                    (
                        pack(&linspace(0.0, 0.0, 1)),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| align_hard::a3_threshold_dynamic(&alignment_probs, p[0]),
                    ),
                    (
                        pack(&linspace(0.95, 1.0, 4)),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| align_hard::a4_threshold_dynamic(&alignment_probs, p[0]),
                    ),
                    (
                        pack(&linspace(0.4, 0.8, 5)),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| {
                            align_hard::a4_threshold_dynamic(&alignment_probs_diagonal, p[0])
                        },
                    ),
                    (
                        cartesian_product(vec![linspace(0.0, 0.2, 3), linspace(0.1, 0.3, 8)]),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| {
                            align_hard::a2_threshold(
                                &align_soft::misc::blur(&alignment_probs, p[0]),
                                p[1],
                            )
                        },
                    ),
                    (
                        pack(&linspace(0.95, 1.0, 4)),
                        optimizer::AlgnMergeAction::INTERSECT,
                        &|p: &[f32]| {
                            evaluator::alignment_reverse(&align_hard::a4_threshold_dynamic(
                                &alignment_probs_rev,
                                p[0],
                            ))
                        },
                    ),
                    (
                        pack(&linspace(0.7, 1.0, 4)),
                        optimizer::AlgnMergeAction::JOIN,
                        &|p: &[f32]| align_hard::a2_threshold(&alignment_probs_levenstein, p[0]),
                    ),
                ],
                &alignment_dev,
            );
            algn
        }
        _ => panic!("Unknown hard algorithm"),
    };

    if let Some(file) = opts.gold {
        let alignment_eval = reader::load_gold(&file, opts.gold_substract_one);
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
