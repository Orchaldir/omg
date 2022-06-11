#[macro_use]
extern crate rocket;

use anyhow::Result;
use omg::data::map::Map2d;
use omg::interface::io::StoragePort;
use omg::logging::init_logging;
use omg_serde::interface::io::StoragePortWithSerde;
use rocket::{routes, State};
use std::sync::Mutex;

struct EditorData {
    map: Map2d,
}

type Data = State<Mutex<EditorData>>;

#[get("/")]
fn get_overview(data: &Data) -> String {
    let data = data.lock().expect("lock shared data");
    format!("{:?}", data.map)
}

#[rocket::main]
async fn main() -> Result<()> {
    init_logging();

    //info!("Starting Map Editor");

    let port = StoragePortWithSerde {};

    let map_generation = port.read("resources/map_generation/biome.yaml")?;
    let map = map_generation.generate();

    let data = EditorData { map };

    if let Err(e) = rocket::build()
        .manage(Mutex::new(data))
        .mount("/", routes![get_overview])
        .launch()
        .await
    {
        //warn!("Rocket didn't launch!");
        drop(e);
    };

    Ok(())
}
