use std::time::Instant;

const MAX_BUFFER_SIZE: usize = 1024;

struct TextStats {
    word_count: i32,
    sentence_count: i32,
    longest_word_length: i32,
    longest_word: *const u8,  // Raw pointer that could dangle
}

fn analyze_text(input_text: &str) -> TextStats {
    let mut word_buffer = [0u8; MAX_BUFFER_SIZE];
    
    let mut result = TextStats {
        word_count: 0,
        sentence_count: 0,
        longest_word_length: 0,
        longest_word: std::ptr::null(),
    };
    
    let mut in_word = false;
    let mut current_word_length = 0;
    let mut word_buffer_pos = 0;
    
    for c in input_text.chars() {
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            if in_word {
                // End of a word
                in_word = false;
                word_buffer[word_buffer_pos] = 0; // Null terminator
                
                result.word_count += 1;
                
                // Check if this is the longest word so far
                if current_word_length > result.longest_word_length {
                    result.longest_word_length = current_word_length;
                    
                    // DANGEROUS: Pointing to stack memory that will be deallocated
                    // This is only possible in unsafe Rust
                    // unsafe {
                        result.longest_word = word_buffer.as_ptr();
                    // }
                }
                
                // Reset for next word
                word_buffer_pos = 0;
                current_word_length = 0;
            }
        } else {
            // Process character as part of a word
            if !in_word {
                in_word = true;
            }
            
            // Check for end of sentences
            if c == '.' || c == '!' || c == '?' {
                result.sentence_count += 1;
            }
            
            // Add character to current word
            if word_buffer_pos < MAX_BUFFER_SIZE - 1 {
                let mut bytes = [0u8; 4];
                let byte_count = c.encode_utf8(&mut bytes).len();
                
                for i in 0..byte_count {
                    if word_buffer_pos < MAX_BUFFER_SIZE - 1 {
                        word_buffer[word_buffer_pos] = bytes[i];
                        word_buffer_pos += 1;
                    }
                }
                
                current_word_length += 1; // Count characters, not bytes
            }
        }
    }
    
    // Handle case where text ends with a word
    if in_word {
        word_buffer[word_buffer_pos] = 0; // Null terminator
        result.word_count += 1;
        
        if current_word_length > result.longest_word_length {
            result.longest_word_length = current_word_length;
            
            // DANGEROUS: Pointing to stack memory that will be deallocated
            // unsafe {
                result.longest_word = word_buffer.as_ptr();
            // }
        }
    }
    
    // If no sentences ended with punctuation, count at least one sentence
    if result.sentence_count == 0 && result.word_count > 0 {
        result.sentence_count = 1;
    }
    
    result
}

fn display_text_stats(stats: &TextStats) {
    println!("Word count: {}", stats.word_count);
    println!("Sentence count: {}", stats.sentence_count);
    println!("Longest word length: {}", stats.longest_word_length);
    
    // DANGEROUS: Dereferencing a dangling pointer!
    if !stats.longest_word.is_null() {
        unsafe {
            // Attempt to reconstruct string from pointer (likely to fail or show garbage)
            let mut len = 0;
            while *stats.longest_word.add(len) != 0 && len < MAX_BUFFER_SIZE {
                len += 1;
            }
            
            let slice = std::slice::from_raw_parts(stats.longest_word, len);
            match std::str::from_utf8(slice) {
                Ok(s) => println!("Longest word: {}", s),
                Err(_) => println!("Longest word: <invalid UTF-8>"),
            }
        }
    } else {
        println!("No words found.");
    }
}

fn main() {
    let start = Instant::now();
    
    let sample_text = "This is a sample text with some longer words like 'extraordinary' and 'magnificent'. \
                      We want to analyze it for word statistics and find the longest word.";
                      
    println!("Sample text: {}\n", sample_text);
    
    let stats = analyze_text(sample_text);
    
    // Display results - this will likely show garbage when trying to print the longest word
    println!("Text Analysis Results:");
    display_text_stats(&stats);
    
    // Process another text to show how stack memory gets overwritten
    println!("\nProcessing another text...");
    let another_text = "This is a different sample with other vocabulary.";
    let _stats2 = analyze_text(another_text);
    
    // Display results for the first text again - now definitely corrupted
    println!("\nOriginal Text Analysis Results (likely corrupted):");
    display_text_stats(&stats);
    
    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);
}