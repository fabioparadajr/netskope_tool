use std::collections::HashMap;
use crate::publisher::build_publisher_map;
use calamine::{RangeDeserializerBuilder, Reader, open_workbook_auto};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use crate::builder::{build_name_id_map, Publisher};

#[derive(Debug, Deserialize)]
pub struct PrivateApp {
    pub app_name: String,
    pub host: String,
    pub port: String,
    pub protocol: String,
    pub publisher_name: String,
    pub tags: String,
    pub use_publisher_dns: bool,
    pub clientless_access: bool,
    pub private_app_protocol: String,
}

#[derive(Debug, Deserialize)]
struct PrivateAppResponse {
    data: PrivateAppData,
}

#[derive(Debug, Deserialize)]
struct PrivateAppData {
    private_apps: Vec<PrivateAppId>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct PrivateAppId{
    pub(crate) app_id: i64,
    pub(crate) app_name: String,
}


pub fn build_privateapps_map(
    tenant: &str,
    token: &str,
) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let url = format!(
        "https://{}.goskope.com/api/v2/steering/apps/private?fields=app_id%2Capp_name",
        tenant
    );
    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()?
        .error_for_status()?;


    let body: PrivateAppResponse = resp.json().expect("failed to deserialize body");

    let map = build_name_id_map(body.data.private_apps);

    Ok(map)
}




pub fn create_privateapp(tenant: &str, token: &str, path: &str) -> Result<(), Box<dyn Error>> {
    
    let mut workbook = open_workbook_auto(path)?;

    let range = workbook.worksheet_range("PrivateApp")?;

    let tenant_url = format!(
        "https://{}.goskope.com/api/v2/steering/apps/private",
        tenant
    );

    let client = reqwest::blocking::Client::new();

    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range::<_, PrivateApp>(&range)?;

    println!("Loading Publishers ID's");
    let publisher_map = build_publisher_map(tenant, token).expect("Failed to build publisher map");

    for result in iter {
        let row: PrivateApp = match result {
            Ok(row) => row,
            Err(e) => {
                eprintln!("Error deserialize: {e}");

                continue;
            }
        };
        let publisher_id = publisher_map
            .get(&row.publisher_name)
            .expect("Failed to get publisher_id");

        if row.clientless_access {
            let body = json!({
                "app_name": row.app_name,
                "host": row.host,
                "clientless_access": row.clientless_access,
                "private_app_protocol": row.private_app_protocol,
                "protocols": [
                    {
                        "port": row.port,
                        "type": row.protocol
                    }
                ],
                "publishers": [
                    {
                        "publisher_id": publisher_id,
                        "publisher_name": row.publisher_name
                    }
                ],
                "tags": [
                    {
                        "tag_name": row.tags
                    }
                ],
            });

            println!("Sending request to {:?}", body);

            if let Err(e) = client
                .post(&tenant_url)
                .bearer_auth(token)
                .json(&body)
                .send()
            {
                eprintln!("Err send request: {e}");
            }
        } else {
            // request client
            let body = json!({
                "app_name": row.app_name,
                "host": row.host,
                "protocols": [
                    {
                        "port": row.port,
                        "type": row.protocol
                    }
                ],
                "publishers": [
                    {
                        "publisher_id": publisher_id,
                        "publisher_name": row.publisher_name
                    }
                ],
                "tags": [
                    {
                        "tag_name": row.tags
                    }
                ],
                "use_publisher_dns": row.use_publisher_dns
            });

            println!("Sending request to {:?}", body);

            if let Err(e) = client
                .post(&tenant_url)
                .bearer_auth(token)
                .json(&body)
                .send()
            {
                eprintln!("Err send request: {e}");
            }
        }
    }

    Ok(())
}


pub fn update_privateapp(tenant: &str, token: &str, path: &str) -> Result<(), Box<dyn Error>> {

    let mut workbook = open_workbook_auto(path)?;

    let range = workbook.worksheet_range("PrivateApp")?;

    let client = reqwest::blocking::Client::new();

    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range::<_, PrivateApp>(&range)?;

    println!("Loading Publishers ID's");
    let publisher_map = build_publisher_map(tenant, token).expect("Failed to build publisher map");
    println!("Loading Private Apps ID's");
    let privateapps_map = build_privateapps_map(tenant, token).expect("Failed to build private apps map");
    dbg!(&privateapps_map);

    for result in iter {
        let row: PrivateApp = match result {
            Ok(row) => row,
            Err(e) => {
                eprintln!("Error deserialize: {e}");

                continue;
            }
        };
        let app_id = match privateapps_map.get(row.app_name.trim()) {
            Some(id) => *id,
            None => {
                eprintln!("Private app not found in tenant: {}", row.app_name);
                continue;
            }
        };

        let tenant_url = format!(
            "https://{}.goskope.com/api/v2/steering/apps/private/{}",
            tenant, app_id
        );


        let publisher_id = publisher_map
            .get(&row.publisher_name)
            .expect("Failed to get publisher_id");

        if row.clientless_access {
            let body = json!({
                "app_name": row.app_name,
                "host": row.host,
                "clientless_access": row.clientless_access,
                "private_app_protocol": row.private_app_protocol,
                "protocols": [
                    {
                        "port": row.port,
                        "type": row.protocol
                    }
                ],
                "publishers": [
                    {
                        "publisher_id": publisher_id,
                        "publisher_name": row.publisher_name
                    }
                ],
                "tags": [
                    {
                        "tag_name": row.tags
                    }
                ],
            });

            println!("Sending request to {:?}", body);

            if let Err(e) = client
                .patch(&tenant_url)
                .bearer_auth(token)
                .json(&body)
                .send()

            {
                eprintln!("Err send request: {e}");
            }

        } else {
            // request client
            let body = json!({
                "app_name": row.app_name,
                "host": row.host,
                "protocols": [
                    {
                        "port": row.port,
                        "type": row.protocol
                    }
                ],
                "publishers": [
                    {
                        "publisher_id": publisher_id,
                        "publisher_name": row.publisher_name
                    }
                ],
                "tags": [
                    {
                        "tag_name": row.tags
                    }
                ],
                "use_publisher_dns": row.use_publisher_dns
            });

            println!("Sending request to {:?}", body);

            if let Err(e) = client
                .patch(&tenant_url)
                .bearer_auth(token)
                .json(&body)
                .send()
            {
                eprintln!("Err send request: {e}");
            }
        }
    }

    Ok(())
}

