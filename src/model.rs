use std::net::Ipv4Addr;
use chrono::{Datelike, Utc};
use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::faker::internet::en::*;

use serde::Serialize;

#[derive(Debug, Dummy, Serialize)]
pub struct Oseba {
    #[dummy(faker = "1..")]
    pub id: u32,
    #[dummy(faker = "FirstName()")]
    pub ime: String,
    #[dummy(faker = "LastName()")]
    pub priimek: String,
    #[dummy(faker = "18..100")]
    pub starost: i32,
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    #[dummy(faker = "IPv4()")]
    pub ip: Ipv4Addr,
}

impl Oseba {
    pub fn leto_rojstva(&self) -> i32 {
        let now = Utc::now();
        let leto = now.date().year();
        leto - self.starost
    }
}