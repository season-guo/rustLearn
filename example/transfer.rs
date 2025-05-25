//字符串改变ownership导致a无了
fn main(){
    let a : String = "Hell".to_string();
    let b = a;
    //let b = a.clone();
    println!("{}", b);
    println!("{}", a);
}