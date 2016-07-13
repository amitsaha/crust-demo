extern crate libc;

extern {
    fn square(x: libc::c_int) -> libc::c_int;
}

fn main() {
    let x = 4;
    println!("Square of {}: {}", x, unsafe {square(4)});
}
