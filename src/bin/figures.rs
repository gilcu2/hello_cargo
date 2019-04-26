struct Point {
    x: u16,
    y: u16,
}

struct Square {
    corner: Point,
    size: u16,
}

struct Rectangle {
    corner: Point,
    size_x: u16,
    size_y: u16,
}

trait Area<T> {
    fn area(self) -> u16;
}

impl Area<Square> for Square {
    fn area(self) -> u16 {
        self.size * self.size
    }
}

impl Area<Rectangle> for Rectangle {
    fn area(self) -> u16 {
        self.size_x * self.size_y
    }
}

fn main() {
    let square = Square {
        corner: Point { x: 0, y: 0 },
        size: 10,
    };

    let rectangle = Rectangle {
        corner: Point { x: 0, y: 0 },
        size_x: 10,
        size_y: 20,
    };

    println!("Square area: {}", square.area());
    println!("Rectangle area: {}", rectangle.area());
}



