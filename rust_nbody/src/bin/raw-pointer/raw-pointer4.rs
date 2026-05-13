fn get_ptr() -> *const i32 {
    let x = 999;
    &x as *const i32
}

fn main() {
    let p = get_ptr();
    // unsafe { println!("{}", *p); } // UB！指向已销毁栈内存
}