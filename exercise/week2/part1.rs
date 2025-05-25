//Example 1:
//不可用引用存在时不可以直接改变本体,尽管s的位置不变只是改了指向的堆内存
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = & ref2;

    //s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}



//Example 2:
//返回栈上指向string的指针的地址的话,那这个s的生命周期在函数内就爆了,那这个地址还有啥用捏
fn drip_drop() -> /*&String*/ String {
    let s = String::from("hello world!");
    //return &s;
    s
}



//Example 3:
//和上面差不多, s2 这样直接把v的元素的所有权让出去了, vec肯定不会让你这样的捏, 所以借用就行了
fn main1() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2 : /*&*/String = /*&*/v[0];
    println!("{}", s2);
}