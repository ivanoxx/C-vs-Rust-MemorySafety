use std::time::Instant;

struct Rectangle {
    length: f64,
    width: f64,
    area: *mut f64,     // Pointer to area calculation result
    perimeter: *mut f64, // Pointer to perimeter calculation result
}

fn calculate_area(r: &mut Rectangle) {
    unsafe {
        r.area = Box::into_raw(Box::new(0.0));
        *(r.area) = r.length * r.width;
    }
}

fn print_info(r: &Rectangle) {
    println!("Rectangle: {:.2} x {:.2}", r.length, r.width);
    
    // Dereference area pointer (potentially uninitialized)
    unsafe {
        println!("Area: {:.2}", *(r.area));
    }
    
    // Dereference perimeter pointer (uninitialized)
    unsafe {
        println!("Perimeter: {:.2}", *(r.perimeter)); // Uninitialized pointer dereference!
    }
}

fn create_rectangle(length: f64, width: f64) -> Rectangle {
    Rectangle {
        length,
        width,
        area: std::ptr::null_mut(),      // null pointer in Rust
        perimeter: std::ptr::null_mut(), // null pointer in Rust
    }
}

fn free_rectangle(r: Rectangle) {
    unsafe {
        if !r.area.is_null() {
            drop(Box::from_raw(r.area));
        }
        
        if !r.perimeter.is_null() {
            drop(Box::from_raw(r.perimeter));
        }
        
        // Struct r will be freed automatically when it goes out of scope
    }
}

fn main() {
    let start = Instant::now();
    
    let mut rect1 = create_rectangle(5.0, 3.0);
    let rect2 = create_rectangle(7.5, 2.5);
    
    calculate_area(&mut rect1);
    
    // Print information (will dereference uninitialized pointers)
    println!("Rectangle 1 Information:");
    print_info(&rect1);  // Uninitialized pointer dereference for perimeter
    
    println!("\nRectangle 2 Information:");
    print_info(&rect2);  // Uninitialized pointer dereference for both area and perimeter
    
    free_rectangle(rect1);
    free_rectangle(rect2);
    
    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);
}