//buffer超出函数stack的data测试，不知道为什么没看到最好的结果
#include <stdio.h>

int main(){
    char a[1];
    gets(a);
    printf("buf = %s\n", a);
    return 0;
}