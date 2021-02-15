#![allow(dead_code)]

mod align_hard;
mod align_soft;
mod utils;

use clap::Clap;
use utils::{cli::OptsDic, evaluator, reader, writer};

/**
 * This binary is used for training a soft alignment model (IBM based) and store the word translation probabilities.
 **/
fn main() {
    let opts = OptsDic::parse();
    eprintln!(
        "Training translation probabilities with IBM1. Saving with threshold {}",
        opts.threshold
    );
    // @TODO: use load_data instead to allow for command line input
    let (sents, (vocab1, vocab2)) = reader::load_file(&opts.file1, &opts.file2, opts.lowercase);

    let word_probs = &align_soft::ibm1::ibm1_raw(&sents, &vocab1, &vocab2).1;
    writer::write_dict(opts.out, word_probs, &vocab1, &vocab2, opts.threshold)
}
