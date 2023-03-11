#![allow(unused)]
use serde::{Serialize, Deserialize};
use futures::TryStreamExt;
use ipfs_api::{IpfsClient, IpfsApi, 
    response::IpfsDetailedFile, 
    request::Get};
use std::{
    io::{self, Read, Write}, 
    any::TypeId,
    io::Cursor,
    fs::File,
    fs,
    time::Duration
};
fn main() {
    let cid = "QmTcDMtRvCN8jctiFBXG5RLfipz3CqLuDfBg9y2faHevuP";
    let path = r"test\new_file.json";
    let path_2 = r"\QmTcDMtRvCN8jctiFBXG5RLfipz3CqLuDfBg9y2faHevuP";
    let data_to_write = File::open(&path).expect("could not read source file");
    connect_and_read((&cid).to_string());
    connect_and_write((data_to_write));
    publish_ipns((&cid).to_string());
    
}

#[tokio::main]
async fn connect_and_read(string_hash: String) {
    let client = IpfsClient::default();

    match client
        .get(&string_hash)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();
            println!("Successful in Reading. Here is your file: \n");
            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}

#[tokio::main]
async fn connect_and_write(file: fs::File) {
    let client = IpfsClient::default();

    match client.add(file).await {
        Ok(res) => println!("\nSuccessful in Writing. Here is your hash: {}", res.hash), 
        Err(e) => eprintln!("error adding file: {}", e)
    }
}

#[tokio::main]
async fn publish_ipns(path: String) {
    let client = IpfsClient::default();

    let name = "public_node";
    let duration = Duration::from_secs(3600*14);

    let result = client.name_publish(&path, false, Some("24h"), Some("24h"), None).await.unwrap();

    let ipns_name = result.name.to_string();

    println!("\nPublished content with CID {} to IPNS name {}", path, ipns_name);

}

struct ShopData { 
    // For Sellers ONLY
    shop_code: String,
    shop_name: String,
    shop_id: String, //immutable
    shop_profile: String,
    website: String,
    email: String,
    address: String,
    country_codee: String,
    social_media: String,
    date_created: String, // immutable
    is_active: String,
    shop_owner_wallet_id: String
}

struct ProductsData {
    product_id: String, // immutable
    shop_id: String, // immutable
    product_name: String,
    product_description: String,
    product_price: String,
    product_image: String, // Placeholder only,this should be IPFS address array
    date_created: String, // immutable
    product_sku: String
}
