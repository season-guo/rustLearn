//编译器知道在最后一次println!(b)后不再用b所以可以编译
//调换前面两个println会编译失败
//实际原因是println! 用的是非mut引用(否则的话可想而知打印不了imutable)
//所以如果把a丢进去就同时产生了mut 和 imut 引用导致错误
//所以看到后面让 b 为 imut 就没问题
fn main(){
    let mut a = String::from("Melin");
    let  b = &mut a;
    println!("{}", b);
    println!("{}", a);

    let b = &a;
    println!("{}", a);
    println!("{}", b);
}