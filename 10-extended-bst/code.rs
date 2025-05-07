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
    
    // Menghapus node dari BST
    fn delete(&mut self, key: i32) {
        self.root = Self::delete_recursive(self.root.take(), key);
    }
    
    // Helper function untuk delete
    fn delete_recursive(node: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        match node {
            None => None,
            Some(mut node) => {
                if key < node.data {
                    node.left = Self::delete_recursive(node.left.take(), key);
                    Some(node)
                } else if key > node.data {
                    node.right = Self::delete_recursive(node.right.take(), key);
                    Some(node)
                } else {
                    // Node tanpa anak
                    if node.left.is_none() && node.right.is_none() {
                        None
                    }
                    // Node dengan satu anak (kanan)
                    else if node.left.is_none() {
                        node.right
                    }
                    // Node dengan satu anak (kiri)
                    else if node.right.is_none() {
                        node.left
                    }
                    // Node dengan dua anak
                    else {
                        let min_value = Self::find_min(&node.right);
                        node.data = min_value;
                        node.right = Self::delete_recursive(node.right.take(), min_value);
                        Some(node)
                    }
                }
            }
        }
    }
    
    // Mencari nilai minimum dalam BST
    fn find_min(node: &Option<Box<Node>>) -> i32 {
        match node {
            None => panic!("Empty tree"),
            Some(node) => {
                match &node.left {
                    None => node.data,
                    Some(_) => Self::find_min(&node.left),
                }
            }
        }
    }
    
    // Traversal inorder
    fn inorder(&self) {
        Self::inorder_recursive(&self.root);
    }
    
    // Helper function untuk inorder
    fn inorder_recursive(node: &Option<Box<Node>>) {
        if let Some(node) = node {
            Self::inorder_recursive(&node.left);
            // print!("{} ", node.data);
            Self::inorder_recursive(&node.right);
        }
    }
    
    // Traversal preorder
    fn preorder(&self) {
        Self::preorder_recursive(&self.root);
    }
    
    // Helper function untuk preorder
    fn preorder_recursive(node: &Option<Box<Node>>) {
        if let Some(node) = node {
            // print!("{} ", node.data);
            Self::preorder_recursive(&node.left);
            Self::preorder_recursive(&node.right);
        }
    }
    
    // Traversal postorder
    fn postorder(&self) {
        Self::postorder_recursive(&self.root);
    }
    
    // Helper function untuk postorder
    fn postorder_recursive(node: &Option<Box<Node>>) {
        if let Some(node) = node {
            Self::postorder_recursive(&node.left);
            Self::postorder_recursive(&node.right);
            // print!("{} ", node.data);
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
    
    // Traversal
    bst.inorder();
    bst.preorder();
    bst.postorder();
    
    // Menghapus beberapa node
    for i in 0..100 {
        let key = (i * 97) % 10000;
        bst.delete(key);
    }
    
    // Mencari beberapa nilai
    for i in 0..100 {
        let key = (i * 101) % 10000;
        let found = bst.search(key);
        if found {
            // println!("Found {}", key);
        } else {
            // println!("{} not found", key);
        }
    }
    
    let duration = start.elapsed();
    println!("Waktu eksekusi: {:?}", duration);
}