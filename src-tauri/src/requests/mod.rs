use reqwest::Error;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    id: u32,
    street: String,
    streetName: String,
    buildingNumber: String,
    city: String,
    zipcode: String,
    country: String,
    county_code: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    id: u32,
    firstname: String,
    lastname: String,
    email: String,
    phone: String,
    birthday: String,
    gender: String,
    website: String,
    image: String,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FakeResponse {
    status: String,
    code: u32,
    total: u32,
    data: Vec<Person>,
}

#[tokio::main]
pub async fn request_test() -> Result<FakeResponse, Error> {
    let request_url = format!("https://fakerapi.it/api/v1/persons?_quantity=1&_locale=en_EN");
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await?;

    if response.status().is_success() {
        let fake_response = response.json::<FakeResponse>().await?;
        // println!("{:?}", fake_response);
        Ok(fake_response)
    } else {
        println!("Fake request failed.");
        Ok(FakeResponse::default())
    }
}
