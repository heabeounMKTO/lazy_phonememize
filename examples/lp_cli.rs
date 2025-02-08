use lazy_phonememize::phonememizer::LazyPhonemizer;
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(long)]
    input_text: String,
    
    #[arg(long)]
    lang: Option<String>
}

fn main(){
   let args = CliArgs::parse(); 

  let _sel_lang = match &args.lang {
        Some(_lang) => _lang,
        None => "en"
    };
   let pz = LazyPhonemizer::init(Some(_sel_lang)).unwrap();
    
let phonemized = pz.convert_to_phonemes(&args.input_text, lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();

let phonemized2 = pz.convert_to_phonemes(&args.input_text, lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();
   // let phonemized = convert_to_phonemes(&args.input_text, Some("en"), lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();
   println!("INPUT_TEXT: {}\nPHONEMIZED: {}", args.input_text, phonemized);
   println!("INPUT_TEXT: {}\nPHONEMIZED: {}", args.input_text, phonemized2);
}
