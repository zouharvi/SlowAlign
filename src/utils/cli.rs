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
#[clap(version = "0.1", author = "Vilém Zouhar <zouhar@ufal.mff.cuni.cz>")]
pub struct OptsMain {
    #[clap(short, long)]
    pub file1: Option<String>,
    #[clap(short, long)]
    pub file2: Option<String>,
    #[clap(short, long)]
    pub sent1: Option<String>,
    #[clap(short, long)]
    pub sent2: Option<String>,
    #[clap(short, long)]
    pub gold: Option<String>,
    #[clap(short, long)]
    pub dic: Option<String>,
    #[clap(short, long, default_value = "static")]
    pub method: String,
    #[clap(long)]
    pub gold_substract_one: bool,
    #[clap(
        short,
        long,
        default_value = "[0.0],[0.0],[1.0],[0.8],[0.0,0.1],[0.95],[0.8]"
    )]
    pub params: ArgExtractorParams,
    #[clap(long)]
    pub lowercase: bool,
}

/**
 * Description of the command line parameters for binary slow_align_dic.
 **/
#[derive(Clap)]
#[clap(version = "0.1", author = "Vilém Zouhar <zouhar@ufal.mff.cuni.cz>")]
pub struct OptsDic {
    pub file1: String,
    pub file2: String,
    pub out: String,
    #[clap(short, long, default_value = "0.3")]
    pub threshold: f32,
    #[clap(long)]
    pub lowercase: bool,
}
