#![allow(dead_code)]

use crate::evaluator::alignment_error_rate;
use crate::optimizer::AlignmentPackage;
use crate::reader::Sent;
use crate::utils::cli::ArgExtractorParams;
use crate::utils::{linspace, pack};
use clap::Clap;

mod align_hard;
mod align_soft;
mod optimizer;
mod utils;

use utils::{cli::OptsMain, evaluator, reader};

/**
 * This binary is used for alignment of new data and exploring the parameter space.
 **/
fn main() {
    let opts = OptsMain::parse();
    let (sents, (vocab1, vocab2)) = reader::load_data(&opts, opts.lowercase);

    // Compute the overall alignment based on the supplied method
    let alignment = match opts.method.as_str() {
        "dic" => {
            let (dic, (dic_vocab1, dic_vocab2)) = reader::load_word_probs(
                opts.dic
                    .expect("Path to word translation probability file has to be provided."),
                opts.lowercase,
                opts.switch_dic,
            );

            let package = AlignmentPackage {
                alignment_fwd: &align_soft::misc::from_dic(
                    &sents,
                    &vocab1,
                    &vocab2,
                    &dic,
                    &dic_vocab1,
                    &dic_vocab2,
                    false,
                ),
                alignment_rev: &align_soft::misc::from_dic(
                    &sents,
                    &vocab2,
                    &vocab1,
                    &dic,
                    &dic_vocab2,
                    &dic_vocab1,
                    true,
                ),
                alignment_diag: &align_soft::misc::diagonal(&sents),
                alignment_lev: &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            };
            optimizer::params_to_alignment(
                &opts
                    .params
                    .unwrap_or(ArgExtractorParams {
                        // Default
                        data: vec![
                            vec![0.0],
                            vec![0.0],
                            vec![1.0],
                            vec![0.8],
                            vec![0.0, 0.1],
                            vec![0.95],
                            vec![0.8],
                        ],
                    })
                    .data,
                &package,
                &optimizer::EXTRACTOR_RECIPES,
            )
        }
        "static" => align_hard::a1_argmax(&align_soft::misc::merge_sum(
            &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            &align_soft::misc::diagonal(&sents),
            opts.params
                .unwrap_or(ArgExtractorParams {
                    // Default
                    data: vec![vec![0.5]],
                })
                .data[0][0],
        )),
        "levenstein" => align_hard::a2_threshold(
            &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            opts.params
                .unwrap_or(ArgExtractorParams {
                    // Default
                    data: vec![vec![0.75]],
                })
                .data[0][0],
        ),
        "ibm1" => align_hard::a1_argmax(&align_soft::ibm1::ibm1(
            &sents,
            &vocab1,
            &vocab2,
            opts.ibm_steps,
        )),
        "search" => {
            let alignment_gold = &reader::load_gold(
                &opts
                    .gold
                    .clone()
                    .expect("Gold alignment needs to be provided for gridsearch"),
                opts.gold_index_one,
            );

            let dev_count = opts.dev_count;
            if dev_count == 0 {
                panic!("Can't do parameter estimation on zero sentences. Make sure to use --dev-count VALUE.")
            }

            let sents_rev = sents
                .iter()
                .map(|(x, y)| (y.clone(), x.clone()))
                .collect::<Vec<(Sent, Sent)>>();

            let package = AlignmentPackage {
                alignment_fwd: &align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2, opts.ibm_steps),
                alignment_rev: &align_soft::ibm1::ibm1(
                    &sents_rev,
                    &vocab2,
                    &vocab1,
                    opts.ibm_steps,
                ),
                alignment_diag: &align_soft::misc::diagonal(&sents),
                alignment_lev: &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            };
            let package_dev = AlignmentPackage {
                alignment_fwd: &package.alignment_fwd[..dev_count],
                alignment_rev: &package.alignment_rev[..dev_count],
                alignment_diag: &package.alignment_diag[..dev_count],
                alignment_lev: &package.alignment_lev[..dev_count],
            };
            let (_algn, params, _aer) = optimizer::gridsearch(
                &package_dev,
                &optimizer::extractor_recipes_params(),
                &optimizer::EXTRACTOR_RECIPES,
                &alignment_gold[..dev_count],
            );
            optimizer::params_to_alignment(&params, &package, &optimizer::EXTRACTOR_RECIPES)
        }
        "a5_fixed" => {
            let sents_rev = sents
                .iter()
                .map(|(x, y)| (y.clone(), x.clone()))
                .collect::<Vec<(Sent, Sent)>>();

            let package = AlignmentPackage {
                alignment_fwd: &align_soft::ibm1::ibm1(&sents, &vocab1, &vocab2, opts.ibm_steps),
                alignment_rev: &align_soft::ibm1::ibm1(
                    &sents_rev,
                    &vocab2,
                    &vocab1,
                    opts.ibm_steps,
                ),
                alignment_diag: &align_soft::misc::diagonal(&sents),
                alignment_lev: &align_soft::misc::levenstein(&sents, &vocab1, &vocab2),
            };
            optimizer::params_to_alignment(
                &opts.params.unwrap().data,
                &package,
                &optimizer::EXTRACTOR_RECIPES,
            )
        }
        _ => panic!("Unknown method"),
    };

    // Print AER if gold alignments were supplied
    if let Some(file) = opts.gold {
        let offset = opts.test_offset.unwrap_or(opts.dev_count);
        let alignment_gold = reader::load_gold(&file, opts.gold_index_one);
        let aer = alignment_error_rate(&alignment[offset..], &alignment_gold[offset..]);
        eprintln!("AER: {}\n", aer);
    };

    // Finally output the alignments
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
