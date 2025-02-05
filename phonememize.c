#include "phonememize.h"
#include <espeak-ng/speak_lib.h>
#include <string.h>


int text2phoneme(const char *input_text, char *output_buffer, int buffer_size, const char *voice)
{
    if (!input_text || !output_buffer || buffer_size <= 0) {
        return -1;
    }
    if (espeak_Initialize(AUDIO_OUTPUT_SYNCHRONOUS, 0, NULL, 0) < 0) {
        return -2;
    }

    espeak_SetVoiceByName(voice);
    const char* phonemes = espeak_TextToPhonemes((const void**)&input_text, 0, 0);
    
    if (!phonemes) {
        espeak_Terminate();
        return -3;
    }
  strncpy(output_buffer, phonemes, buffer_size - 1);
  output_buffer[buffer_size - 1] = '\0'; // terminate da string with \0, 
  espeak_Terminate();  
  return 0;

}
