#include "phonememize.h"
#include <espeak-ng/speak_lib.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

int text2phoneme(const char *input_text, char *output_buffer, 
                 int buffer_size, const char *voice, const int phoneme_mode)
{
    if (!input_text || !output_buffer || buffer_size <= 0) {
        return -1;
    }
    if (espeak_Initialize(AUDIO_OUTPUT_SYNCHRONOUS, 0, NULL, 0) < 0) {
        return -2;
    }

    espeak_SetVoiceByName(voice);

    memset(output_buffer, 0, buffer_size);
    char* text_copy = strdup(input_text);
    char* current_pos = text_copy;
    int remaining_buffer = buffer_size - 1;  // Leave space for null terminator
    char* output_pos = output_buffer;

    while (*current_pos && remaining_buffer > 0) {
        while (*current_pos && (*current_pos == ' ' || *current_pos == '\t' || *current_pos == '\n')) {
            current_pos++;
        }
        if (!*current_pos) break;
        const char* phonemes = espeak_TextToPhonemes((const void**)&input_text, espeakCHARS_AUTO, phoneme_mode);
        if (phonemes) {
            int phonemes_len = strlen(phonemes);
            if (phonemes_len > 0) {
                // Don't add space before the first word
                if (output_pos != output_buffer && remaining_buffer > 1) {
                    *output_pos++ = ' ';
                    remaining_buffer--;
                }
                // Copy phonemes
                int to_copy = (phonemes_len < remaining_buffer) ? phonemes_len : remaining_buffer;
                strncpy(output_pos, phonemes, to_copy);
                output_pos += to_copy;
                remaining_buffer -= to_copy;
            }
        }
        
        while (*current_pos && !(*current_pos == ' ' || *current_pos == '\t' || *current_pos == '\n')) {
            current_pos++;
        }
    }
    
    free(text_copy);
    
    *output_pos = '\0';
    
    espeak_Terminate();
    
    return 0;
}
