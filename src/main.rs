fn main() {
    println!("Hello, world!");
}

fn cstr(string: &str) -> *const u8 {
    todo!()
}

fn read_all(fd: i32) -> Vec<u8> {
    todo!()
}

unsafe extern "C" {
    fn open(pathname: *const u8, flags: i32, mode: u32) -> i32;
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn close(fd: i32) -> i32;
}
