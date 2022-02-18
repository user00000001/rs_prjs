#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct AnotherUser<'a > {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
    let mut user2 = user1.clone();
    user2.email = String::from("another@example.com");
    println!("{:?}", user2);
    println!("{:?}", build_user(String::from("anotheremail@example.com"), String::from("someusername234")));
    let another_user1 = AnotherUser {
        email: "a@a.com",
        username: "a",
        active: true,
        sign_in_count: 1,
    };
    // println!("{:?}", User{username: String::from("someusername345"), email: String::from(""), ..another_user1});
    println!("{:?}", User{username: String::from("someusername345"), email: String::from(""), active: another_user1.active, sign_in_count: another_user1.sign_in_count});
    println!("{:?}", (User{username: String::from("someusername345"), email: String::from(""), ..user2}, another_user1));

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    #[derive(Debug)]
    struct Unit();
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", (black, origin, Unit{}, Unit()));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}