use crate::models::error::Error;
use bson::doc;
use mongodb::Database;

const COLLECTION_NAME: &str = "errors";

pub async fn get_errors_by_upstream_id_downstream_id(
    db: &Database,
    upstream_id: String,
    downstream_id: String,
) -> mongodb::error::Result<Option<Error>> {
    let collection = db.collection::<Error>(COLLECTION_NAME);
    let filter = doc! {"upstreamId": upstream_id, "downstreamId": downstream_id };
    let errors = collection.find_one(filter, None).await?;

    if errors.is_none() {
        return Ok(None);
    }

    Ok(errors)
}
