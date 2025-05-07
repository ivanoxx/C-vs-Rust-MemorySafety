#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Struktur Node untuk Double Linked List
typedef struct DLLNode {
    int data;
    struct DLLNode* prev;
    struct DLLNode* next;
} DLLNode;

// Fungsi untuk membuat node baru
DLLNode* createNode(int data) {
    DLLNode* newNode = (DLLNode*)malloc(sizeof(DLLNode));
    if (newNode == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }
    newNode->data = data;
    newNode->prev = NULL;
    newNode->next = NULL;
    return newNode;
}

// Fungsi untuk menambah node di awal double linked list
DLLNode* insertAtBeginning(DLLNode* head, int data) {
    DLLNode* newNode = createNode(data);
    newNode->next = head;
    
    if (head != NULL) {
        head->prev = newNode;
    }
    
    return newNode;
}

// Fungsi untuk menambah node di akhir double linked list
DLLNode* insertAtEnd(DLLNode* head, int data) {
    DLLNode* newNode = createNode(data);
    
    if (head == NULL) {
        return newNode;
    }
    
    DLLNode* current = head;
    while (current->next != NULL) {
        current = current->next;
    }
    
    current->next = newNode;
    newNode->prev = current;
    
    return head;
}

// Fungsi untuk mencari node dalam double linked list
DLLNode* search(DLLNode* head, int key) {
    DLLNode* current = head;
    while (current != NULL) {
        if (current->data == key) {
            return current;
        }
        current = current->next;
    }
    return NULL;
}

// Fungsi untuk menghapus node dari double linked list
DLLNode* deleteNode(DLLNode* head, int key) {
    if (head == NULL) {
        return NULL;
    }
    
    // Jika node yang dihapus adalah head
    if (head->data == key) {
        DLLNode* temp = head;
        head = head->next;
        
        if (head != NULL) {
            head->prev = NULL;
        }
        
        free(temp);
        return head;
    }
    
    DLLNode* current = head;
    while (current != NULL && current->data != key) {
        current = current->next;
    }
    
    if (current == NULL) {
        return head; // Key tidak ditemukan
    }
    
    // Update prev node
    if (current->prev != NULL) {
        current->prev->next = current->next;
    }
    
    // Update next node
    if (current->next != NULL) {
        current->next->prev = current->prev;
    }
    
    free(current);
    return head;
}

// Fungsi untuk menghapus double linked list
void freeList(DLLNode* head) {
    DLLNode* temp;
    while (head != NULL) {
        temp = head;
        head = head->next;
        free(temp);
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi double linked list
    DLLNode* head = NULL;
    
    start = clock();
    
    // Operasi pada double linked list
    // Menambahkan node di awal (misal 500 node)
    for (int i = 0; i < 500; i++) {
        head = insertAtBeginning(head, i);
    }
    
    // Menambahkan node di akhir (misal 500 node)
    for (int i = 500; i < 1000; i++) {
        head = insertAtEnd(head, i);
    }
    
    // Mencari beberapa nilai
    for (int i = 0; i < 100; i++) {
        int key = rand() % 1000;
        DLLNode* result = search(head, key);
        if (result != NULL) {
            printf("Found %d\n", key);
        } else {
            printf("%d not found\n", key);
        }
    }
    
    // Menghapus beberapa node
    for (int i = 0; i < 100; i++) {
        int key = rand() % 1000;
        head = deleteNode(head, key);
    }
    
    freeList(head);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Waktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}