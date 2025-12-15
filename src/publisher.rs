use crate::builder::Publisher;
use crate::builder::build_name_id_map;
use std::collections::HashMap;
use serde::Deserialize;
use std::error::Error;
#[derive(Debug, Deserialize)]
struct PublishersResponse {
    data: PublishersData,
}

#[derive(Debug, Deserialize)]
struct PublishersData {
    publishers: Vec<Publisher>,
}


pub fn build_publisher_map(
    tenant: &str,
    token: &str,
) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let url = format!(
        "https://{}.goskope.com/api/v2/infrastructure/publishers?fields=publisher_id%2Cpublisher_name",
        tenant
    );
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()?
        .error_for_status()?;

    let body: PublishersResponse = resp.json()?;

    let map = build_name_id_map(body.data.publishers);

    Ok(map)
}

