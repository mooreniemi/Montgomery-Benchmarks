use minimal_mult::{
    vanilla_cios::scalar_mul as vciosmult,
    optimised_cios::scalar_mul_unwrapped as ociosmult,
    yuval_mult::scalar_mul as ymult,
};

fn main() {
    // Original inputs
    let a = [
        4412042023574846361,
        16957435782954092809,
        5236635384556860354,
        4927983752379797604,
    ];
    let b = [
        4412042023574846361,
        16957435782954092809,
        5236635384556860354,
        4927983752379797604,
    ];

    // Run both multiplications
    let result_vcios = vciosmult(a, b);
    let result_ocios = ociosmult(a, b);
    let result_ymult = ymult(a, b);

    println!("vciosmult result: {result_vcios:?}");
    println!("ociosmult result: {result_ocios:?}");
    println!("ymult result: {result_ymult:?}");
}