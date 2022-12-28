
pub fn borrowing(){
    references_borrowing();
    string_slice();
}

fn references_borrowing() {
    // immutable reference
    let s = String::from("hello");
    change(&s);
    // mutable reference
    let mut s = String::from("hello");
    let len = change_mut(&mut s);
    println!("s.len : {s}");
    println!("s.len : {len}");

    // restriction of mutable reference : 변경 가능한 참조는 1번만 가능하다.
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let  r2 = &mut s;  // error: second mutable borrow occurs here
    // println!("{}, {}", r1, r2);

    let mut ss = String::from("hello");
    {
        let r1 = &mut ss;
    } // r1의 사용범위는 여기서 끝나기 때문에 새로운 참조를 하는데 지장이 없다.

    let r2 = &mut ss;
    println!("{}, {}", r1, r2);

    // 불변 참조가 있는 동안에는 가변 참조를 가질 수 없다.
    let mut sss = String::from("가변참조");

    let r1 = &sss; // no prob
    let r2 = &sss; // no prob
    let r3 = &mut sss; // big prob

    // 하지만 불변참조가 사용되고 난 후에는 가변 참조가 가능하다.
    let mut ssss = String::from("가변참조");

    let r1 = &ssss; // no prob
    let r2 = &ssss; // no prob
    println!("{r1}, {r2}"); // 불변참조의 볌위는 마지막으로 사용된 위치에서 끝난다.
    let r3 = &mut ssss; // 따라서 여기는 문제가 되지 않는다.
    println!("r3 가변참조 : {r3}");

    // dangling references
    // dangling 참조가 되지 않도록 조심해야 한다.
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("dangle");
    // &s // s는 이 함수에서 생성된 것이므로 이 함수가 끝나면 값이 해제되는데 해제되는 값의 참조를 반환하기 때문에 문제가 된다.
    s // 따라서 여기서는 &String 이 아닌 해당 값 자체로 반환 해야 한다.
}

fn change(some_string: &String) {
    let len = some_string.len();
    println!("s.len : {len}");
    // some_string.push_str(", world"); // compile error : Cannot borrow immutable local variable `some_string` as mutable
}

fn change_mut(some_string: &mut String) -> usize {
    some_string.push_str(", world");
    some_string.len()
}

fn string_slice() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("frist word : {}", word);

    let hello = &s[0..5];
    println!("&s[0..5] : {}", hello);
    let hello = &s[0..=4];
    println!("&s[0..=4] : {}", hello);
    let hello = &s[..5];
    println!("&s[..5] : {}", hello);
    let world = &s[6..];
    println!("&s[6..] : {}", world);

    let  str_return = first_word_string(&s); // &str String값으로 반환
    // println!("return : {}", str_return);
    s.clear();

    // other slice
    let  a = [1,2,3,4,5];
    let  slice = &a[1..3];
    assert_eq!(slice, &[2,3]);

    // ownership, borrowing, slice의 개념은 Rust 프로그램이 컴파일 시점에서 메모리 안전을 보장한다.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // b' ' : 공백 byte값 32
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn  first_word_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}