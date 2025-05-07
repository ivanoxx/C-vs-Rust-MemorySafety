#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Struktur Node untuk Binary Search Tree
typedef struct BSTNode {
    int data;
    struct BSTNode* left;
    struct BSTNode* right;
} BSTNode;

// Fungsi untuk membuat node baru
BSTNode* createNode(int data) {
    BSTNode* newNode = (BSTNode*)malloc(sizeof(BSTNode));
    if (newNode == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }
    newNode->data = data;
    newNode->left = NULL;
    newNode->right = NULL;
    return newNode;
}

// Fungsi untuk menyisipkan node ke dalam BST
BSTNode* insert(BSTNode* root, int data) {
    if (root == NULL) {
        return createNode(data);
    }
    
    if (data < root->data) {
        root->left = insert(root->left, data);
    } else if (data > root->data) {
        root->right = insert(root->right, data);
    }
    
    return root;
}

// Fungsi untuk mencari nilai dalam BST
BSTNode* search(BSTNode* root, int key) {
    if (root == NULL || root->data == key) {
        return root;
    }
    
    if (key < root->data) {
        return search(root->left, key);
    }
    
    return search(root->right, key);
}

// Fungsi untuk membersihkan BST
void freeBST(BSTNode* root) {
    if (root != NULL) {
        freeBST(root->left);
        freeBST(root->right);
        free(root);
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    // Inisialisasi BST
    BSTNode* root = NULL;
    
    start = clock();
    
    // Operasi pada BST
    // Menambahkan node (misal 1000 node)
    for (int i = 0; i < 1000; i++) {
        int value = rand() % 10000; // Nilai acak untuk menghindari BST yang tidak seimbang
        root = insert(root, value);
    }
    
    // Mencari beberapa nilai
    for (int i = 0; i < 100; i++) {
        int key = rand() % 10000;
        BSTNode* result = search(root, key);
        if (result != NULL) {
            printf("Found %d\n", key);
        } else {
            printf("%d not found\n", key);
        }
    }
    
    freeBST(root);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Waktu eksekusi: %f detik\n", cpu_time_used);
    
    return 0;
}