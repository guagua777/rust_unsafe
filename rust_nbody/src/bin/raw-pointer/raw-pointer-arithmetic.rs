fn main1() {
    let arr = [10, 20, 30];
    // 数组首地址转原始指针
    let mut p = &arr[0] as *const i32;

    unsafe {
        for _ in 0..3 {
            println!("{}", *p);
            p = p.add(1); // 指针向后偏移一个元素
        }
    }
}

fn main() {
    struct_pointer();
}


struct Point {
    x: i32,
    y: i32,
}


fn struct_pointer() {

    let arr = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];

    let mut p = &arr[0] as *const Point;

    unsafe {
        // 超出索引范围
        for _ in 0..3 {
            println!("({}, {})", (*p).x, (*p).y);
            p = p.add(1); // 指针向后偏移一个元素
        }
    }

}