fn main() {
    let r = 1;

    {
        let x = 5;  // `x` 在这一作用域内有效
        c = &x;     // `r` 引用 `x`，但是 `x` 的生命周期仅在这个作用域内
    }

    println!("r: {}", r);  
}