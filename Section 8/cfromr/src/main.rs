#[link(name = "badmath", kind = "static")]
extern "C" {
    fn bad_add(v1: f32, v2: f32) -> f32;
}

fn main() {
    println!("Hello, from RUST!");
    let res = unsafe { bad_add(9., 12.) };
    println!("{}?? Are you sure that's right?", res);
}
