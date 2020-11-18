struct Point<T>{
    x: T,
    y: T,
}
impl <T> Point<T>{
    fn distance_between_points(&self, second_point: Point<T>) -> T{
        ((self.x - second_point.x).powi(2)+(self.y - second_point.y).powi(2)).sqrt()
    }
}
fn main() {
    let first = Point{x:5, y:10};
    let second = Point{x:8, y:12};

    println!("distance = {}",first.distance_between_points(second));
}
