use lambdaworks_math as math;
use math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use math::elliptic_curve::traits::IsEllipticCurve;
use math::cyclic_group::IsGroup;

pub const PRIVATE_KEY_HEX: &str = "6C616D6264617370";

fn main() {
        
    let private_key = u64::from_str_radix(PRIVATE_KEY_HEX, 16).unwrap();
    println!("Private Key: {:?}\n", private_key);
    
   
    // Get the generator point G1 from the BLS12-381 curve
    let generator_g1 = BLS12381Curve::generator();
    println!("Generator G1: {:?}\n", generator_g1);

    // Calculate the public key by multiplying the generator point with the private key
    let public_key = generator_g1.operate_with_self(private_key);
    println!("Public Key: {:?}\n", public_key);

}


