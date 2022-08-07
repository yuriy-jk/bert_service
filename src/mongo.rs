use crate::config;
use futures::stream::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use mongodb::bson::{from_document_with_options, DeserializerOptions};
use mongodb::{Client, Collection, Database};
use mongodb_gridfs::{
    bucket::GridFSBucket, options::GridFSBucketOptions, options::GridFSFindOptions,
};

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    _id: ObjectId,
    filename: String,
    model_id: String,
    model_name: String,
    model_type: String,
    base_model_id: String,
    md5: String,
    chunkSize: i32,
    length: i64,
    uploadDate: DateTime,
}

pub struct Mongo {
    pub client: Client,
    pub db: Database,
    pub collection: Collection<Document>,
    pub model_db: Database,
}

pub async fn init_connection(
    mongo_config: &config::MongoConfig,
) -> Result<Mongo, Box<dyn std::error::Error>> {
    let client = Client::with_uri_str(&mongo_config.host).await?;
    let db = client.database(&mongo_config.db);
    let bert_db = client.database(&mongo_config.model_db);
    let collection = db.collection::<Document>(&mongo_config.collection);
    let mongo = Mongo {
        client: client,
        db: db,
        model_db: bert_db,
        collection: collection,
    };
    Ok(mongo)
}

pub async fn download_model(
    mongo: &Mongo,
    mongo_config: &config::MongoConfig,
    model_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bucket_options = GridFSBucketOptions::builder()
        .bucket_name("models".to_string())
        .build();

    let bucket = GridFSBucket::new(mongo.model_db.clone(), Some(bucket_options));

    let mut cursor_bert = bucket
        .find(
            doc! {"model_id": &mongo_config.model_id},
            GridFSFindOptions::default(),
        )
        .await?;

    while let Some(_doc) = cursor_bert.next().await {
        let options = DeserializerOptions::builder().human_readable(false).build();
        let model: ModelConfig = from_document_with_options(_doc.unwrap(), options).unwrap();

        let _filename = &model.filename;
        let _model_id = &model.model_id;
        let _id = model._id;

        let mut file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(_filename)
            .expect("create failed");

        let mut cursor = bucket.open_download_stream(_id).await?;
        while let Some(buffer) = cursor.next().await {
            file.write(&buffer).expect("error while download model");
        }
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let path = "bert_model";
        fs::create_dir_all(path)?;
        archive
            .extract(model_path)
            .expect("error while extract model archive");

        println!("model={}, successfully download", &model.filename)
    }
    Ok(())
}
