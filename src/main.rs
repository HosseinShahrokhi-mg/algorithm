
struct Point{
    x: f32,
    y: f32,
}
impl Point{
    fn distance_between_points(&self, second_point: Point) -> f32{
        ((self.x - second_point.x).powi(2)+(self.y - second_point.y).powi(2)).sqrt()
        
    }
}
fn main() {
    let first = Point{x:5.0, y:10.0};
    let second = Point{x:8_f32, y:12_f32};

    println!("distance = {}",first.distance_between_points(second));
    //add new code
}
