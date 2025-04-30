#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef struct {
    double length;
    double width;
    double* area;     
    double* perimeter; 
} Rectangle;

void calculateArea(Rectangle* r) {
    // Allocate memory for area, but forget to initialize
    r->area = (double*)malloc(sizeof(double));
    *(r->area) = r->length * r->width;
}

void printInfo(Rectangle* r) {
    printf("Rectangle: %.2f x %.2f\n", r->length, r->width);
    printf("Area: %.2f\n", *(r->area));    // Safe if calculateArea was called
    printf("Perimeter: %.2f\n", *(r->perimeter)); // Uninitialized pointer dereference
}

Rectangle* createRectangle(double length, double width) {
    Rectangle* r = (Rectangle*)malloc(sizeof(Rectangle));
    r->length = length;
    r->width = width;
    // r->area and r->perimeter are not initialized
    return r;
}

void freeRectangle(Rectangle* r) {
    if (r->area != NULL) {
        free(r->area);
    }
    // Not checking r->perimeter because it might not be initialized
    // This can cause problems if freeing an uninitialized pointer
    if (r->perimeter != NULL) {  // Potential uninitialized pointer dereference
        free(r->perimeter);
    }
    free(r);
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    start = clock();
    
    Rectangle* rect1 = createRectangle(5.0, 3.0);
    Rectangle* rect2 = createRectangle(7.5, 2.5);
    
    calculateArea(rect1);
    
    printf("Rectangle 1 Information:\n");
    printInfo(rect1);  // Uninitialized pointer dereference for perimeter
    
    printf("\nRectangle 2 Information:\n");
    printInfo(rect2);  // Uninitialized pointer dereference for both area and perimeter
    
    // Free memory
    freeRectangle(rect1);
    freeRectangle(rect2);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nExecution time: %f seconds\n", cpu_time_used);
    
    return 0;
}