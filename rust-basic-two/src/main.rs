//use core::panicking::panic;

fn main() {
    course_3_1();
    course_3_2();
    course_3_3();
    course_3_4();
    course_3_5();
    course_3_8();
    println!("success");
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

// 使用两种方法让代码工作起来
fn course_3_1() {
    let v = {
        let x = 1;
        x + 2
    };
 
    assert_eq!(v, 3);

    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
 }

 
fn course_3_2() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
 }

 
fn course_3_3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}


fn course_3_4() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = add(x, y);

    assert_eq!(s, 3);
}

fn add(x : i32, y: i32) -> i32 {
    x + y
}

fn course_3_5() {
    print();
 }
 
 // 使用另一个类型来替代 i32
 fn print() -> () {
    println!("hello,world");
 }

 // 用两种方法求解
fn course_3_6() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop {

    };
    panic!("abcd");
}


fn course_3_7() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    panic!("abcd");
    loop {

    };
    todo!();
}


fn course_3_8() {
    // 填空
    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}