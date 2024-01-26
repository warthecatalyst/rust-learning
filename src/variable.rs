fn variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // 可以放在一起，两个
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e : i32
}

fn variable2() {
    const MAX_POINTS: u32 = 100_000;

    let (a,b,c,d,e);
    (a, b) = (1,2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct  {e, ..} = Struct {e:5};
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

//变量遮蔽
fn variable_shadowing() {
    let x = 5;
    // main函数的作用域范围内对之前的变量进行遮蔽
    let x = x+1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}