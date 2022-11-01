mod body;
mod trajectory;
use crate::body::body::Body;

fn main() {
    let body = Body::gen_random_body();
    println!("{:?}", body);
}
