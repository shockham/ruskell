#[macro_use]
extern crate ruskell;

fn main() {
    rkl! {
        add a b = a + b
    }

    println!("{}", add(2f32, 4f32));
}
