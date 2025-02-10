use anyhow::{anyhow, Result};
use std::ffi::{CStr, CString};
use std::sync::Once;
use regex::Regex;

static INIT_G2P_LANG: Once = Once::new();

#[link(name = "phonememize", kind = "static")]
extern "C" {
    fn g2p_initialize(voice: *const libc::c_char) -> libc::c_int;
    fn g2p_terminate();
    fn text2phoneme(
        input: *const libc::c_char,
        output_buffer: *mut libc::c_char,
        buffer_size: libc::c_int,
        phoneme_mode: libc::c_int,
    ) -> libc::c_int;
}

/// FROM ESPEAK
///
/**
* phoneme_mode
        bit 1:   0=eSpeak's ascii phoneme names, 1= International Phonetic Alphabet (as UTF-8 characters).
        bit 7:   use (bits 8-23) as a tie within multi-letter phonemes names
        bits 8-23:  separator character, between phoneme names
**/
pub enum PhonemeOutputType {
    ESpeak,
    ASCII,
}

pub struct LazyPhonemizer {
    pub lang: String,
    pub init_state: bool,
}


pub fn clean_phonemes(phonemes: &str) -> String {
    let number_pattern = Regex::new(r"-\d+|\d+|-").unwrap();
    let cleaned = number_pattern.replace_all(phonemes, "");
    let diacritic_pattern = Regex::new(r"̪").unwrap();
    diacritic_pattern.replace_all(&cleaned, "").into_owned()
}

impl LazyPhonemizer {
    pub fn init(lang: Option<&str>) -> Result<LazyPhonemizer> {
        let _init_lang = match lang {
            Some(_lang) => _lang,
            None => "en",
        };

        let mut init_ok = Ok(());
        let _lang_ptr = CString::new(_init_lang)
            .map_err(|_| "wtf invalid lang input")
            .unwrap();
        INIT_G2P_LANG.call_once(|| unsafe {
            if g2p_initialize(_lang_ptr.as_ptr()) < 0 {
                init_ok = Err("Failed to init g2p".to_string());
            }
        });
        Ok(LazyPhonemizer {
            lang: String::from(_init_lang),
            init_state: match init_ok {
                Ok(_) => true,
                _ => false,
            },
        })
    }


    pub fn convert_to_phonemes(
        &self,
        text: &str,
        output_type: PhonemeOutputType,
    ) -> Result<String> {
        let output_type_int: i32 = match output_type {
            PhonemeOutputType::ESpeak => 0,
            PhonemeOutputType::ASCII => 2,
        };
        let c_text = CString::new(text)
            .map_err(|_| "wtf invalid text input")
            .unwrap();
        //
        /* wtf is a 0u8?
         https://stackoverflow.com/questions/53120755/what-do-number-literals-with-a-suffix-like-0u8-mean-in-rust
        */
        // println!("[DEBUG] text len {}", text.chars().count());
        let mut buffer = vec![0u8; text.chars().count() * 8]; // big ass buffer for no reason
        println!(
            "[DEBUG] [LazyPhonemizer] `lazy_p buffer len {:?}",
            &buffer.len()
        );
        unsafe {
            let result = text2phoneme(
                c_text.as_ptr(),
                buffer.as_mut_ptr() as *mut libc::c_char,
                buffer.len() as i32,
                output_type_int,
            );
            // println!("[DEBUG] conversion REUSLT {}", result);
            if result < 0 {
                return Err(anyhow!("conversion failed with error code {}", result));
            }
            let result_str = CStr::from_ptr(buffer.as_ptr() as *const libc::c_char);
            Ok(result_str.to_string_lossy().into_owned())
        }
    }
}
impl Drop for LazyPhonemizer {
    fn drop(&mut self) {
        unsafe {
            g2p_terminate();
        }
    }
}
