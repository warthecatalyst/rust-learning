fn main() {
    course_4_1();
    course_4_2();
    course_4_3();    
    course_4_4();    
    course_4_5();
    course_4_6();
    course_4_7();
    course_4_8();
    course_4_9();
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}


fn course_4_1() {
    // 使用尽可能多的方法来通过编译
    let x = "hello, world";
    let y = x;
    println!("{},{}",x , y);
}

// 不要修改 main 中的代码
fn course_4_2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


fn course_4_3() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.clone().into_bytes();
    s
}

// 修复错误，不要删除任何代码行
fn course_4_4() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}

// 不要使用 clone，使用 copy 的方式替代
fn course_4_5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


fn course_4_6() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world");
    println!("{}", s1);
}


fn course_4_7() {
    let x = Box::new(5);

    let mut y = x.clone();      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}



fn course_4_8() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);
}


fn course_4_9() {
    let t = (String::from("hello"), String::from("world"));
 
    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;
 
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }
