use std::env;

pub struct IbmConfig {
    pub host: String,
    pub pass: String,
    pub user: String,
    pub manager: String,
    pub channel: String,
    pub queue: String,
    pub next_queue: String,
}

impl IbmConfig {
    pub fn init() -> IbmConfig {
        let config = IbmConfig {
            host: env::var("ibm_host").unwrap(),
            pass: env::var("ibm_pass").unwrap(),
            user: env::var("ibm_user").unwrap(),
            manager: env::var("ibm_manager").unwrap(),
            channel: env::var("ibm_channel").unwrap(),
            queue: env::var("ibm_queue").unwrap(),
            next_queue: env::var("ibm_next_queue").unwrap(),
        };
        config
    }
}

pub struct MongoConfig {
    pub host: String,
    pub db: String,
    pub model_db: String,
    pub collection: String,
    pub model_id: String,
}

impl MongoConfig {
    pub fn init() -> MongoConfig {
        let config = MongoConfig {
            host: format!("mongodb://{}", env::var("mongo_host").unwrap()),
            db: env::var("mongo_db").unwrap(),
            model_db: env::var("mongo_model_db").unwrap(),
            collection: env::var("mongo_collection").unwrap(),
            model_id: env::var("bert_model_id").unwrap(),
        };
        config
    }
}
