use mongodb::{bson::{Document, doc, oid::ObjectId}, error::Error, results::InsertOneResult, Database};

use crate::models::{Channel, Post};

pub async fn add_channel (db: Database, channel: Channel) -> Result<InsertOneResult, Error> {
  return db.collection("channels").insert_one(channel).await;
}

pub async fn get_channel (db: Database, id: ObjectId) -> Result<std::option::Option<Channel>, Error> {
  return db.collection::<Channel>("channels").find_one(doc! {
    "_id": id
  }).await;
}