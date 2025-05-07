#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Struktur data sederhana untuk digunakan
typedef struct {
    int id;
    char* name;
    float score;
} Student;

// Fungsi untuk membuat student baru
Student* create_student(int id, const char* name, float score) {
    Student* s = (Student*)malloc(sizeof(Student));
    if (s == NULL) {
        return NULL;
    }
    
    s->id = id;
    s->name = (char*)malloc(strlen(name) + 1);
    if (s->name == NULL) {
        free(s);
        return NULL;
    }
    
    strcpy(s->name, name);
    s->score = score;
    return s;
}

// Fungsi untuk membebaskan memori student
void free_student(Student* s) {
    if (s != NULL) {
        free(s->name);
        free(s);
    }
}

// Fungsi untuk mencari student berdasarkan ID
void find_student(Student* students[], int count, int target_id) {
    for (int i = 0; i < count; i++) {
        if (students[i]->id == target_id) {
            printf("Found student: %d - %s (Score: %.2f)\n", 
                   students[i]->id, students[i]->name, students[i]->score);
            return;
        }
    }
    printf("Student with ID %d not found.\n", target_id);
}

// Fungsi yang menyebabkan multiple deallocation
void process_student_records(Student* students[], int count) {
    // Memproses data student
    for (int i = 0; i < count; i++) {
        printf("Processing student: %d - %s (Score: %.2f)\n", 
               students[i]->id, students[i]->name, students[i]->score);
        
        // Modifikasi data
        students[i]->score += 10.0;
    }
    
    // Mencari beberapa student
    find_student(students, count, 101);
    find_student(students, count, 105);
    
    // Bug: membebaskan student[0] dua kali
    // Pertama di sini
    free_student(students[0]);
    
    // Kemudian, membebaskan semua records
    for (int i = 0; i < count; i++) {
        // Bug: student[0] akan dibebaskan lagi di sini
        free_student(students[i]);
    }
}

int main() {
    const int NUM_STUDENTS = 5;
    Student* class_roster[5];
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi data
    class_roster[0] = create_student(101, "Alice", 85.5);
    class_roster[1] = create_student(102, "Bob", 78.0);
    class_roster[2] = create_student(103, "Charlie", 92.5);
    class_roster[3] = create_student(104, "Diana", 88.0);
    class_roster[4] = create_student(105, "Edward", 79.5);
    
    start = clock();
    
    // Proses data yang akan menyebabkan multiple deallocation
    process_student_records(class_roster, NUM_STUDENTS);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nWaktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}