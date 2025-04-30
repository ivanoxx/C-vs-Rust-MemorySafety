#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct TaskNode {
    int id;
    char description[100];
    int priority;
    struct TaskNode* next;
    struct TaskNode* dependency; // Task that must be completed before this one
} TaskNode;

TaskNode* createTask(int id, const char* description, int priority) {
    TaskNode* newTask = (TaskNode*)malloc(sizeof(TaskNode));
    if (newTask != NULL) {
        newTask->id = id;
        strncpy(newTask->description, description, 99);
        newTask->description[99] = '\0';
        newTask->priority = priority;
        newTask->next = NULL;
        newTask->dependency = NULL;
    }
    return newTask;
}

void addTask(TaskNode** head, TaskNode* newTask) {
    if (*head == NULL) {
        *head = newTask;
        return;
    }
    
    TaskNode* current = *head;
    while (current->next != NULL) {
        current = current->next;
    }
    current->next = newTask;
}

void setDependency(TaskNode* task, TaskNode* dependency) {
    if (task != NULL) {
        task->dependency = dependency;
    }
}

TaskNode* findTask(TaskNode* head, int id) {
    TaskNode* current = head;
    while (current != NULL) {
        if (current->id == id) {
            return current;
        }
        current = current->next;
    }
    return NULL; 
}

void printTaskDetails(TaskNode* task) {
    printf("Task ID: %d\n", task->id); // Will crash if task is NULL
    printf("Description: %s\n", task->description);
    printf("Priority: %d\n", task->priority);
    
    // Intentional null pointer dereference risk:
    printf("Dependency Task ID: %d\n", task->dependency->id); // Will crash if dependency is NULL
    printf("Dependency Description: %s\n", task->dependency->description);
}

void completeTask(TaskNode* head, int id) {
    TaskNode* task = findTask(head, id);
    
    // Unsafe: No NULL check before dereferencing
    printf("Completing task: %s (ID: %d)\n", task->description, task->id);
    
    if (task->dependency != NULL) {
        printf("Must first complete dependency: %s (ID: %d)\n", 
               task->dependency->description, 
               task->dependency->id);
    } else {
        // Another intentional null pointer dereference:
        printf("No dependency ID needed, value is: %d\n", task->dependency->id); // Will crash
    }
}

void freeTasks(TaskNode* head) {
    TaskNode* current = head;
    TaskNode* next;
    
    while (current != NULL) {
        next = current->next;
        free(current);
        current = next;
    }
}

int main() {
    clock_t start, end;
    double cpu_time_used;
    
    start = clock();
    
    TaskNode* taskList = NULL;
    
    TaskNode* task1 = createTask(1, "Complete project proposal", 5);
    TaskNode* task2 = createTask(2, "Research competitors", 3);
    TaskNode* task3 = createTask(3, "Prepare presentation", 4);
    TaskNode* task4 = NULL; // Deliberately NULL task
    
    addTask(&taskList, task1);
    addTask(&taskList, task2);
    addTask(&taskList, task3);
    
    setDependency(task3, task1); // Task 3 depends on Task 1
    setDependency(task1, task2); // Task 1 depends on Task 2
    
    // Print task details (safe)
    printf("Task List:\n");
    TaskNode* current = taskList;
    while (current != NULL) {
        printf("- ID: %d, Description: %s, Priority: %d\n", 
               current->id, current->description, current->priority);
        current = current->next;
    }
    
    printf("\nDetailed Task View:\n");
    
    // Intentional NULL pointer dereference 1:
    printf("\nTask 4 Details (NULL):\n");
    printTaskDetails(task4); // Will crash - task4 is NULL
    
    // Intentional NULL pointer dereference 2:
    printf("\nCompleting Task 2:\n");
    completeTask(taskList, 2); // Will crash when checking dependency->id which is NULL
    
    freeTasks(taskList);
    
    end = clock();
    cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("\nExecution time: %f seconds\n", cpu_time_used);
    
    return 0;
}