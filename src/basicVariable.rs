pub fn wrapping_test() {
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
}

pub fn float_test() {
    let x = 2.0;
    let y : f32 = 3.0;

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

//语句与表达式
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    x+y
}

pub fn expression_test(x : i32) -> i32 {
    let y = {
        let x = x + 3;
        x + 1
    };
    return y;
}

pub fn ownership() {
    let a = 'a';
    let a = 1;
}

//使用尽可能多的方法通过编译
pub fn ownership_test_one() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{}, {}",  x, y);
}

// 不要修改 main 中的代码
fn ownership_test_two() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String{
    println!("{}", s);
    s
}

fn ownership_test_three() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.into_bytes();
    if let Ok(val) = String::from_utf8(_s) {
        val
    }
    else {
        panic!();
    }
}

// 修复错误，不要删除任何代码行
pub fn ownership_test_four() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}

// 不要使用 clone，使用 copy 的方式替代
fn ownership_test_five() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


fn ownership_test_six() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world")
}


fn ownership_test_seven() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);      // 完成该行代码，不要修改其它行！
    
    *y = 4;
    
    assert_eq!(*x, 5);
}

fn ownership_test_eight() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);
}

fn ownership_test_nine() {
    let t = (String::from("hello"), String::from("world"));
 
    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;
 
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}