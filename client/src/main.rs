use base64::prelude::*;
use std::env;
use tokio::fs::File;

//use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

async fn read_file(file:&str){
    let mut file = File::open(file).await;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await;

    println!("len = {}", contents.len());
    BASE64_STANDARD.encode(file);

}
fn main (){
    let args: Vec<String> = env::args().collect();
    let file_name=&args[1];
    println!("{}",file_name);
}



async fn send_request(file:&str){

    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await;
    return ;

}
