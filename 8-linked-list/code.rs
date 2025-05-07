use std::time::Instant;

// Definisi Node untuk Linked List
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Implementasi untuk Node
impl Node {
    // Membuat node baru
    fn new(data: i32) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

// Struktur LinkedList untuk mengelola linked list
struct LinkedList {
    head: Option<Box<Node>>,
}

// Implementasi untuk LinkedList
impl LinkedList {
    // Membuat linked list baru
    fn new() -> Self {
        LinkedList {
            head: None,
        }
    }
    
    // Menambahkan node di awal
    fn insert_at_beginning(&mut self, data: i32) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
    
    // Mencari node dalam linked list
    fn search(&self, key: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == key {
                return true;
            }
            current = &node.next;
        }
        false
    }
}

fn main() {
    let start = Instant::now();
    
    // Inisialisasi linked list
    let mut list = LinkedList::new();
    
    // Menambahkan node (misal 1000 node)
    for i in 0..1000 {
        list.insert_at_beginning(i);
    }
    
    for i in 0..100 {
        let key = (i * 17) % 1000;
        let found = list.search(key);
        if found {
            println!("Found {}", key);
        } else {
            println!("{} not found", key);
        }
    }
    
    // Linked list akan dibersihkan secara otomatis ketika keluar dari scope
    
    let duration = start.elapsed();
    println!("Waktu eksekusi: {:?}", duration);
}