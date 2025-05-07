#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Struktur Node untuk Linked List
typedef struct Node {
    int data;
    struct Node* next;
} Node;

// Fungsi untuk membuat node baru
Node* createNode(int data) {
    Node* newNode = (Node*)malloc(sizeof(Node));
    if (newNode == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }
    newNode->data = data;
    newNode->next = NULL;
    return newNode;
}

// Fungsi untuk menambah node di awal linked list
Node* insertAtBeginning(Node* head, int data) {
    Node* newNode = createNode(data);
    newNode->next = head;
    return newNode;
}

// Fungsi untuk mencari node dalam linked list
Node* search(Node* head, int key) {
    Node* current = head;
    while (current != NULL) {
        if (current->data == key) {
            return current;
        }
        current = current->next;
    }
    return NULL;
}

// Fungsi untuk menghapus linked list
void freeList(Node* head) {
    Node* temp;
    while (head != NULL) {
        temp = head;
        head = head->next;
        free(temp);
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi linked list
    Node* head = NULL;
    
    start = clock();
    
    // Operasi pada linked list
    // Menambahkan node (misal 1000 node)
    for (int i = 0; i < 1000; i++) {
        head = insertAtBeginning(head, i);
    }
    
    // Mencari beberapa nilai
    for (int i = 0; i < 100; i++) {
        int key = rand() % 1000;
        Node* result = search(head, key);
        if (result != NULL) {
            printf("Found %d\n", key);
        } else {
            printf("%d not found\n", key);
        }
    }
    
    // Membersihkan memori
    freeList(head);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Waktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}