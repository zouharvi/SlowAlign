use clap::Clap;

#[derive(Clap)]
pub struct OptsMain {
    pub file1: String,
    pub file2: String,
    #[clap(short, long)]
    pub gold: Option<String>,
    #[clap(short, long, default_value = "static")]
    pub method: String,
    #[clap(long)]
    pub gold_substract_one: bool,
    #[clap(short, long)]
    pub extractor_params: Option<String>,
}

#[derive(Clap)]
pub struct OptsServer {
    pub file1: String,
    pub file2: String,
    pub out: String,
    #[clap(short, long, default_value = "0.3")]
    pub threshold: f32,
}