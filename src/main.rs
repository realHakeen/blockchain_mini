mod elementals;
use elementals::address::{self, get_address, get_key_pair};
fn main() {
    let (secretKey, publicKey) = get_key_pair();
    let address = get_address(publicKey);
    print!("{:?}", address);
}
