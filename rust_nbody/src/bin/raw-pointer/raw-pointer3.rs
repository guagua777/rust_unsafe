fn main() {
    let val = 666;
    let p: *const i32 = &val;

    unsafe {
        // &* 抵消了，是否可以直接使用p?
        // 裸指针转回 &i32
        let r: &i32 = &*p;
        // let r: &i32 = p;
        println!("{}", r);
    }
}


// *const i32 和 &i32 是两种完全不同的类型，Rust 不会隐式转换它们。

// p 是裸指针 *const i32，它只是一个内存地址，没有任何安全保证
// r 是引用 &i32，Rust 的引用有借用检查器的安全保证（非空、对齐、有效）
// &*p 这两步是必须的：

// *p — 解引用裸指针，得到 i32 值（这步需要 unsafe，因为编译器无法验证指针有效性）
// &(...) — 对该值取引用，得到 &i32
// 虽然 &* 看起来像是"抵消"了，但实际上它完成了一次类型转换：从不安全的裸指针 → 安全的引用。这个转换必须显式写出，Rust 不允许隐式地把裸指针当引用用。



// error[E0308]: mismatched types
//  --> src/bin/raw-pointer/raw-pointer3.rs:9:23
//   |
// 9 |         let r: &i32 = p;
//   |                ----   ^ expected `&i32`, found `*const i32`
//   |                |
//   |                expected due to this
//   |
//   = note: expected reference `&i32`
//            found raw pointer `*const i32`