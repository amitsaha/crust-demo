extern crate gcc;

fn main() {
    gcc::Config::new().file("src/square.c").compile("libdouble.a");
}
