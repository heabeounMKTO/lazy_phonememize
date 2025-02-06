use lazy_phonememize::phonememizer::LazyPhonemizer;
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(long)]
    input_text: String
}

fn main(){
   let args = CliArgs::parse(); 
   let pz = LazyPhonemizer::init(Some("en")).unwrap();
    
let phonemized = pz.convert_to_phonemes(&args.input_text, lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();

let phonemized2 = pz.convert_to_phonemes(&args.input_text, lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();
   // let phonemized = convert_to_phonemes(&args.input_text, Some("en"), lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();
   println!("INPUT_TEXT: {}\nPHONEMIZED: {}", args.input_text, phonemized);
   println!("INPUT_TEXT: {}\nPHONEMIZED: {}", args.input_text, phonemized2);
}
