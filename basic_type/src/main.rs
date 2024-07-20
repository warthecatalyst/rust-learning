use num::complex::Complex;

fn main() {
    // let mut x : u8 = 255;
    // x = x.wrapping_add(20);
    // println!("x = {}", x);
    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
    // println!("success");
    // let x = 10.0;
    // let y : f32 = 20.0;
    // println!("x = {}, y = {}", x, y);
    // assert!(0.1 + 0.2 == 0.3);

    // let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // println!("abc (f32)");
    // println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    // println!("         0.3: {:x}", (abc.2).to_bits());
    // println!();

    // println!("xyz (f64)");
    // println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("         0.3: {:x}", (xyz.2).to_bits());
    // println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // // assert!(xyz.0 + xyz.1 == xyz.2);

    // let x = (-42.0_f32).sqrt();
    // if x.is_nan() {
    //     println!("未定义的数学行为")
    // }

    // for i in 1..=5 {
    //     println!("{}", i)
    // }

    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
    let result = a * b;
    println!("{:.2} + {:.2}i", result.re, result.im);

    course_2_17();
}

fn course_2_1() {
    let x: i32 = 5;
    let mut y = 5;
    y = x;
    
    let z = 10; // 这里 z 的类型是? i32
}

// 填空
fn course_2_2() {
    let v: u16 = 38_u8 as u16;
}


// 修改 `assert_eq!` 让代码工作
fn course_2_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("{}", type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


// 填空，让代码工作
fn course_2_4() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
}


// 解决代码中的错误和 `panic`
fn course_2_5() {
    let v1 = 211_u8 + 8;
    let v2 = u8::checked_add(211, 8).unwrap();
    println!("{},{}",v1,v2);
 }

 
// 修改 `assert!` 让代码工作
fn course_2_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}


// 将 ? 替换成你的答案
fn course_2_7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}


fn course_2_8() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!(0.1+0.2 != 0.3);
}

fn course_2_9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}


// 填空
use std::ops::{Range, RangeInclusive};
fn course_2_10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}


// 填空，并解决错误
fn course_2_11() {
    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    
    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);
    
    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// 修改2处 `assert_eq!` 让代码工作
use std::mem::size_of_val;
fn course_2_12() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!")
}

// 修改一行让代码正常打印
fn course_2_13() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}


// 使成功打印
fn course_2_14() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!")
    }
}


fn course_2_15() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!")
}


// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn course_2_16() {
    let v: () = ();

    let _v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
// 让代码工作：修改 `assert!` 中的 `4`
fn course_2_17() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);   //占用0内存

    println!("Success!")
}


