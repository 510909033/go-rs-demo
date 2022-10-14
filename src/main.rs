/*
导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体：
{:?}
{:#?}


*/

fn main() {
    println!("Hello, world!");
    println!("{}", demo());

//    another_function(5,6);
//    d括号复杂表达式();
//    gfor遍历数组();
//    gloop循环();
//    h错误示例();
//    h函数返回值所有权();
//    g错误的租借示例();
//    j字符串切片();
    k结构体();
}

fn demo() -> i32 {
    let a = 1;

    a
}

fn another_function(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn d括号复杂表达式() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}


fn d函数嵌套() {
    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());
}

// 条件语句  f
fn fif判断() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
}

fn f三元表达式() {
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}


// 循环g
/*
没有do while 循环
*/

fn gwhile循环() {
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
}

fn gfor遍历数组() {
    let a = [10, 20, 30, 40, 50];

    for i in a.iter() {
        println!("值为 : {}, {:p}", i, i); // {:p}
    }

    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}

fn gloop循环() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}

//loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。这是一个十分巧妙的设计，因为 loop 这样的循环常被用来当作查找工具使用，如果找到了某个东西当然要将这个结果交出去：
fn gloop循环和返回值() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
}

//所有权规则 h
/*
Rust 中的每个值都有一个变量，称为其所有者。
一次只能有一个所有者。
当所有者不在程序运行范围时，该值将被删除。

仅在栈中的数据的"移动"方式是直接复制
"基本数据"类型有这些：
所有整数类型，例如 i32 、 u32 、 i64 等。
布尔类型 bool，值为 true 或 false 。
所有浮点类型，f32 和 f64。
字符类型 char。
仅包含以上类型数据的元组（Tuples）。
*/

fn h错误示例() {
    let s1 = String::from("hello");
    let s2 = s1;
//    println!("{}, world!", s1); // 错误！s1 已经失效


    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}


fn h函数所有权() {
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
} // 函数结束, x 无效, 然后是 s. 但 s 已被移动, 所以不用被释放


fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

// 函数所有权结束

fn h函数返回值所有权() {
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权

//    println!("{}", s2);//错误 s2无法访问
} // s3 无效被释放, s2 被移动, s1 无效被释放.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string  // a_string 被当作返回值移出函数
}


//引用与租借
fn g引用与租借() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
}

fn g引用与租借_函数() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
引用不会获得值的所有权。
引用只能租借（Borrow）值的所有权。
引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：
*/

fn g错误的租借示例() {
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = s1;


    let s2 = s3;
    println!("{}", s2);
}

// 切片 j

fn j字符串切片() {
    let s = String::from("broadcast");

    //注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    let a = "boardcast";
    let c = &a[2..4];

    //有一个快速的办法可以将 String 转换成 &str：
    let mut s1 = String::from("hello");
    let s2 = &s1[..];

    println!("{}", s1);
    s1.push('t');
    println!("{}", s1);
}

//结构体 k
fn k结构体() {
    struct Site {
        domain: String,
        name: String,
        nation: String,
        found: u32,
//        str:&str
    }

    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
//        str:"haha",
    };

    //结构体更新语法：
    //新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，仅需更改其中的一两个字段的值
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };


    //元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    //结构体所有权


    //结构体方法
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        //结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());

    //结构体关联函数
    /*
    之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。
这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
一直使用的 String::from 函数就是一个"关联函数"。
    */
    impl Rectangle {
        fn demo() {
//            println!("{:?}", Rectangle::demo());
        }
    }

    let demo1 = Rectangle::demo();


    //单元结构体
    //结构体可以只作为一种象征而无需任何成员：

    struct UnitStruct;
    //我们称这种没有身体的结构体为单元结构体（Unit Struct）。
}







