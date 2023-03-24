struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name:String::from("Adam"),
        likes_oranges: true,
        age: 25,
    };

    println!("Person name is: {}", person.age);

//lets compose a struct with our defined structure
    let origin = Point2D(100, 100);

    println!("Point contains {:?} and {:?}", origin.0, origin.1);

// destruct our Point2D struct
    let Point2D(x, y) = origin;

    println!("Point contains {:?} and {:?}", x, y);
}
