use rdf_fusion::store::Store;

#[tokio::main]
async fn main() {
    let store = Store::from_env();
    println!("Store created successfully!");
}
