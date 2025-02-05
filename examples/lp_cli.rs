use lazy_phonememize::phonememizer::convert_to_phonemes;
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[arg(long)]
    input_text: String
}

fn main(){
   let args = CliArgs::parse(); 
   let phonemized = convert_to_phonemes(&args.input_text, Some("en"), lazy_phonememize::phonememizer::PhonemeOutputType::ASCII).unwrap();
   println!("INPUT_TEXT: {}\nPHONEMIZED: {}", args.input_text, phonemized);
}
