use serde_json::Value;

pub fn get_publisher(tenant: &str, token: &str) {
    let tenant_url = format!(
        "https://{}.goskope.com/api/v2/infrastructure/publishers?fields=publisher_id%2Cpublisher_name",
        tenant
    );

    let client = reqwest::blocking::Client::new();

    let request = client
        .get(&tenant_url)
        .bearer_auth(&token)
        .send()
        .expect("Failed to send request");
    let response = request.text().expect("Failed to read response");

    let json: Value = serde_json::from_str(&response).expect("Failed to read JSON");

    let publishers = &json["data"]["publishers"];

    if let Some(array) = publishers.as_array() {
        for p in array {
            let id = p["publisher_id"].as_i64().unwrap();
            let name = p["publisher_name"].as_str().unwrap();

            println!("publisher_id: {}", id);
            println!("publisher_name: {}", name);
            println!("---");
        }
    }
}
