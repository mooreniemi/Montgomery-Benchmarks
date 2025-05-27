use minimal_mult::{
    vanilla_cios::scalar_mul as vciosmult,
    optimised_cios::scalar_mul_unwrapped as ociosmult,
    yuval_mult::scalar_mul as ymult,
    constants::U64_P,
};
use num_bigint::BigUint;

// Convert [u64; 4] to BigUint
fn to_biguint(x: [u64; 4]) -> BigUint {
    let mut bytes = Vec::new();
    for &limb in x.iter().rev() {
        bytes.extend_from_slice(&limb.to_be_bytes());
    }
    BigUint::from_bytes_be(&bytes)
}

// Reduce modulo P
fn reduce_mod_p(x: [u64; 4]) -> [u64; 4] {
    let x_big = to_biguint(x);
    let p_big = to_biguint(U64_P);
    let reduced = x_big % p_big;
    let mut bytes = reduced.to_bytes_be();
    while bytes.len() < 32 {
        bytes.insert(0, 0);
    }

    let mut out = [0u64; 4];
    for i in 0..4 {
        let start = i * 8;
        out[3 - i] = u64::from_be_bytes(bytes[start..start + 8].try_into().unwrap());
    }
    out
}

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

    // Reduce all results modulo p
    let reduced_vcios = reduce_mod_p(result_vcios);
    let reduced_ocios = reduce_mod_p(result_ocios);
    let reduced_ymult = reduce_mod_p(result_ymult);

    println!("\nReduced modulo p:");
    println!("reduced vcios: {reduced_vcios:?}");
    println!("reduced ocios: {reduced_ocios:?}");
    println!("reduced ymult: {reduced_ymult:?}");

    // Check if they're all equal after reduction
    println!("\nPairwise comparisons:");
    println!("vcios == ocios: {}", reduced_vcios == reduced_ocios);
    println!("vcios == ymult: {}", reduced_vcios == reduced_ymult);
    println!("ocios == ymult: {}", reduced_ocios == reduced_ymult);

    if reduced_vcios == reduced_ocios && reduced_ocios == reduced_ymult {
        println!("\n✅ All implementations give the same result after reduction!");
    } else {
        println!("\n❌ Some results differ even after reduction");
    }
}