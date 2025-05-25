//链表结构体
//像下面这种定义是有问题的, 因为 rust 不知道你借用的 next 的生命周期 是否和整个结构体的生命周期一致,这样就会出现结构体字段无效的情况
//(当然你可以手动写生命周期注释,不过会很麻烦且难以管理)
/*
struct Node {
    val : i32,
    next : &mut Node,
}
*/
//但是用 Box就行, 因为Box 可以理解为是一个 指针 本身, 其掌控了自己的所有权, 所以不会受本体的干扰
//同时由于rust 的 空 要用 Option, 所以还要再包装一下
struct Node {
    val : i32,
    next : Option<Box<Node>>,
}

struct  LinkedList {
    head : Option<Box<Node>>,
    length : u32,
}

impl Node {
    fn new(val : i32, next : Option<Box<Node>>) -> Option<Box<Node>>{
        Some(Box::new(Node{val : val, next : next}))
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList{head : Node :: new(0, None), length : 0}
    }
}

fn main(){
    let mut list = LinkedList :: new();
    assert_eq!(list.length, 0);
}

