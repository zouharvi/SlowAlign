use crate::evaluator::alignment_error_rate;
use crate::optimizer::AlignmentPackage;
use crate::reader::Sent;
use crate::utils::{linspace, pack};
use clap::Clap;

mod align_hard;
mod align_soft;
mod optimizer;
mod utils;
const GOLD_DEV_COUNT: usize = 20;

use utils::{cli::OptsMain, evaluator, reader};

fn main() {
    let opts = OptsMain::parse();

    let (sents, (vocab1, vocab2)) = reader::load_data(&opts);

    let alignment = match opts.method.as_str() {
        "dic" => {
            let (dic, (dic_vocab1, dic_vocab2)) = reader::load_word_probs(
                opts.dic
                    .expect("Path to word translation probability file has to be provided."),
            );
            let package = AlignmentPackage {
                alignment_fwd: &align_soft::misc::from_dic(
                    &sents,
                    &vocab1,
                    &vocab2,
                    &dic,
                    &dic_vocab1,
                    &dic_vocab2,
                ),
                alignment_rev: &align_soft::misc::from_dic_rev(
                    &sents,
                    &vocab1,
                    &vocab2,
                    &dic,
                    &dic_vocab1,
                    &dic_vocab2,
                ),
                alignment_diag: &align_soft::misc::diagonal(&sents),
                alignment_lev: &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            };
            optimizer::params_to_alignment(
                &[
                    vec![0.0],
                    vec![0.0],
                    vec![0.1],
                    vec![0.6],
                    vec![0.0, 0.2],
                    vec![0.1],
                    vec![0.9],
                ],
                &package,
                &optimizer::extractor_recipes,
            )
        }
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
                    .expect("Gold alignment needs to be provided for gridsearch"),
                opts.gold_substract_one,
            )[..GOLD_DEV_COUNT];

            let sents_rev = sents
                .iter()
                .map(|(x, y)| (y.clone(), x.clone()))
                .collect::<Vec<(Sent, Sent)>>();

            let package = AlignmentPackage {
                alignment_fwd: &align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2)[..GOLD_DEV_COUNT],
                alignment_rev: &align_soft::ibm1::ibm1(&sents_rev, &vocab2, &vocab1)
                    [..GOLD_DEV_COUNT],
                alignment_diag: &align_soft::misc::diagonal(&sents[..GOLD_DEV_COUNT]),
                alignment_lev: &align_soft::misc::levenstein(
                    &sents[..GOLD_DEV_COUNT],
                    &vocab1,
                    &vocab2,
                ),
            };
            let (algn, params, _aer) = optimizer::gridsearch(
                &package,
                &optimizer::extractor_recipes_params(),
                &optimizer::extractor_recipes,
                &alignment_dev,
            );
            algn
        }
        _ => panic!("Unknown method"),
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
