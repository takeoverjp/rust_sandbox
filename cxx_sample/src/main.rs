extern {
    pub fn hello();
}

fn main() {
    println!("Hello from Rust");
    unsafe { hello() }
}
