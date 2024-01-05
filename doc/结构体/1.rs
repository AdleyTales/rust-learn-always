#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("Adleytales"),
        email: String::from("abc@126.com"),
        active: true,
        sign_in_count: 2,
    };

    //   mut
    user1.email = String::from("abcd@126.com");

    println!("{:#?}", user1);
}
