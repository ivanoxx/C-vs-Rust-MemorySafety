#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Represents a resource with an ID and data payload
typedef struct {
    int id;
    char* name;
    double* values;
    int values_count;
} Resource;

// Global cache of resource references (not owners!)
Resource** resource_cache;
int cache_size = 0;
int cache_capacity = 10;

// Initialize the cache
void init_cache() {
    resource_cache = (Resource**)malloc(sizeof(Resource*) * cache_capacity);
    for (int i = 0; i < cache_capacity; i++) {
        resource_cache[i] = NULL;
    }
}

// Add a resource reference to cache (does NOT take ownership)
void cache_resource(Resource* res) {
    if (cache_size < cache_capacity) {
        resource_cache[cache_size++] = res;
        printf("Resource %d (%s) cached at index %d\n", res->id, res->name, cache_size - 1);
    }
}

// Create a new resource
Resource* create_resource(int id, const char* name, int values_count) {
    Resource* res = (Resource*)malloc(sizeof(Resource));
    
    res->id = id;
    res->name = (char*)malloc(strlen(name) + 1);
    strcpy(res->name, name);
    
    res->values_count = values_count;
    res->values = (double*)malloc(sizeof(double) * values_count);
    
    // Initialize with some values
    for (int i = 0; i < values_count; i++) {
        res->values[i] = id * 100.0 + i;
    }
    
    printf("Created resource %d (%s) with %d values\n", id, name, values_count);
    return res;
}

// Free a resource's memory
void free_resource(Resource* res) {
    if (res != NULL) {
        printf("Freeing resource %d (%s)...\n", res->id, res->name);
        free(res->name);
        free(res->values);
        free(res);
    }
}

// Print resource details
void print_resource(Resource* res) {
    printf("Resource %d: %s\n", res->id, res->name);
    printf("Values: ");
    for (int i = 0; i < res->values_count; i++) {
        printf("%.1f ", res->values[i]);
    }
    printf("\n");
}

// Access cached resources (dangerous after resource is freed)
void access_cached_resources() {
    printf("\nAccessing cached resources:\n");
    for (int i = 0; i < cache_size; i++) {
        printf("Cache index %d: ", i);
        
        // Dangerous: dereferencing potentially freed pointers
        print_resource(resource_cache[i]); // Potential dangling heap pointer dereference!
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    start = clock();
    
    // Initialize our resource cache
    init_cache();
    
    // Create resources and add references to cache
    Resource* res1 = create_resource(1, "First Resource", 3);
    Resource* res2 = create_resource(2, "Second Resource", 5);
    Resource* res3 = create_resource(3, "Third Resource", 2);
    
    // Cache the resources (just references, not ownership)
    cache_resource(res1);
    cache_resource(res2);
    cache_resource(res3);
    
    printf("\nAccessing resources (safe at this point):\n");
    access_cached_resources();
    
    // Now free the first and third resources
    printf("\nFreeing resources 1 and 3...\n");
    free_resource(res1);
    free_resource(res3);
    
    // WARNING: res1 and res3 now point to freed memory!
    // But they're still in the cache...
    
    printf("\nAttempting to access cached resources after freeing 1 and 3:\n");
    // This will cause dangling heap pointer dereferences for res1 and res3
    access_cached_resources();
    
    // Clean up resources
    free_resource(res2); // Only free res2, as res1 and res3 are already freed
    free(resource_cache);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nExecution time: %f seconds\n", cpu_time_used);
    
    return 0;
}