//函数参数夺取a的ownership
//可以通过以下几种方法解决
//1.用&接用
//2.返回值返回去
//3.用clone拷贝
//fn test(s : &String){
//fn test(s : String) -> String {
fn test(s : String){
    println!("{}", s);
    //s
}

fn main(){
    let a : String = "Melin".to_string();
    test(a);
    //let a = test(a);
    //test(&a);
    //test(a.clone())
    println!("{}", a);
}