use std::time::Instant;

// Represents a resource with an ID and data payload
struct Resource {
    id: i32,
    name: String,
    values: Vec<f64>,
}

// Implementation details for Resource
impl Resource {
    // Create a new resource
    fn new(id: i32, name: &str, values_count: usize) -> Self {
        let mut values = Vec::with_capacity(values_count);
        
        // Initialize with some values
        for i in 0..values_count {
            values.push(id as f64 * 100.0 + i as f64);
        }
        
        println!("Created resource {} ({}) with {} values", id, name, values_count);
        
        Resource {
            id,
            name: name.to_string(),
            values,
        }
    }
    
    // Print resource details
    fn print(&self) {
        println!("Resource {}: {}", self.id, self.name);
        print!("Values: ");
        for val in &self.values {
            print!("{:.1} ", val);
        }
        println!();
    }
}

// Global cache of resource references (raw pointers, not owners!)
struct ResourceCache {
    cache: Vec<*const Resource>,
}

impl ResourceCache {
    // Initialize the cache
    fn new(capacity: usize) -> Self {
        ResourceCache {
            cache: Vec::with_capacity(capacity),
        }
    }
    
    // Add a resource reference to cache (does NOT take ownership)
    fn cache_resource(&mut self, res: &Resource) {
        let ptr = res as *const Resource;
        self.cache.push(ptr);
        println!("Resource {} ({}) cached at index {}", res.id, res.name, self.cache.len() - 1);
    }
    
    // Access cached resources (dangerous after resource is dropped)
    unsafe fn access_cached_resources(&self) {
        println!("\nAccessing cached resources:");
        for (i, &ptr) in self.cache.iter().enumerate() {
            print!("Cache index {}: ", i);
            
            // Dangerous: dereferencing potentially dangling pointers
            let res_ref = &*ptr; // Potential dangling heap pointer dereference!
            res_ref.print();
        }
    }
}

fn main() {
    let start = Instant::now();
    
    // Initialize our resource cache
    let mut cache = ResourceCache::new(10);
    
    // Create resources
    let res1 = Resource::new(1, "First Resource", 3);
    let res2 = Resource::new(2, "Second Resource", 5);
    let res3 = Resource::new(3, "Third Resource", 2);
    
    // Cache the resources (just references, not ownership)
    cache.cache_resource(&res1);
    cache.cache_resource(&res2);
    cache.cache_resource(&res3);
    
    println!("\nAccessing resources (safe at this point):");
    unsafe {
        cache.access_cached_resources();
    }
    
    // Now let's create a scope where res1 and res3 will be dropped at the end
    {
        // Move res1 and res3 into this scope
        let _temp_res1 = res1;
        let _temp_res3 = res3;
        
        println!("\nResources 1 and 3 will be dropped at the end of this scope...");
    } // res1 and res3 are dropped here!
    
    // WARNING: cache still has pointers to res1 and res3, but they are now dangling!
    
    println!("\nAttempting to access cached resources after dropping 1 and 3:");
    // This will cause dangling heap pointer dereferences for res1 and res3
    unsafe {
        cache.access_cached_resources();
    }
    
    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);
}