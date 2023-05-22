mod generate;

fn main() {
    let (pub_key, priv_key) = generate::generate_key();
    println!("公開鍵: {:?}", pub_key);
    println!("秘密鍵: {:?}", priv_key);
}
