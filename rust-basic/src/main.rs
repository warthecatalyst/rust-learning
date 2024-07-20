struct Struct {
    e: i32
}

const N: u32 = 800;

fn main() {
    course1_9();
}

fn add (i : i32, j : i32) -> i32{
    i+j
}

fn course1_1 () {
    let x: i32 = 5; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}

fn course1_2 () {
    let mut x = 1;
    x += 2; 

    println!("x = {}", x); 
}

fn course1_3 () {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y); 
}

// 修复错误
fn course1_4() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> &'static str {
    let x = "hello";
    return x;
}


// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn course1_5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}


fn course1_6() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x; 
    // x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!"; 
}


// compiler warning: unused variable: `x`
#[allow(unused_variables)]
fn course1_7() {
    let x = 1; 
}

fn course1_8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

fn course1_9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3, 2]);
    println!("success");
}



