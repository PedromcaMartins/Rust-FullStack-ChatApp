use rocket::fairing::AdHoc;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(backend::AppState::new())
        .mount("/", backend::routes())
        .attach(AdHoc::on_ignite("Database Migrations", backend::run_migrations))
}