use fake::{Fake, Faker};
use crate::model::Oseba;

mod model;

#[macro_use] extern crate rocket;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        osebe: Faker.fake::<[Oseba; 10]>()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
