// Rust 原始指针 = 不受 Rust 安全规则保护的 “裸指针”
// 它就是一个内存地址，和 C 语言的指针几乎一模一样
// Rust 不会帮你检查空指针、野指针、重复释放、数据竞争
// 必须写在 unsafe 块里才能使用

// 两种原始指针
// *const T    // 只读原始指针（不能改指向的值）
// *mut T      // 可写原始指针（能改指向的值）


fn main1() {
    let mut x = 10;
    // &mut x    as    *mut i32
    let p = &mut x as *mut i32;  // 转成原始指针

    // 必须用 unsafe 才能读写指针
    unsafe {
        *p = 20;  // 修改指针指向的值
        println!("{}", *p); // 20
    }

    println!("{}", x); // 20（确实改成功了）
}


fn main() {
    let mut val = 100;

    // 直接从引用转为 只读 原始指针
    // let p_const: *const i32 = &val;
    // let p_mut: *mut i32 = &mut val;

    // 安全：引用转原始指针
    let p_const: *const i32 = &val;
    let p_mut: *mut i32 = &mut val;

    // 不安全：解引用读写
    unsafe {
        println!("从const指针取值: {}", *p_const);
        *p_mut = 200;
        println!("修改后: {}", *p_mut);
    }

    println!("原变量: {}", val);
}