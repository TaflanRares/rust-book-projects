#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn non_zero_side(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 0,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square1 = Rectangle::square(50);

    if rect1.non_zero_side() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    } else {
        println!("The rectangle has a width or height of zero.");
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));
    println!("Can square1 hold rect1? {}", square1.can_hold(&rect1));
}