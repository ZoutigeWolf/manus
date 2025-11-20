use reqwest::Client;
use crate::models::{ManusData, Me, Token};

pub async fn get_token(username: &String, password: &String) -> Result<Token, reqwest::Error> {
    let client = Client::new();

    let params = [
        ("grant_type", "password"),
        ("client_id", "employee"),
        ("username", username.as_str()),
        ("password", password.as_str())
    ];

    let data = client
        .post("https://server.manus.plus/intergamma/app/token")
        .form(&params)
        .send()
        .await?
        .json::<Token>()
        .await?;

    Ok(data)
}

pub async fn get_me(token: &Token) -> Result<Me, reqwest::Error> {
    let client = Client::new();

    let data = client
        .get("https://server.manus.plus/intergamma/api/user/me")
        .bearer_auth(token.access_token.as_str())
        .send()
        .await?
        .json::<Me>()
        .await?;

    Ok(data)
}


pub async fn get_manus_data(node_id: &String, employee_id: &String, year: &u32, week: &u32, token: &Token) -> Result<ManusData, reqwest::Error> {
    let client = Client::new();

    let data = client
        .get(format!("https://server.manus.plus/intergamma/api/node/{}/employee/{}/schedule/{}/{}/fromData", node_id, employee_id, year, week))
        .bearer_auth(token.access_token.as_str())
        .send()
        .await?
        .json::<ManusData>()
        .await?;

    Ok(data)
}

