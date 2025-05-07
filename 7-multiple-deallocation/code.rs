use std::rc::Rc;
use std::time::Instant;

// Struktur data student
struct Student {
    id: i32,
    name: String,
    score: f32,
}

// Implementasi metode untuk Student
impl Student {
    fn new(id: i32, name: &str, score: f32) -> Self {
        Student {
            id,
            name: String::from(name),
            score,
        }
    }
}

// Fungsi untuk mencari student berdasarkan ID
fn find_student(students: &Vec<Rc<Student>>, target_id: i32) {
    for student in students {
        if student.id == target_id {
            println!("Found student: {} - {} (Score: {:.2})",
                     student.id, student.name, student.score);
            return;
        }
    }
    println!("Student with ID {} not found.", target_id);
}

// Fungsi yang mencoba menyebabkan multiple deallocation
fn process_student_records(students: &mut Vec<Rc<Student>>) {
    // Memproses data student
    for student in students.iter() {
        println!("Processing student: {} - {} (Score: {:.2})", 
                 student.id, student.name, student.score);
        
        // Catatan: Score tidak dapat dimodifikasi karena Student di-wrap dalam Rc
        // dan bersifat immutable
    }
    
    // Mencari beberapa student
    find_student(students, 101);
    find_student(students, 105);
    
    // Mencoba untuk "drop" student[0] secara manual
    // Tidak masalah karena Rc mengelola reference counting
    if !students.is_empty() {
        // Dalam Rust, ini hanya mengurangi reference count, bukan menghapus memori
        let first_student = students[0].clone();
        drop(first_student); // Mengurangi reference count
    }
    
    // Mencoba drop semua students
    // Dalam Rust, ini aman karena sistem kepemilikan
    for i in 0..students.len() {
        let student = students[i].clone();
        drop(student); // Mengurangi reference count
    }
}

fn main() {
    // Membuat vector Rc<Student> untuk berbagi kepemilikan
    let mut class_roster: Vec<Rc<Student>> = vec![
        Rc::new(Student::new(101, "Alice", 85.5)),
        Rc::new(Student::new(102, "Bob", 78.0)),
        Rc::new(Student::new(103, "Charlie", 92.5)),
        Rc::new(Student::new(104, "Diana", 88.0)),
        Rc::new(Student::new(105, "Edward", 79.5)),
    ];
    
    let start = Instant::now();
    
    // Proses data - tidak akan menyebabkan double free karena Rust's ownership system
    process_student_records(&mut class_roster);
    
    let duration = start.elapsed();
    println!("\nWaktu eksekusi: {:?}", duration);
    
    // Semua resource akan dibersihkan secara otomatis saat scope berakhir
    println!("Program completed successfully");
}