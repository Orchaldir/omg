extern crate image;
#[macro_use]
extern crate rocket;

use anyhow::Result;
use image::ColorType;
use omg::data::map::attribute::Attribute;
use omg::data::map::Map2d;
use omg::data::math::selector::ColorSelector;
use omg::interface::map::MapStorage;
use omg::interface::selector::SelectorStorage;
use omg::logging::init_logging;
use omg_serde::interface::map::MapStorageWithSerde;
use omg_serde::interface::selector::SelectorStorageWithSerde;
use rocket::fs::NamedFile;
use rocket::{routes, State};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;

struct EditorData {
    map: Map2d,
    selectors: HashMap<usize, ColorSelector>,
}

#[get("/")]
fn home(data: &State<EditorData>) -> Template {
    let map_name = data.map.name();

    Template::render(
        "home",
        context! {
            map_name: map_name,
            width: data.map.size().width(),
            height: data.map.size().height(),
            attributes: get_attributes(&data.map),
        },
    )
}

#[get("/view/<attribute_id>")]
async fn view_attribute(data: &State<EditorData>, attribute_id: usize) -> Template {
    Template::render(
        "view_attribute",
        context! {
            attribute_id: attribute_id,
            attribute_name: get_attribute_name(&data.map, attribute_id),
            attributes: get_attributes(&data.map),
        },
    )
}

#[get("/view/<id0>/<id1>/<id2>/<id3>")]
async fn view_quad(
    data: &State<EditorData>,
    id0: usize,
    id1: usize,
    id2: usize,
    id3: usize,
) -> Template {
    Template::render(
        "view_quad",
        context! {
            attribute0: (id0, get_attribute_name(&data.map, id0)),
            attribute1: (id1, get_attribute_name(&data.map, id1)),
            attribute2: (id2, get_attribute_name(&data.map, id2)),
            attribute3: (id3, get_attribute_name(&data.map, id3)),
            attributes: get_attributes(&data.map),
        },
    )
}

#[get("/map/<attribute_id>")]
async fn get_map(data: &State<EditorData>, attribute_id: usize) -> Option<NamedFile> {
    let map = &data.map;

    if let Some(attribute) = map.get_attribute(attribute_id) {
        create_gray_map(attribute).await
    } else {
        None
    }
}

#[get("/map/color/<attribute_id>")]
async fn get_color_map(data: &State<EditorData>, attribute_id: usize) -> Option<NamedFile> {
    let map = &data.map;

    if let Some(attribute) = map.get_attribute(attribute_id) {
        if let Some(selector) = data.selectors.get(&attribute_id) {
            create_color_map(attribute, selector).await
        } else {
            create_gray_map(attribute).await
        }
    } else {
        None
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    init_logging();

    //info!("Starting Map Editor");

    let map_storage = MapStorageWithSerde {};

    let map_generation = map_storage.read("../resources/map_generation/biome.yaml")?;
    let map = map_generation.generate();

    let selector_storage = SelectorStorageWithSerde {};

    let selectors = map
        .get_all()
        .iter()
        .enumerate()
        .filter_map(|(i, attribute)| {
            selector_storage
                .read(&format!(
                    "../resources/color_selector/{}.yaml",
                    attribute.name()
                ))
                .map(|s| (i, s))
                .ok()
        })
        .collect();

    if let Err(e) = rocket::build()
        .manage(EditorData { map, selectors })
        .mount(
            "/",
            routes![home, view_attribute, view_quad, get_map, get_color_map],
        )
        .attach(Template::fairing())
        .launch()
        .await
    {
        //warn!("Rocket didn't launch!");
        drop(e);
    };

    Ok(())
}

async fn create_gray_map(attribute: &Attribute) -> Option<NamedFile> {
    create_map(attribute, attribute.get_all(), ColorType::L8).await
}

async fn create_color_map(attribute: &Attribute, selector: &ColorSelector) -> Option<NamedFile> {
    let buf: Vec<u8> = attribute
        .get_all()
        .iter()
        .flat_map(|value| {
            let color = selector.get(*value);
            let array: [u8; 3] = color.into();
            array
        })
        .collect();

    create_map(attribute, &buf, ColorType::Rgb8).await
}

async fn create_map(attribute: &Attribute, buf: &[u8], color_type: ColorType) -> Option<NamedFile> {
    let path = get_map_path(attribute);
    let size = attribute.size();

    image::save_buffer(path.clone(), buf, size.width(), size.height(), color_type).unwrap();

    NamedFile::open(path).await.ok()
}

fn get_map_path(attribute: &Attribute) -> String {
    format!("../temp/map-{}.png", attribute.name())
}

fn get_attributes(map: &Map2d) -> Vec<(usize, &str)> {
    map.get_all()
        .iter()
        .enumerate()
        .map(|(i, a)| (i, a.name()))
        .collect()
}

fn get_attribute_name(map: &Map2d, attribute_id: usize) -> &str {
    map.get_attribute(attribute_id)
        .map(|a| a.name())
        .unwrap_or("Unknown")
}
