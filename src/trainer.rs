mod utils;
mod align_hard;
mod align_soft;

use clap::Clap;
use utils::{cli::OptsServer, evaluator, reader, writer};

fn main() {
    let opts = OptsServer::parse();
    eprintln!("Training translation probabilities with IBM1. Saving with threshold {}", opts.threshold);
    let (sents, (vocab1, vocab2)) = reader::load_all(opts.file1, opts.file2);

    let word_probs = &align_soft::ibm1::ibm1_raw(&sents, &vocab1, &vocab2).1;
    writer::write_dict(opts.out, word_probs, &vocab1, &vocab2, opts.threshold)
}