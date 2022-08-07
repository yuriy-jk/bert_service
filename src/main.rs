mod config;
mod mongo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let ibm_config = config::IbmConfig::init();
    let mongo_config = config::MongoConfig::init();

    let mongo = mongo::init_connection(&mongo_config).await?;

    mongo::download_model(&mongo, &mongo_config, ".").await?;

    // let filter = doc! {"ND": "10005030/0205019/0080151"};
    // let mut cursor = mongo.collection.find(filter, None).await?;
    // while let Some(document) = cursor.try_next().await? {
    //     println!("{:?}", document.get("ND"))
    // }
    Ok(())
}
