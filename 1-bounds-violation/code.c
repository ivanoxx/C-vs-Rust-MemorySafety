#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

#define MAX_STUDENTS 5
#define MAX_NAME_LENGTH 20

typedef struct {
    int id;
    char name[MAX_NAME_LENGTH];
    float scores[3];  // Menyimpan 3 nilai ujian
    float average;
} Student;

// Menghitung nilai rata-rata untuk semua mahasiswa
void calculateAverages(Student students[], int count) {
    for (int i = 0; i <= count; i++) {  // Bug: <= bukannya 
        float sum = 0;
        for (int j = 0; j < 3; j++) {
            sum += students[i].scores[j];
        }
        students[i].average = sum / 3;
    }
}

// Menampilkan data mahasiswa terurut berdasarkan nilai rata-rata
void displaySortedStudents(Student students[], int count) {
    // Array untuk menyimpan indeks terurut
    int indices[MAX_STUDENTS];
    
    // Inisialisasi array indeks
    for (int i = 0; i < MAX_STUDENTS; i++) {
        indices[i] = i;
    }
    
    // Pengurutan sederhana berdasarkan nilai rata-rata
    for (int i = 0; i < count - 1; i++) {
        for (int j = 0; j < count - i - 1; j++) {
            if (students[indices[j]].average < students[indices[j + 1]].average) {
                int temp = indices[j];
                indices[j] = indices[j + 1];
                indices[j + 1] = temp;
            }
        }
    }
    
    // Menampilkan data yang sudah terurut
    printf("\nDaftar Mahasiswa (Terurut berdasarkan nilai):\n");
    printf("ID\tNama\t\tNilai Rata-rata\n");
    
    for (int i = 0; i < count + 1; i++) {  // Bug: count + 1 akan melewati batas
        int idx = indices[i];
        printf("%d\t%s\t\t%.2f\n", 
               students[idx].id, 
               students[idx].name, 
               students[idx].average);
    }
}

// Mencari mahasiswa berdasarkan ID
void findStudentById(Student students[], int count, int searchId) {
    for (int i = 0; i <= count; i++) {  // Bug: <= bukannya 
        if (students[i].id == searchId) {
            printf("Mahasiswa ditemukan:\n");
            printf("Nama: %s\n", students[i].name);
            printf("Nilai rata-rata: %.2f\n", students[i].average);
            return;
        }
    }
    printf("Mahasiswa dengan ID %d tidak ditemukan.\n", searchId);
}

int main() {
    Student students[MAX_STUDENTS];
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi data mahasiswa
    students[0] = (Student){101, "Ali", {85.5, 90.0, 82.5}, 0};
    students[1] = (Student){102, "Budi", {75.0, 80.5, 85.0}, 0};
    students[2] = (Student){103, "Cindy", {95.0, 92.5, 88.0}, 0};
    students[3] = (Student){104, "Dodi", {70.0, 65.5, 75.0}, 0};
    students[4] = (Student){105, "Eka", {88.0, 84.5, 90.0}, 0};
    
    start = clock();
    
    // Menghitung nilai rata-rata (berpotensi bounds violation)
    calculateAverages(students, MAX_STUDENTS);
    
    // Menampilkan mahasiswa terurut (berpotensi bounds violation)
    displaySortedStudents(students, MAX_STUDENTS);
    
    // Mencari mahasiswa dengan ID 103
    findStudentById(students, MAX_STUDENTS, 103);
    
    // Mencari mahasiswa dengan ID yang tidak ada
    findStudentById(students, MAX_STUDENTS, 110);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nWaktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}