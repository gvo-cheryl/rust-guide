pub fn structure() {
    // 구조체 인스턴스 생성
    // 모든 필드값을 채워야 한다.
    let user1 = User {
        email: String::from("it.progrowh@gmail.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };

    // 필드값을 변경할 수 있는 구조체
    let mut user2 = User {
        email: String::from("it.progrowh@gmail.com"),
        username: String::from("someuser"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("rust@gmail.com");
    println!("{}", user2.email);

    // 이메일과 사용자 이름을 받아 User인스턴스를 반환하는 함수
    build_user(String::from("email"), String::from("username"));

    // 인스턴스 업데이트 방법1
    let user_update1 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("update@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    // 인스턴스 업데이트 방법2
    let user_update2 = User {
        email: String::from("update2@gmail.com"),
        ..user2 // user1에서 가져오려고 하면 이미 사용된 값이라 컴파일에러가 난다
    };

    // 명명된 필드 없이 튜플 구조체를 사용하여 다른 유형 생성
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 필드가 없는 단위 유사 구조체
    // 일반적으로 일부 유형에 특성을 구현해야 하지만, 유형 자체에 저장하려는 데이터가 없을때 유용
    // 항상 다른 모든 유형의 모든 인스턴스와 동일하도록하고 테스트 목적으로 사용한다.
    // 자세한 내용은 10장에서 다시 살펴보자!
    let subject = AlwaysEqual;

    /// 구조체 데이터의 소유권
    // 앞에서 구조체를 사용할때 문자열 슬라이스가 아닌 String 소유 유형을 사용했다.
    // 이는 구조체의 각 인스턴스가 모든 데이터를 소유하고 전체 구조체가 유효한 동안 해당 데이터가 유효하도록 하기 위함이다.
    // 구조체가 참조를 저장하는 것도 가능하지만, 이ㅏ는 10장의 lifetimes를 사용해야 한다.
    // 따라서 아래와 같은 방법으로 참조를 저장하는 것은 에러가 난다.
    // struct  UserOwnership {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // an example program using structs
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));

    // refactoring with tuples
    let rectangle = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_rectangle(rectangle));

    // refactoring with structs: adding more meaning
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_rect(&rect));

    // adding useful functionality with derived traits
    println!("rect is {:#?}", rect);
    println!("rect is {:?}", rect);

    // debug format : dbg! 매크로
    let a = 2;
    let b = dbg!(a * 2) + 1;

    /// A wrapper around `usize` which importantly is not Copyable.
    #[derive(Debug)]
    struct NoCopy(usize);

    let a = NoCopy(42);
    let _ = dbg!(a); // <-- `a` is moved here.
    // let _ = dbg!(a); // <-- `a` is moved again; error!

    let scale = 2;
    let  rect2 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(&rect2);

    method();
}

fn  method(){
    /// 메서드
    /// - 함수와 유사하며, fn 키워드와 이름으로 메서드를 선언하고 매개 변수와 반환 값을 가질 수 있으며
    ///   메서드가 다른 곳에서 호출될 때 실행되는 이부 코드를 포함한다.
    /// - 함수와 달리 메서드는 구조체의 컨텍스트 내에서 정의되며,
    ///   첫번째 매개변수는 항상 self 메서드가 속한 구조체의 인스턴스를 나타내며 호출된다.
    ///
    /// 1. 방법 정의
    let  rect3 = Rectangle {
        width: 30,
        height:50,
    };
    println!("The area of the rectangle is {} square pixels.", rect3.area_rect());
    if rect3.width() {
        println!("The rectangle has a nonzero width; it is {}", rect3.width);
    }

    // 2. 매개변수가 더 많은 방법
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 정사각형 사이즈의 인스턴스 만들기
    let  square = dbg!(Rectangle::square(30));
}

impl Rectangle {
    fn area_rect(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width >0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width * self.height > rect.width * rect.height
        // self.width > other.width && self.height > other.height
    }

    // 정사각형 사이즈의 인스턴스 만들기
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area_rectangle(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 구조체 정의
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // email: email 과 동일
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;