struct User{
    name: String,
    email : String,
    sign_in_count: u32,
}
fn main() {
    println!("Hello, world!");
    let user1 = User{
        name : String::from("Shubham Jhiknaria"),
        email : String::from("sharmashubh428@gmail.com"),
        sign_in_count: 0,
    };

    let user2 = User{
        name : String::from("Ashu"),
        ..user1
    };

    // let name1  = user1.name; //works
    // let email1 = user1.email; //doesnt work value moved above.
}
