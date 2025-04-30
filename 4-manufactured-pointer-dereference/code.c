#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct {
    int key;
    char data[32];
    int is_valid;
} CacheEntry;

typedef struct {
    CacheEntry* entries;
    int capacity;
    int* fast_access_table; // Table of offsets for fast lookups
} MemoryCache;

MemoryCache* initializeCache(int capacity) {
    MemoryCache* cache = (MemoryCache*)malloc(sizeof(MemoryCache));
    if (cache == NULL) {
        return NULL;
    }
    
    cache->capacity = capacity;
    cache->entries = (CacheEntry*)malloc(sizeof(CacheEntry) * capacity);
    cache->fast_access_table = (int*)malloc(sizeof(int) * capacity);
    
    if (cache->entries == NULL || cache->fast_access_table == NULL) {
        free(cache->entries);
        free(cache->fast_access_table);
        free(cache);
        return NULL;
    }
    
    for (int i = 0; i < capacity; i++) {
        cache->entries[i].key = -1;
        memset(cache->entries[i].data, 0, 32);
        cache->entries[i].is_valid = 0;
        cache->fast_access_table[i] = i; // Initially just direct mapping
    }
    
    return cache;
}

void addToCacheUnsafe(MemoryCache* cache, int key, const char* data) {
    // Find an empty slot
    int index = key % cache->capacity;
    cache->entries[index].key = key;
    strncpy(cache->entries[index].data, data, 31);
    cache->entries[index].data[31] = '\0';
    cache->entries[index].is_valid = 1;
}

// This function deliberately creates a "manufactured" pointer 
// by converting an arbitrary integer to a pointer
CacheEntry* getDirectPointerUnsafe(int address_value) {
    // Intentionally creating a manufactured pointer
    // 0x1337 is an arbitrary address that is unlikely to be valid
    CacheEntry* manufactured_ptr = (CacheEntry*)address_value;
    return manufactured_ptr;
}

// Attempt to read from cache with direct pointer access (unsafe)
void readWithManufacturedPointer(MemoryCache* cache, int address_value) {
    printf("Attempting to read from manufactured pointer at address 0x%X\n", address_value);
    
    // Get a manufactured pointer to what we "think" is a cache entry
    CacheEntry* entry_ptr = getDirectPointerUnsafe(address_value);
    
    // Dangerous: Dereferencing a manufactured pointer
    printf("Key: %d\n", entry_ptr->key);
    printf("Data: %s\n", entry_ptr->data);
    printf("Is Valid: %d\n", entry_ptr->is_valid);
}

void freeCache(MemoryCache* cache) {
    if (cache != NULL) {
        free(cache->entries);
        free(cache->fast_access_table);
        free(cache);
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    start = clock();
    
    MemoryCache* cache = initializeCache(10);
    if (cache == NULL) {
        printf("Failed to initialize cache\n");
        return 1;
    }
    
    addToCacheUnsafe(cache, 123, "Test data 1");
    addToCacheUnsafe(cache, 456, "Test data 2");
    addToCacheUnsafe(cache, 789, "Test data 3");
    
    printf("Cache initialized with 3 entries\n\n");
    
    // First, try a valid access for comparison
    printf("Reading valid cache entry:\n");
    CacheEntry* valid_entry = &(cache->entries[0]);
    printf("Key: %d\n", valid_entry->key);
    printf("Data: %s\n", valid_entry->data);
    printf("Is Valid: %d\n\n", valid_entry->is_valid);
    
    // Now attempt access with manufactured pointers
    // This will likely crash or show garbage data
    printf("Reading with manufactured pointer:\n");
    readWithManufacturedPointer(cache, 0x1337);
    
    // If the program hasn't crashed, try another bad address
    printf("\nReading with another manufactured pointer:\n");
    readWithManufacturedPointer(cache, 0x42424242);
    
    freeCache(cache);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nExecution time: %f seconds\n", cpu_time_used);
    
    return 0;
}