use query_string::encode;

fn main() {
    let original = "Hello World!";
    let encoded = encode(&original);
    println!("{encoded}")
}
