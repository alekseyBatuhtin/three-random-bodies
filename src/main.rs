mod body;
mod trajectory;
use crate::body::body::Body;

fn main() {
    let body = Body::new(1.0, 1.0, 1.0, 100);
    println!("{:?}", body);
}
