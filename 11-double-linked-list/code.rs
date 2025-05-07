use std::time::Instant;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Struktur Node untuk Double Linked List
type NodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

struct Node<T> {
    data: T,
    prev: Option<WeakNodeRef<T>>,
    next: Option<NodeRef<T>>,
}

// Implementasi untuk Node
impl<T> Node<T> {
    // Membuat node baru
    fn new(data: T) -> Self {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

// Struktur untuk mengelola Double Linked List
struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<WeakNodeRef<T>>,
    size: usize,
}

// Implementasi untuk DoublyLinkedList
impl<T> DoublyLinkedList<T> 
where T: PartialEq {
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
    
    // Mencari node dalam linked list
    fn search(&self, key: T) -> bool {
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == key {
                return true;
            }
            let next = node.borrow().next.clone();
            current = next;
        }
        
        false
    }
    
    // Menghapus node dari linked list
    fn delete(&mut self, key: T) -> bool {
        if self.head.is_none() {
            return false;
        }
        
        // Jika node yang dihapus adalah head
        if self.head.as_ref().unwrap().borrow().data == key {
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
            return true;
        }
        
        // Mencari node yang akan dihapus
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == key {
                // Update prev node
                if let Some(prev) = node.borrow().prev.as_ref().and_then(|w| w.upgrade()) {
                    prev.borrow_mut().next = node.borrow().next.clone();
                }
                
                // Update next node
                if let Some(next) = &node.borrow().next {
                    next.borrow_mut().prev = node.borrow().prev.clone();
                } else {
                    // Jika node adalah tail
                    self.tail = node.borrow().prev.clone();
                }
                
                self.size -= 1;
                return true;
            }
            
            current = match &node.borrow().next {
                Some(next) => Some(next.clone()),
                None => None,
            };
        }
        
        false
    }
}

fn main() {
    let start = Instant::now();
    
    // Inisialisasi double linked list
    let mut list = DoublyLinkedList::new();
    
    // Menambahkan node di awal (misal 500 node)
    for i in 0..500 {
        list.insert_at_beginning(i);
    }
    
    // Menambahkan node di akhir (misal 500 node)
    for i in 500..1000 {
        list.insert_at_end(i);
    }
    
    // Mencari beberapa nilai (dengan nilai deterministik)
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
    for i in 0..100 {
        let key = (i * 19) % 1000;
        let deleted = list.delete(key);
        if deleted {
            // println!("Deleted {}", key);
        } else {
            // println!("Failed to delete {}", key);
        }
    }
    
    let duration = start.elapsed();
    println!("Waktu eksekusi: {:?}", duration);
}