fn main() {
    let x: u64 = 0x12345678;
    let p = &x as *const u64;

    // 强转为 u32 指针
    let p_u32 = p as *const u32;

    unsafe {
        println!("{:x}", *p_u32);
    }
}