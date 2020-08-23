fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // println!("tup: {}", tup);

    let (x, y, z) = tup;

    println!("x y z: {} {} {}", x, y, z);
    println!("tup.0 tup.1 tup.2: {} {} {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("first second: {} {}", first, second);
    // arr[5]; // panic

    // function
    another_function(5);

    let x = 5;

    // 無名関数的な？
    let y = {
        let x = 3;
        x + 1
        // x + 1; // セミコロンつけると文になって値返さない
    };

    println!("{} {}", x, y);

    println!("five: {}", five());

    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // if number {} // ifはboolのみ

    loop {
        println!("in loop");
        break;
    }

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number = number - 1;
        // let number = number - 1; // while自体はシャーディングする前のnumber見てる
    }

    let arr2 = [10, 20, 30, 40, 50];

    for element in arr2.iter() {
        println!("{}", element);
    }

    {
        let mut s = String::from("hello");
        s.push_str(", world");

        println!("{}", s);
    } // この時点でsのスコープは終わり、メモリ返還される(drop関数call)

    let s1 = String::from("hello");
    let s2 = s1; // ポインタ参照コピー // ムーブされたと判定し、s1が無効になる
    let s3 = s2.clone(); // deep copy // s2有効

    // println!("{}, world!", s1);
    println!("{}, world!", s2);
    println!("{}, world!", s3);

    takes_ownership(s3);
    // println!("{}, world!", s3); // takes_ownershipの引数としてs3がムーブされたのでここではs3はdropずみ

    let s4 = String::from("hello");
    let len4 = calculate_length(&s4);

    println!("s4: {} {}", s4, len4);

    let mut s5 = String::from("hello");
    change_string(&mut s5);
    println!("s5: {}", s5);

    let r1_s5 = &mut s5;
    let r2_s5 = &mut s5;

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.sign_in_count = 1;

    // 変数名同名
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    let user3 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    // 構造体展開
    let user4 = User {
        username: String::from("someusername456"),
        ..user3
    };

    let black = Color(0, 0, 0);

    println!("The area of rectangle is {} square pixels.", area((30, 50)));
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels.", area2(&rect2));

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    println!("The area of rectangle is {} square pixels.", rect2.area());

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    // Option enuのSomeとNone
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 8;
    let y: Option<i8> = Some(5);
    // yはNoneの可能性もあるのでOptionの場合はちゃんと判定入れる
    // let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("default");
    }
}

fn another_function(x: i32) {
    println!("Another function {}", x);
}

fn five() -> i32 {
    5
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     // スコープ外にデータ持ち出せない
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

struct User {
    username: String, // &strだと参照先のデータの生存を意識しないといけない（ライフタイムなる機能使えばいい感じにできるっぽい
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// デバッグ出力できるようにするための注釈
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    // enumとmatchアーム
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
