#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

/* area method for our Rectangle struct */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height
    }

    /* Associated function */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

#[derive(Debug)]
enum USState{
    CA,
    NY
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(USState)
}

fn main() {
    /* no apparent relation between our data */
    let width = 30;
    let height = 50;

    /* refactor: fixing using tuple types*/
    let rec1 = (30, 50);

    /* refactor: use struct! */
    let rec2 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle: {} square pixels", area(width, height));

    println!("The area of the rectangle using tuple : {} square pixels", area_tup(rec1));

    println!("The area of the rectangle using struc: {} square pixels", rec2.area());

    println!("Our struct looks like: {:#?}", rec2);

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rec12 hold rec3? {}", rec2.can_hold(&rect3));

    let sq = Rectangle::square(3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => 25,
    }
}


