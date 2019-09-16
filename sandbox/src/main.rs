fn main() {
    let a = &[1, 2, 3];
    for aa in a {
        let b = aa * aa;
        let c = iss(aa);
    }
    println!("Hello, world!")
}

fn iss<'a>(a: &'a i32) -> &i32 {
    return a;
}
