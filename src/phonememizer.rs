use std::ffi::{CString, CStr};
use anyhow::{Result,  anyhow};
use std::mem::size_of;

#[link(name="phonememize", kind="static")]
extern "C" {
    fn text2phoneme(input: *const libc::c_char,
                    output_buffer: *mut libc::c_char,
                    buffer_size: libc::c_int,
                    voice: *const libc::c_char,
                    phoneme_mode: libc::c_int) -> libc::c_int;
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
    ASCII
}


pub fn convert_to_phonemes(text: &str, lang: Option<&str>, output_type: PhonemeOutputType) -> Result<String> {
    let input_lang = match lang {
        Some(_lang) => _lang,
        None => "en"
    };
    let output_type_int: i32 = match output_type {
        PhonemeOutputType::ESpeak => 0,
        PhonemeOutputType::ASCII => 2
    };
    let c_text = CString::new(text).map_err(|_| "wtf invalid text input").unwrap();
    let input_ptr = CString::new(input_lang).map_err(|_| "wtf invalid lang input").unwrap();
    // 
    /* wtf is a 0u8? 
     https://stackoverflow.com/questions/53120755/what-do-number-literals-with-a-suffix-like-0u8-mean-in-rust 
    */
    // println!("[DEBUG] text len {}", text.chars().count());
    let mut buffer = vec![0u8; 4096]; // big ass buffer for no reason
    // println!("[DEBUG] `lazy_p buffer len {:?}", &buffer.len());
    unsafe {
        let result = text2phoneme(c_text.as_ptr(), 
                                  buffer.as_mut_ptr() as *mut libc::c_char, 
                                  buffer.len() as i32, 
                                  input_ptr.as_ptr(),
                                  output_type_int);
        if result < 0 {
           return Err(anyhow!("conversion failed with error code {}", result));
        }
        let result_str = CStr::from_ptr(buffer.as_ptr() as *const libc::c_char);
        Ok(result_str.to_string_lossy().into_owned())
    }
}


