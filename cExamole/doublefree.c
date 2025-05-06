//释放同一块内存两次
#include <malloc.h>

int main(){
    int *a = malloc(sizeof(int));
    free(a);
    free(a);
    return 1;
}