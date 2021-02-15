use clap::Clap;
use std::num::ParseFloatError;
use std::str::FromStr;

/**
 * Holder for extractor parameters. Can be parsed from a string.
 **/
pub struct ArgExtractorParams {
    pub data: Vec<Vec<f32>>,
}

impl FromStr for ArgExtractorParams {
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let raw = raw.split_whitespace().collect::<String>().replace("]", "");
        let param_parents = raw
            .split('[')
            .map(|line| line.split(',').collect::<Vec<&str>>());
        let mut data = vec![];
        for tokens in param_parents.skip(1) {
            data.push(
                tokens
                    .iter()
                    .filter(|tok| !tok.is_empty())
                    .map(|tok| tok.parse().expect("Incorrect parameter numeric value"))
                    .collect::<Vec<f32>>(),
            );
        }
        Ok(ArgExtractorParams { data })
    }
    type Err = ParseFloatError;
}

/**
 * Description of the command line parameters for binary slow_align.
 **/
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Vilém Zouhar <zouhar@ufal.mff.cuni.cz>")]
pub struct OptsMain {
    #[clap(
        short,
        long,
        about = "Path to the source file to align. (If both files and sentences are provided, only files are used)."
    )]
    pub file1: Option<String>,
    #[clap(
        short,
        long,
        about = "Path to the target file to align. (If both files and sentences are provided, only files are used)."
    )]
    pub file2: Option<String>,
    #[clap(
        short,
        long,
        about = "List of source sentences (separated by \\n) to align."
    )]
    pub sent1: Option<String>,
    #[clap(
        short,
        long,
        about = "List of target sentences (separated by \\n) to align."
    )]
    pub sent2: Option<String>,
    #[clap(
        short,
        long,
        about = "Path to the file with alignments (single space separated, x-y for sure alignments, x?y for possible). `x` and `y` are (by default) 0-indexed token indicies."
    )]
    pub gold: Option<String>,
    #[clap(
        short,
        long,
        about = "OPUS-like dictionary of word translation probabilities"
    )]
    pub dic: Option<String>,
    #[clap(
        short,
        long,
        default_value = "static",
        about = "Which alignment method pipeline to use (static, dic, levenstein, ibm1, search)"
    )]
    pub method: String,
    #[clap(
        long,
        about = "Treat gold alignments as if they are 1-indexed (default is 0-indexed)"
    )]
    pub gold_index_one: bool,
    #[clap(
        short,
        long,
        default_value = "[0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]",
        about = "Comma-separated arrays of parameters to the estimator recipe"
    )]
    pub params: ArgExtractorParams,
    #[clap(
        long,
        about = "Treat everything case-insensitive (default is case-sensitive, even though that provides slightly worse performance)."
    )]
    pub lowercase: bool,
    #[clap(
        long,
        about = "Number of sentences (from the top) to use for parameter estimation.",
        default_value = "0",
    )]
    pub dev_count: usize,
    #[clap(
        long,
        about = "Offset from which to evaluate data. If not supplied, use --dev-count value (so that dev and test do not overlap).",
    )]
    pub test_offset: Option<usize>,
}

/**
 * Description of the command line parameters for binary slow_align_dic.
 **/
#[derive(Clap)]
#[clap(version = "0.1", author = "Vilém Zouhar <zouhar@ufal.mff.cuni.cz>")]
pub struct OptsDic {
    #[clap(about = "Path to the source file to train word translation probabilities on.")]
    pub file1: String,
    #[clap(about = "Path to the target file to train word translation probabilities on.")]
    pub file2: String,
    #[clap(about = "Path to the output word translation probabilities dictionary.")]
    pub out: String,
    #[clap(
        short,
        long,
        default_value = "0.2",
        about = "Threshold under which translation probabilities will be omitted. Lower values lead to better approximation, but also larger file size."
    )]
    pub threshold: f32,
    #[clap(
        long,
        about = "Treat everything case-insensitive (default is case-sensitive, even though that provides slightly worse performance)."
    )]
    pub lowercase: bool,
}
