use num_bigint::{BigInt};

// generate a key pair (public key, private key)
pub fn generate_key() -> ((BigInt, BigInt), BigInt) {
    let p: i32 = 3;
    let q: i32 = 5;

    let n: i32 = p * q;
    let e: i32 = 3;
    
    let d: i32 = n * (p - 1) * (q - 1) + 1;

    ((BigInt::from(n), BigInt::from(e)), BigInt::from(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let (pub_key, priv_key) = generate_key();
        assert_eq!(pub_key, (BigInt::from(15), BigInt::from(3)));
        assert_eq!(priv_key, BigInt::from(61));
    }
}