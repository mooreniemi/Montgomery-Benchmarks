use minimal_mult::{
    vanilla_cios::scalar_mul as oldmult,
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
    let result_old = oldmult(a, b);
    let result_new = ymult(a, b);

    println!("Old multiplication result: {result_old:?}");
    println!("New multiplication result: {result_new:?}");
}