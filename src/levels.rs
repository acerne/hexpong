use crate::component::block;
use crate::themes;
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
    pub block_size: f32,
    pub blocks: Vec<block::Hexagon>,
}

impl Level {
    pub fn new(file_path: String) -> Self {
        let mut f = File::open(file_path).expect("Unable to open level file");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Unable to read level file");
        let yaml = &YamlLoader::load_from_str(&contents).unwrap()[0]["level"];
        let block_size = yaml["block-size"]
            .as_f64()
            .expect("Missing property: block-size") as f32;
        Level {
            name: String::from(yaml["name"].as_str().expect("Missing property: name")),
            shape: LevelShape::from_str(yaml["shape"].as_str().expect("Missing property: shape")),
            block_size: block_size,
            blocks: parse_blocks(yaml, block_size),
        }
    }
    pub fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        for hexagon in self.blocks.iter() {
            hexagon.draw(ctx, theme)?;
        }
        Ok(())
    }
}

fn parse_blocks(yaml: &yaml_rust::Yaml, block_size: f32) -> Vec<block::Hexagon> {
    let mut blocks = Vec::new();
    if !yaml["blocks"].is_array() {
        panic! {"Invalid property: blocks"}
    }
    for node in yaml["blocks"].as_vec().unwrap() {
        let index = block::GridIndex {
            q: node["q"].as_i64().expect("Missing property: block index") as i32,
            r: node["r"].as_i64().expect("Missing property: block index") as i32,
        };
        let point = index.to_unit(block_size);
        blocks.push(block::Hexagon::new(
            point.x,
            point.y,
            block_size,
            block::BlockType::from_str(
                node["block"]
                    .as_str()
                    .expect("Missing property: block type"),
            ),
        ));
    }
    blocks
}
