use std::time::Instant;

// Struktur Node untuk BST
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Implementasi untuk Node
impl Node {
    // Membuat node baru
    fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

// Struktur untuk mengelola BST
struct BST {
    root: Option<Box<Node>>,
}

// Implementasi untuk BST
impl BST {
    // Membuat BST baru
    fn new() -> Self {
        BST {
            root: None,
        }
    }
    
    // Menyisipkan node ke dalam BST
    fn insert(&mut self, data: i32) {
        self.root = Self::insert_recursive(self.root.take(), data);
    }
    
    // Helper function untuk insert
    fn insert_recursive(node: Option<Box<Node>>, data: i32) -> Option<Box<Node>> {
        match node {
            None => Some(Box::new(Node::new(data))),
            Some(mut node) => {
                if data < node.data {
                    node.left = Self::insert_recursive(node.left.take(), data);
                } else if data > node.data {
                    node.right = Self::insert_recursive(node.right.take(), data);
                }
                Some(node)
            }
        }
    }
    
    // Mencari nilai dalam BST
    fn search(&self, key: i32) -> bool {
        Self::search_recursive(&self.root, key)
    }
    
    // Helper function untuk search
    fn search_recursive(node: &Option<Box<Node>>, key: i32) -> bool {
        match node {
            None => false,
            Some(node) => {
                if node.data == key {
                    true
                } else if key < node.data {
                    Self::search_recursive(&node.left, key)
                } else {
                    Self::search_recursive(&node.right, key)
                }
            }
        }
    }
}

fn main() {
    let start = Instant::now();
    
    // Inisialisasi BST
    let mut bst = BST::new();
    
    // Menambahkan node (misal 1000 node dengan nilai deterministik)
    for i in 0..1000 {
        let value = (i * 83) % 10000;
        bst.insert(value);
    }
    
    // Mencari beberapa nilai
    for i in 0..100 {
        let key = (i * 97) % 10000;
        let found = bst.search(key);
        if found {
            println!("Found {}", key);
        } else {
            println!("{} not found", key);
        }
    }
    
    // BST akan dibersihkan secara otomatis ketika keluar dari scope
    
    let duration = start.elapsed();
    println!("Waktu eksekusi: {:?}", duration);
}