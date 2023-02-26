use groovy_ssl::md5;

fn main() {
    println!("Hello, world!");
    println!("{}", md5::hash(b"asdf"));
}
