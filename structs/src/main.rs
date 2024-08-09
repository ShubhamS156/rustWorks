struct User {
    name: String,
    email: String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn width(&self) -> u32{
        self.width
    }
    //above 3 are methods, next one is associated function.
    //difference is that it doesn't need reference to Self(similar to static funcs?)
    
    // Returns a new rectangle with equal h and w.
    fn Square(size : u32) -> Self{
        Self{
            width : size,
            height : size,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let user1 = User {
        name: String::from("Shubham Jhiknaria"),
        email: String::from("sharmashubh428@gmail.com"),
        sign_in_count: 0,
    };

    let user2 = User {
        name: String::from("Ashu"),
        ..user1
    };

    // let name1  = user1.name; //works
    // let email1 = user1.email; //doesnt work value moved above.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle: {:#?} \nArea: {}", rect1, rect1.area());
    
    let rectEq = Rectangle::Square(10);
}
