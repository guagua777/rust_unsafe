use std::ptr;

fn main() {
    // 空原始指针
    let p: *const i32 = ptr::null();

    if p.is_null() {
        println!("这是空指针");
    }

    // 不能直接解引用空指针，会UB（未定义行为）
    unsafe { println!("{}", *p); } // 崩溃/未定义行为
}