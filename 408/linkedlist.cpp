#include <stdio.h>
// 2019 真题
typedef struct node {
    int data;
    struct node* next;
} NODE;

struct node* list_pop(struct node* head) {
    struct node* node = head;
    struct node* prev = NULL;
    
    while(node->next != NULL) {
        prev = node;
        node = node->next;
    }
    prev->next = NULL;
    return node;
}

size_t list_len(struct node* head) {
    struct node* node = head;
    size_t len = 1;
    while(node->next != NULL) {
        node = node->next;
        len++;
    }
    return len;
}

void shuffle_list(struct node* head) {
    size_t len = list_len(head) / 2;
    struct node* node = head;
    size_t i = 0;
    while(i < len) {
        struct node* tail = list_pop(head);
        struct node* next = node->next;
        tail->next = next;
        node->next = tail;
        node = next;
        i++;
    }
}

void print_list(struct node* head) {
    struct node* node = head;
    while(node->next != NULL) {
        printf("%d ", node->data);
        node = node->next;
    }
    printf("%d ", node->data);
    printf("\n");
}

int main() {
    struct node node_1;
    struct node node_2;
    struct node node_3;
    struct node node_4;    
    node_1.data = 1;
    node_2.data = 2;
    node_3.data = 3;
    node_4.data = 4;
    node_1.next = &node_2;
    node_2.next = &node_3;
    node_3.next = &node_4;
    node_4.next = NULL;
    print_list(&node_1);
    shuffle_list(&node_1);
    print_list(&node_1);
}