use std::{default, str::FromStr};
use secp256k1::{Secp256k1, SecretKey};

use crate::elementals::address;

pub struct Node{

}

impl Node {
    //发起交易，我们暂不支持从用户处收集交易，所以所有的交易的来自节点发起和签名
    //因此，我们的节点需要有一个自己的账号地址，这里账号会有两个地址，一个是参与网络地址，一个是用户地址，我们均用同一个地址
    //也就是说，如果我们想要发起一笔网络交易，就必须参与运行节点，我们也暂时不公开节点调用api

    //首先允许节点，然后初始化，包括成立一个地址或者输入已经有的地址
    //值得注意的是，我们的私钥保存方式，有两种本地文件和内存之中，一个更方便一个更安全
    //我们选择文件保存
    pub fn init(){
        println!("Please enter the private key or generate a key pair:");
        println!("p for privatekey g for generate a key pair");

        let mut choose: String = String::new();
        let input = std::io::stdin().read_line(&mut choose).unwrap();
        choose.to_ascii_lowercase();
        match choose.as_str() {
            "p" =>{
                println!("please enter ur private key:");
                let mut input_secrate_key = String::new();
                std::io::stdin().read_line(&mut input_secrate_key);
                let secrate_key = SecretKey::from_str(input_secrate_key.as_str()).unwrap();
                address::get_public_key(secrate_key);
            },
            "g" =>{

            },
            _ =>{

            }
        }
    }


    //是先是寻找其他的对等点peer，根据Discv4协议的specs设计
    pub fn inquery(){
        
    }
}