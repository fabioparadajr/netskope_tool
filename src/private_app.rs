
use calamine::{
    RangeDeserializerBuilder, Reader,
    open_workbook_auto,
};
use serde::{Deserialize};
use serde_json::json;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct PrivateApp {
    pub app_name: String,
    pub host: String,
    pub port: String,
    pub protocol: String,
    pub publisher_id: String,
    pub publisher_name: String,
    pub tags: String,
    pub use_publisher_dns: bool,
    pub clientless_access: bool,
    pub private_app_protocol: String,
}

pub fn create_privateapp(tenant: &str, token: &str, path: &str) -> Result<(), Box<dyn Error>> {
    //let path = "/home/fabio/RustroverProjects/netskope_tool/target/debug/applications.xlsx";

    // Já trata o erro aqui em vez de usar unwrap()
    let mut workbook = open_workbook_auto(path)?;

    let range = workbook.worksheet_range("PrivateApp")?;

    let tenant_url = format!(
        "https://{}.goskope.com/api/v2/steering/apps/private",
        tenant
    );

    let client = reqwest::blocking::Client::new();

    // Cria o iterador de linhas -> PrivateApp
    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range::<_, PrivateApp>(&range)?;

    // AGORA vem o loop: um request por linha válida
    for result in iter {
        let row: PrivateApp = match result {
            Ok(row) => row,
            Err(e) => {
                eprintln!("Erro ao desserializar linha da planilha: {e}");
                // aqui você decide: pula a linha com erro e continua
                continue;
            }
        };
        if row.clientless_access == true {
            // request modo clientless(browser)
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
                        "publisher_id": row.publisher_id,
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
                eprintln!("Erro ao enviar request: {e}");
            }
        } else {
            // request modo client
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
                        "publisher_id": row.publisher_id,
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
                eprintln!("Erro ao enviar request: {e}");
            }
        }
    }

    Ok(())
}
