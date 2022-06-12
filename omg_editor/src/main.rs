extern crate image;
#[macro_use]
extern crate rocket;

use anyhow::Result;
use omg::data::map::Map2d;
use omg::interface::map::MapStorage;
use omg::logging::init_logging;
use omg_serde::interface::map::MapStorageWithSerde;
use rocket::fs::NamedFile;
use rocket::{routes, State};

fn get_map_path(attribute: usize) -> String {
    format!("temp/map-{}.png", attribute)
}

#[get("/map/<attribute_id>")]
async fn get_map(map: &State<Map2d>, attribute_id: usize) -> Option<NamedFile> {
    if let Some(attribute) = map.get_attribute(attribute_id) {
        let path = get_map_path(attribute_id);

        image::save_buffer(
            path.clone(),
            attribute.get_all(),
            map.size().width(),
            map.size().height(),
            image::ColorType::L8,
        )
        .unwrap();

        NamedFile::open(path).await.ok()
    } else {
        None
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    init_logging();

    //info!("Starting Map Editor");

    let port = MapStorageWithSerde {};

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
