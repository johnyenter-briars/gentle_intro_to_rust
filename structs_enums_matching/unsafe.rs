static mut A: u32 = 0;
static mut B: u32 = 0;
static mut C: u32 = 0;

fn main() {
    unsafe {
        A = 3;
        B = 4;
        A = A + B;
        C = B;
        println!("{} {} {}", A, B, C);
        C = A;
    }
}