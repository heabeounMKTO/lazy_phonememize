use std::ffi::{CString, CStr};
use anyhow::{Result,  anyhow};

#[link(name="phonememize", kind="static")]
extern "C" {
    fn text2phoneme(input: *const libc::c_char,
                    output_buffer: *mut libc::c_char,
                    buffer_size: libc::c_int,
                    voice: *const libc::c_char) -> libc::c_int;
}

pub fn convert_to_phonemes(text: &str, lang: Option<&str>) -> Result<String> {
    let input_lang = match lang {
        Some(_lang) => _lang,
        None => "en"
    };
    let c_text = CString::new(text).map_err(|_| "wtf invalid text input").unwrap();
    let input_ptr = CString::new(input_lang).map_err(|_| "wtf invalid lang input").unwrap();
    // 
    /* wtf is a 0u8? 
     https://stackoverflow.com/questions/53120755/what-do-number-literals-with-a-suffix-like-0u8-mean-in-rust 
    */
    let mut buffer = vec![0u8; 8096]; // big ass buffer for no reason
    unsafe {
        let result = text2phoneme(c_text.as_ptr(), 
                                  buffer.as_mut_ptr() as *mut libc::c_char, 
                                  buffer.len() as i32, 
                                  input_ptr.as_ptr());
        if result < 0 {
           return Err(anyhow!("conversion failed with error code {}", result));
        }
        let result_str = CStr::from_ptr(buffer.as_ptr() as *const libc::c_char);
        Ok(result_str.to_string_lossy().into_owned())
    }
}


