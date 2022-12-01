mod md5;

fn main() {
    println!("Hello, world!");
    println!("{}", md5::to_string(&md5::md5(b"asdf")));
}
