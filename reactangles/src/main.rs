#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!("can_hold Result: {}", rect1.can_hold(&rect2));

    println!("rect2: {:#?}", rect1.can_hold(&rect2));

    if rect1.width() || rect1.height() {
        println!("The rectangle has a nonzero width; it is {:?}", Rectangle::square(4));
    }
}
