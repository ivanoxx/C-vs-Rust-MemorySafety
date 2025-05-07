use std::time::Instant;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Definisi tipe untuk node references
type NodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

// Struktur Node untuk Extended Double Linked List
struct Node<T> {
    data: T,
    prev: Option<WeakNodeRef<T>>,
    next: Option<NodeRef<T>>,
}

// Implementasi untuk Node
impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

// Struktur untuk mengelola Extended Double Linked List
struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<WeakNodeRef<T>>,
    size: usize,
}

// Implementasi untuk DoublyLinkedList
impl<T> DoublyLinkedList<T> 
where T: PartialEq + Copy {
    // Membuat double linked list baru
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    
    // Menambahkan node di awal
    fn insert_at_beginning(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            },
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
        
        self.size += 1;
    }
    
    // Menambahkan node di akhir
    fn insert_at_end(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        
        match self.tail.take() {
            Some(old_tail) => {
                if let Some(old_tail_strong) = old_tail.upgrade() {
                    old_tail_strong.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail_strong));
                    self.tail = Some(Rc::downgrade(&new_node));
                }
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        
        self.size += 1;
    }
    
    // Menambahkan node di posisi tertentu
    fn insert_at_position(&mut self, data: T, position: usize) -> bool {
        if position > self.size {
            return false;
        }
        
        if position == 0 {
            self.insert_at_beginning(data);
            return true;
        }
        
        if position == self.size {
            self.insert_at_end(data);
            return true;
        }
        
        let new_node = Rc::new(RefCell::new(Node::new(data)));
        let mut current = self.head.clone();
        
        for _ in 0..position-1 {
            if let Some(node) = current {
                // PERBAIKAN: Gunakan clone untuk next node
                let next = node.borrow().next.clone();
                current = next;
            }
        }
        
        if let Some(current_node) = current {
            let next_node = current_node.borrow().next.clone();
            
            if let Some(next) = next_node {
                new_node.borrow_mut().next = Some(next.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&current_node));
                
                next.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                current_node.borrow_mut().next = Some(new_node);
                
                self.size += 1;
                return true;
            }
        }
        
        false
    }
    
    // Mencari node dalam linked list
    fn search(&self, key: T) -> bool {
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == key {
                return true;
            }
            // PERBAIKAN: Gunakan clone untuk next node
            let next = node.borrow().next.clone();
            current = next;
        }
        
        false
    }
    
    // Menghapus node di awal
    fn delete_at_beginning(&mut self) -> bool {
        if self.head.is_none() {
            return false;
        }
        
        let old_head = self.head.take().unwrap();
        
        match old_head.borrow_mut().next.take() {
            Some(new_head) => {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            },
            None => {
                self.tail = None;
            }
        }
        
        self.size -= 1;
        true
    }
    
    // Menghapus node di akhir
    fn delete_at_end(&mut self) -> bool {
        if self.tail.is_none() {
            return false;
        }
        
        if let Some(tail) = self.tail.take() {
            if let Some(tail_node) = tail.upgrade() {
                // PERBAIKAN: Ambil prev dalam blok terpisah untuk menghindari borrow issue
                let prev_weak = {
                    let mut tail_ref = tail_node.borrow_mut();
                    tail_ref.prev.take()
                };
                
                match prev_weak {
                    Some(prev_weak) => {
                        if let Some(prev) = prev_weak.upgrade() {
                            prev.borrow_mut().next = None;
                            self.tail = Some(Rc::downgrade(&prev));
                        }
                    },
                    None => {
                        self.head = None;
                    }
                }
                
                self.size -= 1;
                return true;
            }
        }
        
        false
    }
    
    // Menghapus node berdasarkan key
    fn delete_node(&mut self, key: T) -> bool {
        if self.head.is_none() {
            return false;
        }
        
        // Jika node yang akan dihapus adalah head
        if let Some(head) = &self.head {
            if head.borrow().data == key {
                return self.delete_at_beginning();
            }
        }
        
        // Jika node yang akan dihapus adalah tail
        if let Some(tail_weak) = &self.tail {
            if let Some(tail) = tail_weak.upgrade() {
                if tail.borrow().data == key {
                    return self.delete_at_end();
                }
            }
        }
        
        // Mencari node di tengah
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == key {
                // Mengambil prev dan next nodes
                let prev_weak = node.borrow().prev.clone();
                let next_opt = node.borrow().next.clone();
                
                if let (Some(prev_weak), Some(next)) = (prev_weak, next_opt) {
                    if let Some(prev) = prev_weak.upgrade() {
                        // Update links
                        prev.borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                        
                        self.size -= 1;
                        return true;
                    }
                }
            }
            
            // PERBAIKAN: Gunakan clone untuk next node
            let next = node.borrow().next.clone();
            current = next;
        }
        
        false
    }
    
    // Memutar list (reverse)
    fn reverse(&mut self) {
        if self.head.is_none() || self.size <= 1 {
            return;
        }
        
        let mut current = self.head.take();
        let mut prev: Option<NodeRef<T>> = None;
        
        self.tail = match &current {
            Some(node) => Some(Rc::downgrade(node)),
            None => None,
        };
        
        while let Some(current_node) = current {
            // Simpan next sebelum diubah
            let next = current_node.borrow_mut().next.take();
            
            // Tukar prev dan next
            current_node.borrow_mut().next = prev.clone();
            
            if let Some(prev_node) = &prev {
                current_node.borrow_mut().prev = Some(Rc::downgrade(prev_node));
            } else {
                current_node.borrow_mut().prev = None;
            }
            
            // Lanjut ke node berikutnya
            prev = Some(current_node);
            current = next;
        }
        
        // Update head
        self.head = prev;
    }
}

fn main() {
    let start = Instant::now();
    
    // Inisialisasi extended double linked list
    let mut list = DoublyLinkedList::new();
    
    // Menambahkan node di awal dan akhir
    for i in 0..400 {
        list.insert_at_beginning(i);
    }
    
    for i in 400..800 {
        list.insert_at_end(i);
    }
    
    // Menambahkan node di posisi tertentu
    for i in 800..1000 {
        let position = i % list.size;
        list.insert_at_position(i, position);
    }
    
    // Mencari beberapa nilai
    for i in 0..100 {
        let key = (i * 17) % 1000;
        let found = list.search(key);
        if found {
            // println!("Found {}", key);
        } else {
            // println!("{} not found", key);
        }
    }
    
    // Menghapus beberapa node
    for _ in 0..50 {
        list.delete_at_beginning();
    }
    
    for _ in 0..50 {
        list.delete_at_end();
    }
    
    for i in 0..100 {
        let key = (i * 19) % 1000;
        list.delete_node(key);
    }
    
    // Memutar list
    list.reverse();
    
    let duration = start.elapsed();
    println!("Waktu eksekusi: {:?}", duration);
}