fn main() {
    let (width, height, depth): (i32, i32, i32) = (4, 7, 10);
    println!("Area is {}", area(width, height));
    println!("Volume is {}", volume(width, height, depth));
}

fn area(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}