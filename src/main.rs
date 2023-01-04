mod  structure;
mod borrowing;
extern crate core;

use std::io;
use std::ptr::addr_of_mut;

fn main() {

    borrowing::borrowing();
    structure::structure();

}

fn variables() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    print!("{THREE_HOURS_IN_SECONDS}");


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}\n");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces : {spaces}");

    let guess: i32 = "-42".parse().expect("Not a number!");
    println!("{guess}");

    // guess

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

fn data_type() {
    /// scalar -----------------------------
    let x: f32 = 2.0;
    let y: f64 = 3.0;
    println!("{}, {}", x, y);

    // division
    let x = 5 / 2;      // 2
    let y = 5.0 / 2.0;  // 2.5
    println!("{}, {}", x, y);

    // remainder
    let x = 43 % 5;     // 3
    println!("{}", x);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_emo = '🥰';

    /// compound -----------------------------
    let tup_explicit: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{}", tup_explicit.0);     // 500
    println!("{}", tup.1);              // 6.4
    println!("{z}");                    // 1

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("{}", a[0]);
    println!("{}", months[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];      // [3, 3, 3, 3, 3]

    println!("{}", a[2]);       // 3
    println!("{}", b[3]);       // 3

    println!("============= array index ============");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn statements_expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let plus_one = plus_one(5);
    println!("plus one: {plus_one}\n");
}

// 반환값이 있는 함수
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow() {
    // if문
    println!("\n>> if");
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop
    println!("\n>> loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // 이중 반복문에서 내-외부 종료 시점 설정
    println!("\n>> 이중 반복문에서 내-외부 종료 시점 설정");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while
    println!("\n>> while");
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!");

    // for
    println!("\n>> for");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!\n")
}

fn ownership() {
    // 변수범위
    println!(">> 변수범위");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    /// 러스트에서 복제는 얕은 복사처럼 보이지만
    /// 소유권이 이전된 이후에 본래의 변수가 무효화 되기 때문에
    /// '이동'되었다고 보는 것이 맞다.
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // ERROR 발생!

    let s1 = String::from("hello");
    let s2 = s1.clone(); // 따라서 복사하고자 할 때는 clone을 이용해야 한다.

    println!("s1 = {}, s2 = {}", s1, s2);

    // 반면 정수의 경우는 다르다!
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    /// 정수는 이미 알려진 크기를 갖은 채 스택에 완전히 저장되기 때문에,
    /// 실제 값의 복사본을 빠르게 만들 수 있고,
    /// 변수를 생성한 후 유효하지 않도록 방지할 이유가 없다.

    let s = String::from("hello");  // 사용 범위 안에 들어 온다.

    takes_ownership(s);             // 해당 함수로 변수가 이동 하고 더 이상 이 곳 에서는 사용할 수 없다.

    let x = 5;                      // 사용 범위 안에 들어 온다.

    makes_copy(x);                  // 해당 함수로 변수가 이동 하지만, 자동으로 복사 되었기 때문에 이 곳 에서도 사용할 수 있다.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // 여기서 파라미터의 범위가 끝나고 'drop'이 호출 되며, 백업 메모리가 확보된다.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // 여기서 int 파라미터의 범위가 끝나지만, 특별히 바뀌는 것은 없다.
