use clap::Parser;
use lazy_phonememize::phonememizer::LazyPhonemizer;

#[derive(Parser)]
struct CliArgs {
    #[arg(long)]
    input_text: String,

    #[arg(long)]
    lang: Option<String>,
}

fn main() {
    let args = CliArgs::parse();

    let _sel_lang = match &args.lang {
        Some(_lang) => _lang,
        None => "en",
    };
    let pz = LazyPhonemizer::init(Some(_sel_lang)).unwrap();

    let phonemized = pz
        .convert_to_phonemes(
            &args.input_text,
            lazy_phonememize::phonememizer::PhonemeOutputType::ASCII,
        )
        .unwrap();
    println!(
        "INPUT_TEXT: {}\nPHONEMIZED: {}",
        args.input_text, phonemized
    );
}
