#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unsafe_op_in_unsafe_fn, static_mut_refs)]

use std::arch::x86_64::*;
use std::f64::consts::PI;
use std::mem;

// Note 1
#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3], // 速度
    mass: f64, // 质量，大量
}

// Note 1
const SOLAR_MASS: f64 = 4. * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

// 静态可变数组，类型为body
//     ,------------------------------------- Note 2
//     |                           ,--------- Note 3
//     v                           v
static mut solar_Bodies: [body; BODIES_COUNT] = [
    body {
        // Sun      <------------------ Note 4
        mass: SOLAR_MASS,
        position: [0.; 3], // <-------------- Note 5
        velocity: [0.; 3],
    },
    body {
        // Jupiter
        position: [
            // <--------------------- Note 6
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
        ],
        velocity: [
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },
    body {
        // Saturn
        position: [
            // <--------------------- Note 6
            8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01,
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
            4.99852801234917238e-03 * DAYS_PER_YEAR,
            2.30417297573763929e-05 * DAYS_PER_YEAR,
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },
    body {
        // Uranus
        position: [
            // <--------------------- Note 6
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
        ],
        velocity: [
            2.96460137564761618e-03 * DAYS_PER_YEAR,
            2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },
    body {
        // Neptune
        position: [
            // <--------------------- Note 6
            1.53796971148509165e+01,
            -2.59193146099879641e+01,
            1.79258772950371181e-01,
        ],
        velocity: [
            2.68067772490389322e-03 * DAYS_PER_YEAR,
            1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR,
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    },
    // ... more planets here ...
];



// 将方法去掉unsafe，参数由指针改为mut数组
// 使用的地方，将指针改为数组索引

//                           ,------------------------- Note 1
//                           |          ,-------------- Note 2
//                           v          v
fn offset_Momentum(bodies: &mut [body; BODIES_COUNT]) {
    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            //     v----------------------------------- Note 3
            bodies[0].velocity[m] -=
                bodies[i].velocity[m] * bodies[i].mass / SOLAR_MASS;
        }
    }
}


// The answer comes back to local reasoning

fn output_Energy(bodies: &mut [body; BODIES_COUNT]){
    let mut energy = 0.;
    for i in 0..BODIES_COUNT {
        // Add the kinetic energy for each body.
        energy += 0.5 * bodies[i].mass * (
            bodies[i].velocity[0]
                * bodies[i].velocity[0]
            + bodies[i].velocity[1]
                * bodies[i].velocity[1]
            + bodies[i].velocity[2]
                * bodies[i].velocity[2]);

        // Add the potential energy between this body and
        // every other body.
        for j in i+1..BODIES_COUNT {
            // The standard library provides 
            // std::mem::MaybeUninit for expressing storage locations that might be uninitialized.
            // let mut position_Delta =   // <----------- Note 1
            //     [mem::MaybeUninit::<f64>::uninit(); 3];
            // for m in 0..3 {
            //     position_Delta[m].as_mut_ptr().write(
            //         bodies[i].position[m]
            //             - bodies[j].position[m]
            //     );
            // }


            // 它不再依赖于那些如果使用不当可能会破坏内存安全的功能，例如未初始化的内存。
            // 事实上，使用未初始化的内存是唯一剩下的 unsafe 因素。 
            // 我们可以通过删除函数顶部的 unsafe 关键字并重新编译来检查 output_Energy 值。如果编译成功，那就没问题了。
            let mut position_Delta = [0.; 3];
            for m in 0..3 {
                position_Delta[m] = bodies[i].position[m] - bodies[j].position[m];
                
            }

            // 不再需要类型转换        
            // let position_Delta: [f64; 3] = // <------- Note 2
            //     mem::transmute(position_Delta);

            energy -= bodies[i].mass
                * bodies[j].mass
                / f64::sqrt(               // <------- Note 3
                      position_Delta[0]*position_Delta[0]+
                      position_Delta[1]*position_Delta[1]+
                      position_Delta[2]*position_Delta[2]
                  );
        }
    }

    // Output the total energy of the system.
    println!("{:.9}", energy);
}







unsafe fn advance(bodies: &mut [body; BODIES_COUNT]) {

    const INTERACTIONS_COUNT: usize =
        BODIES_COUNT * (BODIES_COUNT - 1) / 2;
    const ROUNDED_INTERACTIONS_COUNT: usize =
        INTERACTIONS_COUNT + INTERACTIONS_COUNT % 2;


    // // I’ve written Align16 as a tuple struct with unnamed fields
    // // Note 1
    // #[repr(align(16))]
    // #[derive(Copy, Clone)]
    // struct Align16([f64; ROUNDED_INTERACTIONS_COUNT]);


    // union 成员不是公开的
    #[derive(Copy, Clone)]
    #[repr(C)]
    union Interactions {
        scalars: [f64; ROUNDED_INTERACTIONS_COUNT],
        vectors: [__m128d; ROUNDED_INTERACTIONS_COUNT / 2],
    }

    impl Interactions {
        pub fn as_scalars(&mut self) -> &mut [f64; ROUNDED_INTERACTIONS_COUNT] {
            unsafe {
                &mut self.scalars
            }
        }

        pub fn as_vectors(&mut self)
            -> &mut [__m128d; ROUNDED_INTERACTIONS_COUNT / 2]
        {
            unsafe {
                &mut self.vectors
            }
        }
    }
    

    // Note 2
    // 标记为static
    // static mut position_Deltas: [Align16; 3] =
    //     [Align16([0.; ROUNDED_INTERACTIONS_COUNT]); 3];
    // static mut magnitudes: Align16 =
    //     Align16([0.; ROUNDED_INTERACTIONS_COUNT]);


    static mut position_Deltas: [Interactions; 3] =
        [Interactions { scalars: [0.; ROUNDED_INTERACTIONS_COUNT] }; 3];
    static mut magnitudes: Interactions =
        Interactions { scalars: [0.; ROUNDED_INTERACTIONS_COUNT] };    

    {
        let mut k = 0;
        for i in 0..BODIES_COUNT-1 {
            for j in i+1..BODIES_COUNT {
                for m in 0..3 {
                    // position_Deltas[m].0[k] =
                    //     bodies[i].position[m]
                    //     - bodies[j].position[m];
                    position_Deltas[m].as_scalars()[k] =
                        bodies[i].position[m] - bodies[j].position[m];    
                }
                k += 1;
            }
        }
    }   


    for i in 0..ROUNDED_INTERACTIONS_COUNT/2 {

        // 跟 output_Energy 中一样 let mut position_Delta = [0.; 3];
        // 使用 零初始化数组并立即覆盖它
        // let mut position_Delta =
        //     [mem::MaybeUninit::<__m128d>::uninit(); 3];
        let mut position_Delta = [_mm_setzero_pd(); 3];

        for m in 0..3 {
            // position_Delta[m].as_mut_ptr().write(
            //     // Rust 中的类型转换比 C 多一次（ 注 1 ）。
            //     // 这是因为 Rust 中的表达式 &x 将 x 的地址作为引用， 
            //     // 但我们想要的是一个原始指针——所以我们先将其转换为一个指针，然后再转换为另一种类型的指针
            //     *(&position_Deltas[m].0
            //         as *const f64         // <----- Note 1
            //         as *const __m128d).add(i)
            // );

            // position_Delta[m] = position_Deltas[m].vectors[i];
            position_Delta[m] = position_Deltas[m].as_vectors()[i];
        }

        // 不再需要类型转换
        // let position_Delta: [__m128d; 3] =
        //     mem::transmute(position_Delta); 



    let distance_Squared: __m128d = _mm_add_pd(
        _mm_add_pd(
            _mm_mul_pd(position_Delta[0], position_Delta[0]),
            _mm_mul_pd(position_Delta[1], position_Delta[1]),
        ),
        _mm_mul_pd(position_Delta[2], position_Delta[2]),
    );

    let mut distance_Reciprocal: __m128d =
    _mm_cvtps_pd(_mm_rsqrt_ps(_mm_cvtpd_ps(distance_Squared)));
    for _ in 0..2 {
        distance_Reciprocal = _mm_sub_pd(
            _mm_mul_pd(distance_Reciprocal, _mm_set1_pd(1.5)),
            _mm_mul_pd(
                _mm_mul_pd(
                    _mm_mul_pd(
                        _mm_set1_pd(0.5),
                        distance_Squared,
                    ),
                    distance_Reciprocal,
                ),
                _mm_mul_pd(
                    distance_Reciprocal,
                    distance_Reciprocal,
                ),
            ));
    }

    // 直接使用数组索引
    // (magnitudes.0.as_mut_ptr() as *mut __m128d)
    //     .add(i)
    //     .write(_mm_mul_pd(
    //         _mm_div_pd(
    //             _mm_set1_pd(0.01),
    //             distance_Squared,
    //         ),
    //         distance_Reciprocal,
    //     )); 
    magnitudes.as_vectors()[i] =  _mm_mul_pd(
            _mm_div_pd(
                _mm_set1_pd(0.01),
                distance_Squared,
            ),
            distance_Reciprocal,
        );   
    } // end of loop

    // {
    //     let mut k = 0;
    //     for i in 0..BODIES_COUNT-1 {
    //         for j in i+1..BODIES_COUNT {
    //             let i_mass_magnitude =
    //                 bodies[i].mass * magnitudes.0[k];
    //             let j_mass_magnitude =
    //                 bodies[j].mass * magnitudes.0[k];
    //             for m in 0..3 {
    //                 bodies[i].velocity[m] -=
    //                 position_Deltas[m].0[k] * j_mass_magnitude;
    //                 bodies[j].velocity[m] +=
    //                 position_Deltas[m].0[k] * i_mass_magnitude;
    //             }
    //             k += 1;
    //         }
    //     }
    // }


    {
        let mut k = 0;
        for i in 0..BODIES_COUNT-1 {
            for j in i+1..BODIES_COUNT {
                let i_mass_magnitude =
                    bodies[i].mass * magnitudes.as_scalars()[k];
                let j_mass_magnitude =
                    bodies[j].mass * magnitudes.as_scalars()[k];
                for m in 0..3 {
                    bodies[i].velocity[m] -=
                    position_Deltas[m].as_scalars()[k] * j_mass_magnitude;
                    bodies[j].velocity[m] +=
                    position_Deltas[m].as_scalars()[k] * i_mass_magnitude;
                }
                k += 1;
            }
        }
    }

    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            bodies[i].position[m] +=
                0.01 * bodies[i].velocity[m];
        }
    }       
}

// fn main() {
//     unsafe {
//         offset_Momentum(solar_Bodies.as_mut_ptr());  // Note 1
//         output_Energy(solar_Bodies.as_mut_ptr());
//         let c = std::env::args().nth(1).unwrap()  // Note 2
//             .parse().unwrap();
//         for _ in 0..c {
//             advance(solar_Bodies.as_mut_ptr())
//         }
//         output_Energy(solar_Bodies.as_mut_ptr());
//     }
// }

fn main() {
    unsafe {
        // 尽量少用指针
        // 将指针改为mut数组
        // 1. 不为null， 2. 不会未初始化， 3, 数组类型匹配
        // 通过避免使用原始指针造成类型瓶颈，我们保证了参数永远不会为 NULL ，大小始终符合预期，并且都已初始化。
        offset_Momentum(&mut solar_Bodies);       // <--- Changed!
        output_Energy(&mut solar_Bodies);
        let c = std::env::args().nth(1).unwrap().parse().unwrap();
        for _ in 0..c { advance(&mut solar_Bodies) }
        output_Energy(&mut solar_Bodies);
    }
}