mod md5;

fn main() {
    println!("Hello, world!");
    println!("{:x?}", md5::md5(b"asdf"));
    // println!("{:x?}", md5::md5(b"a"));
    // println!("{:x?}", md5::md5(b""));
    // println!("{:x?}", md5::md5(b"asdfasdfasdfasdf"));
    println!(
        "{:x?}",
        md5::md5(
            b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        )
    );
}
