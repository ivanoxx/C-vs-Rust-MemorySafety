#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Struktur Node untuk Extended Double Linked List
typedef struct DLLNode {
    int data;
    struct DLLNode* prev;
    struct DLLNode* next;
} DLLNode;

// Struktur untuk mengelola Extended Double Linked List
typedef struct {
    DLLNode* head;
    DLLNode* tail;
    int size;
} DoublyLinkedList;

// Fungsi untuk inisialisasi list
DoublyLinkedList* createList() {
    DoublyLinkedList* list = (DoublyLinkedList*)malloc(sizeof(DoublyLinkedList));
    if (list == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }
    list->head = NULL;
    list->tail = NULL;
    list->size = 0;
    return list;
}

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

// Fungsi untuk menambah node di awal
void insertAtBeginning(DoublyLinkedList* list, int data) {
    DLLNode* newNode = createNode(data);
    
    if (list->head == NULL) {
        list->head = newNode;
        list->tail = newNode;
    } else {
        newNode->next = list->head;
        list->head->prev = newNode;
        list->head = newNode;
    }
    
    list->size++;
}

// Fungsi untuk menambah node di akhir
void insertAtEnd(DoublyLinkedList* list, int data) {
    DLLNode* newNode = createNode(data);
    
    if (list->tail == NULL) {
        list->head = newNode;
        list->tail = newNode;
    } else {
        newNode->prev = list->tail;
        list->tail->next = newNode;
        list->tail = newNode;
    }
    
    list->size++;
}

// Fungsi untuk menambah node di posisi tertentu
void insertAtPosition(DoublyLinkedList* list, int data, int position) {
    if (position < 0 || position > list->size) {
        printf("Invalid position\n");
        return;
    }
    
    if (position == 0) {
        insertAtBeginning(list, data);
        return;
    }
    
    if (position == list->size) {
        insertAtEnd(list, data);
        return;
    }
    
    DLLNode* newNode = createNode(data);
    DLLNode* current = list->head;
    
    for (int i = 0; i < position - 1; i++) {
        current = current->next;
    }
    
    newNode->next = current->next;
    newNode->prev = current;
    current->next->prev = newNode;
    current->next = newNode;
    
    list->size++;
}

// Fungsi untuk mencari node
DLLNode* search(DoublyLinkedList* list, int key) {
    DLLNode* current = list->head;
    
    while (current != NULL) {
        if (current->data == key) {
            return current;
        }
        current = current->next;
    }
    
    return NULL;
}

// Fungsi untuk menghapus node di awal
void deleteAtBeginning(DoublyLinkedList* list) {
    if (list->head == NULL) {
        return;
    }
    
    DLLNode* temp = list->head;
    
    if (list->head == list->tail) {
        list->head = NULL;
        list->tail = NULL;
    } else {
        list->head = list->head->next;
        list->head->prev = NULL;
    }
    
    free(temp);
    list->size--;
}

// Fungsi untuk menghapus node di akhir
void deleteAtEnd(DoublyLinkedList* list) {
    if (list->tail == NULL) {
        return;
    }
    
    DLLNode* temp = list->tail;
    
    if (list->head == list->tail) {
        list->head = NULL;
        list->tail = NULL;
    } else {
        list->tail = list->tail->prev;
        list->tail->next = NULL;
    }
    
    free(temp);
    list->size--;
}

// Fungsi untuk menghapus node berdasarkan key
int deleteNode(DoublyLinkedList* list, int key) {
    if (list->head == NULL) {
        return 0;
    }
    
    // Jika node yang dihapus adalah head
    if (list->head->data == key) {
        deleteAtBeginning(list);
        return 1;
    }
    
    // Jika node yang dihapus adalah tail
    if (list->tail->data == key) {
        deleteAtEnd(list);
        return 1;
    }
    
    // Mencari node di tengah
    DLLNode* current = list->head->next;
    
    while (current != list->tail) {
        if (current->data == key) {
            current->prev->next = current->next;
            current->next->prev = current->prev;
            free(current);
            list->size--;
            return 1;
        }
        current = current->next;
    }
    
    return 0; // Key tidak ditemukan
}

// Fungsi untuk memutar list (reverse)
void reverseList(DoublyLinkedList* list) {
    if (list->head == NULL || list->head == list->tail) {
        return;
    }
    
    DLLNode* current = list->head;
    DLLNode* temp = NULL;
    
    // Menukar prev dan next untuk semua node
    while (current != NULL) {
        temp = current->prev;
        current->prev = current->next;
        current->next = temp;
        current = current->prev;
    }
    
    // Menukar head dan tail
    temp = list->head;
    list->head = list->tail;
    list->tail = temp;
}

// Fungsi untuk menampilkan list (forward)
void displayForward(DoublyLinkedList* list) {
    if (list->head == NULL) {
        printf("List is empty\n");
        return;
    }
    
    DLLNode* current = list->head;
    printf("List (forward): ");
    
    while (current != NULL) {
        printf("%d ", current->data);
        current = current->next;
    }
    
    printf("\n");
}

// Fungsi untuk menampilkan list (backward)
void displayBackward(DoublyLinkedList* list) {
    if (list->tail == NULL) {
        printf("List is empty\n");
        return;
    }
    
    DLLNode* current = list->tail;
    printf("List (backward): ");
    
    while (current != NULL) {
        printf("%d ", current->data);
        current = current->prev;
    }
    
    printf("\n");
}

// Fungsi untuk membersihkan list
void freeList(DoublyLinkedList* list) {
    DLLNode* current = list->head;
    DLLNode* next;
    
    while (current != NULL) {
        next = current->next;
        free(current);
        current = next;
    }
    
    free(list);
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi double linked list
    DoublyLinkedList* list = createList();
    
    start = clock();
    
    // Operasi pada double linked list
    // Menambahkan node di awal dan akhir
    for (int i = 0; i < 400; i++) {
        insertAtBeginning(list, i);
    }
    
    for (int i = 400; i < 800; i++) {
        insertAtEnd(list, i);
    }
    
    // Menambahkan node di posisi tertentu
    for (int i = 800; i < 1000; i++) {
        int position = i % list->size;
        insertAtPosition(list, i, position);
    }
    
    // Mencari beberapa nilai
    for (int i = 0; i < 100; i++) {
        int key = (i * 17) % 1000;
        DLLNode* result = search(list, key);
        if (result != NULL) {
            // printf("Found %d\n", key);
        } else {
            // printf("%d not found\n", key);
        }
    }
    
    // Menghapus beberapa node
    for (int i = 0; i < 50; i++) {
        deleteAtBeginning(list);
    }
    
    for (int i = 0; i < 50; i++) {
        deleteAtEnd(list);
    }
    
    for (int i = 0; i < 100; i++) {
        int key = (i * 19) % 1000;
        deleteNode(list, key);
    }
    
    // Memutar list
    reverseList(list);
    
    // Menampilkan list (opsional, dimatikan untuk pengukuran performa)
    // displayForward(list);
    // displayBackward(list);
    
    // Membersihkan memori
    freeList(list);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Waktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}