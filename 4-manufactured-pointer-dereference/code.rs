use std::time::Instant;

struct CacheEntry {
    key: i32,
    data: [u8; 32],
    is_valid: i32,
}

struct MemoryCache {
    entries: Vec<CacheEntry>,
    capacity: usize,
    fast_access_table: Vec<usize>, // Table of offsets for fast lookups
}

impl MemoryCache {
    fn new(capacity: usize) -> Self {
        let mut entries = Vec::with_capacity(capacity);
        let mut fast_access_table = Vec::with_capacity(capacity);
        
        for i in 0..capacity {
            entries.push(CacheEntry {
                key: -1,
                data: [0; 32],
                is_valid: 0,
            });
            fast_access_table.push(i); // Initially just direct mapping
        }
        
        MemoryCache {
            entries,
            capacity,
            fast_access_table,
        }
    }
    
    fn add_to_cache_unsafe(&mut self, key: i32, data: &str) {
        // Find an empty slot
        let index = (key as usize) % self.capacity;
        self.entries[index].key = key;
        
        // Copy data safely (with bounds checking)
        let bytes = data.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), 31);
        self.entries[index].data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        self.entries[index].is_valid = 1;
    }
}

// This function deliberately creates a "manufactured" pointer
// by converting an arbitrary integer to a pointer
fn get_direct_pointer_unsafe(address_value: usize) -> *mut CacheEntry {
    // Intentionally creating a manufactured pointer
    let manufactured_ptr = address_value as *mut CacheEntry;
    manufactured_ptr
}

// Attempt to read from cache with direct pointer access (unsafe)
fn read_with_manufactured_pointer(_cache: &MemoryCache, address_value: usize) {
    println!("Attempting to read from manufactured pointer at address 0x{:X}", address_value);
    
    // Get a manufactured pointer to what we "think" is a cache entry
    let entry_ptr = get_direct_pointer_unsafe(address_value);
    
    // Dangerous: Dereferencing a manufactured pointer
    unsafe {
        println!("Key: {}", (*entry_ptr).key);
        
        // Convert data to a string for printing
        let data_slice = &(*entry_ptr).data;
        let data_str = std::str::from_utf8_unchecked(data_slice);
        println!("Data: {}", data_str);
        
        println!("Is Valid: {}", (*entry_ptr).is_valid);
    }
}

fn main() {
    let start = Instant::now();
    
    let mut cache = MemoryCache::new(10);
    
    cache.add_to_cache_unsafe(123, "Test data 1");
    cache.add_to_cache_unsafe(456, "Test data 2");
    cache.add_to_cache_unsafe(789, "Test data 3");
    
    println!("Cache initialized with 3 entries\n");
    
    println!("Reading valid cache entry:");
    let valid_entry = &cache.entries[0];
    println!("Key: {}", valid_entry.key);
    
    let data_str = std::str::from_utf8(&valid_entry.data).unwrap_or("Invalid UTF-8");
    println!("Data: {}", data_str);
    
    println!("Is Valid: {}\n", valid_entry.is_valid);
    
    // Now attempt access with manufactured pointers
    // This will likely cause a panic in Rust
    println!("Reading with manufactured pointer:");
    read_with_manufactured_pointer(&cache, 0x1337);
    
    // If the program hasn't panicked, try another bad address
    println!("\nReading with another manufactured pointer:");
    read_with_manufactured_pointer(&cache, 0x42424242);
    
    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);
}