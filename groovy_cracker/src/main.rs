use groovy_ssl::md5;

fn main() {
    println!("Hello, world!");
    println!("{}", md5::md5(b"asdf"));
}
