use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Serializer};

pub fn serialize_option_object_id<S>(
    option_object_id: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match option_object_id {
        Some(object_id) => object_id.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_object_id<S>(object_id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    object_id.serialize(serializer)
}
