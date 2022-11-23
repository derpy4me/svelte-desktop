use reqwest::Result;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct Address {
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

#[derive(Deserialize, Debug)]
struct Person {
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

#[derive(Deserialize, Debug, Default)]
struct FakeResponse {
    status: String,
    code: u32,
    total: u32,
    data: Vec<Person>,
}


#[tokio::main]
pub async fn request_test() -> Result<()> {
    let request_url = format!("https://fakerapi.it/api/v1/persons?_quantity=1&_locale=en_EN");
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await?;

    if response.status().is_success() {
        let response_test = response.json::<FakeResponse>().await?;
        println!("{:?}", response_test);
    } else {
        println!("Fake request failed.");
    }

    Ok(())
}