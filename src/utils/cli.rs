use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    pub file1: String,
    pub file2: String,
    #[clap(short, long)]
    pub gold: Option<String>,
    #[clap(short, long, default_value = "static")]
    pub method: String,
    #[clap(short, long)]
    pub gold_substract_one: bool,
    #[clap(short, long)]
    pub extractor_params: Option<String>,
}