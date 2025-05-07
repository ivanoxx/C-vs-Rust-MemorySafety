#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define MAX_BUFFER_SIZE 1024

// Structure to hold text analysis results
typedef struct {
    int word_count;
    int sentence_count;
    int longest_word_length;
    char* longest_word;  // Will point to a stack-allocated buffer (dangerous!)
} TextStats;

// Process text and return statistics with a dangling pointer
TextStats* analyze_text(const char* input_text) {
    // Stack-allocated buffer for word processing
    char word_buffer[MAX_BUFFER_SIZE];
    memset(word_buffer, 0, MAX_BUFFER_SIZE);
    
    // Stack-allocated result structure
    static TextStats result;
    result.word_count = 0;
    result.sentence_count = 0;
    result.longest_word_length = 0;
    result.longest_word = NULL;
    
    int in_word = 0;
    int current_word_length = 0;
    int word_buffer_pos = 0;
    
    // Process the input text
    for (int i = 0; input_text[i] != '\0'; i++) {
        char c = input_text[i];
        
        // Check for word boundaries
        if (c == ' ' || c == '\t' || c == '\n' || c == '\r') {
            if (in_word) {
                // End of a word
                in_word = 0;
                word_buffer[word_buffer_pos] = '\0';
                
                result.word_count++;
                
                // Check if this is the longest word so far
                if (current_word_length > result.longest_word_length) {
                    result.longest_word_length = current_word_length;
                    
                    // DANGEROUS: Pointing to stack memory that will be deallocated
                    result.longest_word = word_buffer;
                }
                
                // Reset for next word
                word_buffer_pos = 0;
                current_word_length = 0;
            }
        } else {
            // Process character as part of a word
            if (!in_word) {
                in_word = 1;
            }
            
            // Check for end of sentences
            if (c == '.' || c == '!' || c == '?') {
                result.sentence_count++;
            }
            
            // Add character to current word
            if (word_buffer_pos < MAX_BUFFER_SIZE - 1) {
                word_buffer[word_buffer_pos++] = c;
                current_word_length++;
            }
        }
    }
    
    // Handle case where text ends with a word
    if (in_word) {
        word_buffer[word_buffer_pos] = '\0';
        result.word_count++;
        
        if (current_word_length > result.longest_word_length) {
            result.longest_word_length = current_word_length;
            
            // DANGEROUS: Pointing to stack memory that will be deallocated
            result.longest_word = word_buffer;
        }
    }
    
    // If no sentences ended with punctuation, count at least one sentence
    if (result.sentence_count == 0 && result.word_count > 0) {
        result.sentence_count = 1;
    }
    
    // Return pointer to the static result structure
    // This is safe, but result.longest_word points to stack memory!
    return &result;
}

// Function to display text statistics
void display_text_stats(const TextStats* stats) {
    printf("Word count: %d\n", stats->word_count);
    printf("Sentence count: %d\n", stats->sentence_count);
    printf("Longest word length: %d\n", stats->longest_word_length);
    
    // DANGEROUS: Dereferencing a dangling pointer!
    if (stats->longest_word != NULL) {
        printf("Longest word: %s\n", stats->longest_word);
    } else {
        printf("No words found.\n");
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    start = clock();
    
    const char* sample_text = "This is a sample text with some longer words like 'extraordinary' and 'magnificent'. "
                             "We want to analyze it for word statistics and find the longest word.";
                             
    printf("Sample text: %s\n\n", sample_text);
    
    // Get text statistics
    TextStats* stats = analyze_text(sample_text);
    
    // Display results - this will likely show garbage or crash when trying to print the longest word
    printf("Text Analysis Results:\n");
    display_text_stats(stats);
    
    // Process another text to show how stack memory gets overwritten
    printf("\nProcessing another text...\n");
    const char* another_text = "This is a different sample with other vocabulary.";
    TextStats* stats2 = analyze_text(another_text);
    
    // Display results for the first text again - now definitely corrupted
    printf("\nOriginal Text Analysis Results (likely corrupted):\n");
    display_text_stats(stats); // Same as stats2 due to static result
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nExecution time: %f seconds\n", cpu_time_used);
    
    return 0;
}