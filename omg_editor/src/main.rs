extern crate image;
#[macro_use]
extern crate rocket;

use anyhow::Result;
use omg::data::map::Map2d;
use omg::interface::io::StoragePort;
use omg::logging::init_logging;
use omg_serde::interface::io::StoragePortWithSerde;
use rocket::fs::NamedFile;
use rocket::{routes, State};

#[get("/")]
async fn get_map(map: &State<Map2d>) -> Option<NamedFile> {
    let attribute = map.get_attribute(0);
    image::save_buffer(
        "image.png",
        attribute.get_all(),
        map.size().width(),
        map.size().height(),
        image::ColorType::L8,
    )
    .unwrap();
    NamedFile::open("image.png").await.ok()
}

#[rocket::main]
async fn main() -> Result<()> {
    init_logging();

    //info!("Starting Map Editor");

    let port = StoragePortWithSerde {};

    let map_generation = port.read("resources/map_generation/biome.yaml")?;
    let map = map_generation.generate();

    if let Err(e) = rocket::build()
        .manage(map)
        .mount("/", routes![get_map])
        .launch()
        .await
    {
        //warn!("Rocket didn't launch!");
        drop(e);
    };

    Ok(())
}
