//重新分配内存导致的指针悬空
#include <vector>
#include <iostream>
#include <new>
using namespace std;
int main(){
    vector<int>* v = new vector<int>(2) ;
    (*v)[0] = 1;
    (*v)[1] = 2;
    cout << "capacity: " << v -> capacity() << endl;
    cout << "address of v " << v -> data() << endl;
    int* s = v -> data(); 
    cout << "address of s " << s << endl;
    v -> push_back(1);
    cout << "address of v " << v -> data() << endl;
    cout << "capacity: " << v -> capacity() << endl;
    cout << s[1] << endl;
    return 1;
}