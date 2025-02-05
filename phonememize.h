#ifndef PHONEMEIZE_H
#define PHONEMEIZE_H


#ifdef __cplusplus
extern "C" {
#endif

int text2phoneme(const char* input_text, 
                 char* output_buffer, 
                 int buffer_size, 
                 const char *voice, 
                 const int phoneme_mode);

#ifdef __cplusplus
}
#endif


#endif //PHONEMEIZE_H
