use crate::component::block;
use ggez::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::*;

pub struct Theme {
    pub background: graphics::Color,
    pub wall: graphics::Color,
    pub player1: graphics::Color,
    pub player2: graphics::Color,
    pub player3: graphics::Color,
    blocks: HashMap<block::BlockType, graphics::Color>,
}

impl Theme {
    pub fn new(file_path: String) -> Self {
        let mut f = File::open(file_path).expect("Unable to open theme file");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Unable to read theme file");
        let yaml = &YamlLoader::load_from_str(&contents).unwrap()[0]["theme"];
        Theme {
            background: decode_color(
                yaml["background"]
                    .as_str()
                    .expect("Missing property: background"),
            ),
            wall: decode_color(yaml["wall"].as_str().expect("Missing property: wall")),
            player1: decode_color(yaml["player1"].as_str().expect("Missing property: player1")),
            player2: decode_color(yaml["player2"].as_str().expect("Missing property: player2")),
            player3: decode_color(yaml["player3"].as_str().expect("Missing property: player3")),
            blocks: parse_blocks(yaml),
        }
    }
    pub fn get_block_color(&self, block_type: &block::BlockType) -> graphics::Color {
        self.blocks[&block_type]
    }
}

fn decode_color(s: &str) -> graphics::Color {
    if s.len() == 6 {
        graphics::Color::from_rgb(
            u8::from_str_radix(&s[0..2], 16).expect("Unable to decode color"),
            u8::from_str_radix(&s[2..4], 16).expect("Unable to decode color"),
            u8::from_str_radix(&s[4..6], 16).expect("Unable to decode color"),
        )
    } else if s.len() == 8 {
        graphics::Color::from_rgba(
            u8::from_str_radix(&s[0..2], 16).expect("Unable to decode color"),
            u8::from_str_radix(&s[2..4], 16).expect("Unable to decode color"),
            u8::from_str_radix(&s[4..6], 16).expect("Unable to decode color"),
            u8::from_str_radix(&s[6..8], 16).expect("Unable to decode color"),
        )
    } else {
        panic!("Unable to decode color")
    }
}

fn parse_blocks(yaml: &yaml_rust::Yaml) -> HashMap<block::BlockType, graphics::Color> {
    let mut blocks = HashMap::new();
    if !yaml["blocks"].is_array() {
        panic! {"Invalid property: blocks"}
    }
    for node in yaml["blocks"].as_vec().unwrap() {
        let block_type = block::BlockType::from_str(
            node["block"]
                .as_str()
                .expect("Missing property: block type"),
        );
        let block_color = decode_color(node["color"].as_str().expect("Missing property: color"));
        blocks.insert(block_type, block_color);
    }
    blocks
}
