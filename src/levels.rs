use crate::component::block;
use crate::VisualComponent;
use ggez::*;
use std::fs::File;
use std::io::Read;
use yaml_rust::*;

pub enum LevelShape {
    Hexagonal,
}

impl LevelShape {
    fn from_str(input: &str) -> LevelShape {
        match input {
            "Hexagonal" => LevelShape::Hexagonal,
            _ => panic!("Invalid level shape"),
        }
    }
}

pub struct Level {
    pub name: String,
    pub shape: LevelShape,
    pub size: usize,
    pub blocks: Vec<block::Hexagon>,
}

impl Level {
    pub fn new(file_path: String) -> Self {
        let mut f = File::open(file_path).expect("Unable to open level file");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Unable to read level file");
        let yaml = YamlLoader::load_from_str(&contents).unwrap();
        let level = &yaml[0]["level"];
        Level {
            name: String::from(level["name"].as_str().expect("Missing property: name")),
            shape: LevelShape::from_str(level["shape"].as_str().expect("Missing property: shape")),
            size: level["size"].as_i64().expect("Missing property: size") as usize,
            blocks: parse_blocks(level),
        }
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for hexagon in self.blocks.iter() {
            hexagon.draw(ctx)?;
        }
        for hexagon in self.blocks.iter() {
            hexagon.draw_trace(ctx)?;
        }
        Ok(())
    }
}

fn parse_blocks(yaml: &yaml_rust::Yaml) -> Vec<block::Hexagon> {
    let mut blocks = Vec::new();
    if !yaml["blocks"].is_array() {
        panic! {"Invalid property: blocks"}
    }
    for node in yaml["blocks"].as_vec().unwrap() {
        let index = block::GridIndex {
            q: node["q"].as_i64().expect("Missing property: block index") as i32,
            r: node["r"].as_i64().expect("Missing property: block index") as i32,
        };
        let tile_radius = 20.0; // TODO: temporary!, add relative radius or adapt to screen size
        let point = index.to_pixel(tile_radius);
        blocks.push(block::Hexagon {
            x: point.x,
            y: point.y,
            r: tile_radius,
            phi: 0.0,
            block_type: block::BlockType::from_str(
                node["block"]
                    .as_str()
                    .expect("Missing property: block type"),
            ),
        });
    }
    blocks
}
