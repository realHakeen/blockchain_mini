/*
 * @Author: realHakeen yhk15802841343@gmail.com
 * @Date: 2023-08-04 21:49:19
 * @LastEditors: realHakeen yhk15802841343@gmail.com
 * @LastEditTime: 2023-08-16 09:42:08
 * @FilePath: /blockchain_mini/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod elementals;
mod networking;
mod test;
use elementals::address::{self, get_address, get_key_pair};

fn main() {
    let (secretKey, publicKey) = get_key_pair();
    let address = get_address(publicKey);
    print!("{:?}", address);
}
