use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    pub file1: String,
    pub file2: String,
    #[clap(short, long)]
    pub gold: Option<String>,
    #[clap(short, long, default_value = "ibm1")]
    pub soft: String,
    #[clap(short, long, default_value = "basic")]
    pub hard: String,
    #[clap(short, long)]
    pub extractor_params: Option<String>,
}