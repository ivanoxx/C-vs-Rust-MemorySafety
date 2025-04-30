use std::time::Instant;

const MAX_STUDENTS: usize = 5;

struct Student {
    id: i32,
    name: String,
    scores: [f32; 3],
    average: f32,
}

fn calculate_averages(students: &mut [Student]) {
    for i in 0..=students.len() {  // Bug: ..= bukannya ..
        if i < students.len() {  // Rust akan panic tanpa pengecekan ini
            let sum: f32 = students[i].scores.iter().sum();
            students[i].average = sum / 3.0;
        }
    }
}

fn display_sorted_students(students: &[Student]) {
    let mut indices: Vec<usize> = (0..MAX_STUDENTS).collect();
    
    indices.sort_by(|&a, &b| students[b].average.partial_cmp(&students[a].average).unwrap());
    
    println!("\nDaftar Mahasiswa (Terurut berdasarkan nilai):");
    println!("ID\tNama\t\tNilai Rata-rata");
    
    for i in 0..students.len() + 1 {  // Bug: + 1 akan melewati batas
        if i < indices.len() {  // Rust akan panic tanpa pengecekan ini
            let idx = indices[i];
            println!("{}\t{}\t\t{:.2}", 
                    students[idx].id, 
                    students[idx].name, 
                    students[idx].average);
        }
    }
}

fn find_student_by_id(students: &[Student], search_id: i32) {
    for i in 0..=students.len() {  // Bug: ..= bukannya ..
        // if i < students.len() {  // Rust akan panic tanpa pengecekan ini
            if students[i].id == search_id {
                println!("Mahasiswa ditemukan:");
                println!("Nama: {}", students[i].name);
                println!("Nilai rata-rata: {:.2}", students[i].average);
                return;
            }
        // }
    }
    println!("Mahasiswa dengan ID {} tidak ditemukan.", search_id);
}

fn main() {
    let mut students = [
        Student { id: 101, name: String::from("Ali"), scores: [85.5, 90.0, 82.5], average: 0.0 },
        Student { id: 102, name: String::from("Budi"), scores: [75.0, 80.5, 85.0], average: 0.0 },
        Student { id: 103, name: String::from("Cindy"), scores: [95.0, 92.5, 88.0], average: 0.0 },
        Student { id: 104, name: String::from("Dodi"), scores: [70.0, 65.5, 75.0], average: 0.0 },
        Student { id: 105, name: String::from("Eka"), scores: [88.0, 84.5, 90.0], average: 0.0 },
    ];
    
    let start = Instant::now();
    
    calculate_averages(&mut students);
    
    display_sorted_students(&students);
    
    find_student_by_id(&students, 103);
    
    // Mencari mahasiswa dengan ID yang tidak ada
    find_student_by_id(&students, 110);
    
    let duration = start.elapsed();
    println!("\nWaktu eksekusi: {:?}", duration);
}   