use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Serializer};

pub fn serialize_option_object_id<S>(
    to_serialize: &Option<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match to_serialize {
        Some(id) => id.to_string().serialize(serializer),
        None => serializer.serialize_none(),
    }
}
