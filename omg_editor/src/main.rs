extern crate image;
#[macro_use]
extern crate rocket;

use anyhow::Result;
use omg::data::map::Map2d;
use omg::data::math::selector::ColorSelector;
use omg::interface::map::MapStorage;
use omg::interface::selector::SelectorStorage;
use omg::logging::init_logging;
use omg_serde::interface::map::MapStorageWithSerde;
use omg_serde::interface::selector::SelectorStorageWithSerde;
use rocket::fs::NamedFile;
use rocket::{routes, State};

fn get_map_path(attribute: usize) -> String {
    format!("temp/map-{}.png", attribute)
}

struct EditorData {
    map: Map2d,
    selector: ColorSelector,
}

#[get("/map/<attribute_id>")]
async fn get_map(data: &State<EditorData>, attribute_id: usize) -> Option<NamedFile> {
    let map = &data.map;

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

#[get("/map/color/<attribute_id>")]
async fn get_color_map(data: &State<EditorData>, attribute_id: usize) -> Option<NamedFile> {
    let map = &data.map;

    if let Some(attribute) = map.get_attribute(attribute_id) {
        let path = get_map_path(attribute_id);

        let buf: Vec<u8> = attribute
            .get_all()
            .iter()
            .flat_map(|value| {
                let color = data.selector.get(*value);
                let array: [u8; 3] = color.into();
                array
            })
            .collect();

        image::save_buffer(
            path.clone(),
            &buf,
            map.size().width(),
            map.size().height(),
            image::ColorType::Rgb8,
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

    let map_storage = MapStorageWithSerde {};

    let map_generation = map_storage.read("resources/map_generation/biome.yaml")?;
    let map = map_generation.generate();

    let selector_storage = SelectorStorageWithSerde {};
    let selector = selector_storage.read("resources/color_selector/elevation.yaml")?;

    if let Err(e) = rocket::build()
        .manage(EditorData { map, selector })
        .mount("/", routes![get_map, get_color_map])
        .launch()
        .await
    {
        //warn!("Rocket didn't launch!");
        drop(e);
    };

    Ok(())
}
