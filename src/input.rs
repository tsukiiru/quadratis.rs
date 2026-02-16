use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Input {
    /// path to the sample image (with extension)
    #[arg(short, long, default_value_t = String::from("./sample.png"))]
    pub sample: String,

    /// path to the folder with images
    #[arg(short, long, default_value_t = String::from("./images"))]
    pub images: String,
}
