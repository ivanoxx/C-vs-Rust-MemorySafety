use std::time::Instant;

struct TaskNode {
    id: i32,
    description: String,
    priority: i32,       
    next: *mut TaskNode,
    dependency: *mut TaskNode,   // Task that must be completed before this one
}

fn create_task(id: i32, description: &str, priority: i32) -> *mut TaskNode {
    let new_task = Box::new(TaskNode {
        id,
        description: description.to_string(),
        priority,
        next: std::ptr::null_mut(),
        dependency: std::ptr::null_mut(),
    });
    
    Box::into_raw(new_task)
}

fn add_task(head: &mut *mut TaskNode, new_task: *mut TaskNode) {
    unsafe {
        if head.is_null() {
            *head = new_task;
            return;
        }
        
        let mut current = *head;
        while !(*current).next.is_null() {
            current = (*current).next;
        }
        (*current).next = new_task;
    }
}

fn set_dependency(task: *mut TaskNode, dependency: *mut TaskNode) {
    unsafe {
        if !task.is_null() {
            (*task).dependency = dependency;
        }
    }
}

fn find_task(head: *mut TaskNode, id: i32) -> *mut TaskNode {
    unsafe {
        let mut current = head;
        while !current.is_null() {
            if (*current).id == id {
                return current;
            }
            current = (*current).next;
        }
        std::ptr::null_mut() // Task not found
    }
}

// Function to print task details - has null pointer dereference risk
fn print_task_details(task: *mut TaskNode) {
    unsafe {
        println!("Task ID: {}", (*task).id); // Will panic if task is NULL
        println!("Description: {}", (*task).description);
        println!("Priority: {}", (*task).priority);
        
        // Intentional null pointer dereference risk:
        println!("Dependency Task ID: {}", (*(*task).dependency).id); // Will panic if dependency is NULL
        println!("Dependency Description: {}", (*(*task).dependency).description);
    }
}

fn complete_task(head: *mut TaskNode, id: i32) {
    unsafe {
        let task = find_task(head, id);
        
        // Unsafe: No NULL check before dereferencing
        println!("Completing task: {} (ID: {})", (*task).description, (*task).id);
        
        if !(*task).dependency.is_null() {
            println!("Must first complete dependency: {} (ID: {})", 
                   (*(*task).dependency).description, 
                   (*(*task).dependency).id);
        } else {
            // Another intentional null pointer dereference:
            println!("No dependency ID needed, value is: {}", (*(*task).dependency).id); // Will panic
        }
    }
}

fn free_tasks(head: *mut TaskNode) {
    unsafe {
        let mut current = head;
        while !current.is_null() {
            let next = (*current).next;
            drop(Box::from_raw(current));
            current = next;
        }
    }
}

fn main() {
    let start = Instant::now();
    
    let mut task_list: *mut TaskNode = std::ptr::null_mut();
    
    let task1 = create_task(1, "Complete project proposal", 5);
    let task2 = create_task(2, "Research competitors", 3);
    let task3 = create_task(3, "Prepare presentation", 4);
    let task4: *mut TaskNode = std::ptr::null_mut(); // Deliberately NULL task
    
    add_task(&mut task_list, task1);
    add_task(&mut task_list, task2);
    add_task(&mut task_list, task3);
    
    set_dependency(task3, task1); // Task 3 depends on Task 1
    set_dependency(task1, task2); // Task 1 depends on Task 2
    
    // Print task list (safe)
    println!("Task List:");
    unsafe {
        let mut current = task_list;
        while !current.is_null() {
            println!("- ID: {}, Description: {}, Priority: {}", 
                   (*current).id, (*current).description, (*current).priority);
            current = (*current).next;
        }
    }
    
    println!("\nDetailed Task View:");
    
    // Intentional NULL pointer dereference 1:
    println!("\nTask 4 Details (NULL):");
    print_task_details(task4); // Will panic - task4 is NULL
    
    
    // Intentional NULL pointer dereference 2:
    println!("\nCompleting Task 2:");
    complete_task(task_list, 2); // Will panic when checking dependency->id which is NULL
    
    free_tasks(task_list);
    
    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);
}