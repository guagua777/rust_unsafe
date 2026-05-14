# rust_unsafe

This is a tutorial on translating C language code using unsafe, gradually eliminating unsafe, and ultimately achieving implementation using only regular Rust code

## reference
1. https://cliffle.com/p/dangerust/


# unsafe优化的整体流程

## 第一部分：
1. 定义结构体struct，并使用与c兼容的格式来布局struct #[repr(C)] 
2. 使用rsut的const来代替c中的宏定义
3. 使用static和mut来定义struct
4. 将函数定义为unsafe
    ```
    unsafe fn my_function() {
        // 一些不安全的操作代码
    }
    ```
5. 将c语言中的参数：数组  body bodies[]，替换为 原始指针 bodies: *mut body
6. 使用原始指针的计算来访问数组元素
7. 使用未初始化的变量 std::mem::MaybeUninit
8. 使用重新解释内存类型，std::mem::transmute
9. 函数内部使用static 限定符的变量
10. 使用#[repr(align(16))]来设置内存对齐
11. 使用{},将某些临时变量限制在作用域内
12. 使用as进行类型转换
13. 使用unsafe {}来包裹不安全的操作代码

## 第二部分：
1. 参数将指针，改为rust数组

## 第三部分：
1. 处理未初始化内存，使用rust的，零初始化数组并立即覆盖它
2. 不再需要类型转换，对内存进行重新解释的类型转换
3. 去掉output_Energy方法的unsafe标识
   
## 第四部分：
1. 使用union定义，两个浮点数或者是__m128d类型
2. 并为该union定义两个访问函数，可以获取不同的类型，即将该内存解释为不同的类型，同时尽量缩减unsafe的范围


## 第五部分：
1. 将 static mut 变量 定义为let mut 变量，并通过参数进行传递
2. 使用unsafe块包裹向量运算
3. 最终可实现将advance方法的unsafe标识去掉，同时，main方法也可去掉unsafe块
   
## Wrapping up 最终，我们做了什么
1. We’ve reached our destination! Let’s look back at what we’ve done in the series so far.

2. Starting with a C program, we’ve produced an equivalent Rust program, following an incremental process without sweeping architectural changes. Comparing the two,
   1. The Rust code doesn’t use any pointer arithmetic or unchecked array indexing, which eliminates bugs like buffer overruns, stack smashes, and the like.
   2. The Rust code doesn’t use raw pointers at all, relying on Rust references to eliminate the possibility of NULL pointer bugs or accesses to uninitialized memory.
   3. While we don’t use threads, the Rust code is fully-reentrant and thread safe, so we could operate in a threaded environment if needed.
   4. The Rust program is slightly faster than the C program compiled by Clang (by about 3%), and significantly faster than GCC’s output (by 17%).
   5. The Rust source code is longer, mostly due to the need to call Intel SIMD intrinsics directly, while the C program relies on non-standard operator overloading.
   6. When both programs are statically linked4, the Rust binary is smaller — 40% the size of the GCC/Clang output.
   7. Compile times are roughly equivalent (GCC is slightly faster, Clang, slightly slower).


## 第六部分：
1. 使用struct
2. 使用rust内置函数：bodies.split_first_mut()
3. 使用for循环迭代
4. for (i, body) in bodies.iter().enumerate()
5. for (i, mag) in magnitudes.iter_mut().enumerate()
6. for (m, pos) in body.position.iter_mut().enumerate()