use crate::component::player;
use crate::component::wall;
use crate::levels;
use std::fs::File;
use std::io::Read;
use yaml_rust::*;

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub enum Controller {
    Player1,
    Player2,
    Player3,
    Wall,
}

#[derive(Clone)]
pub enum Side {
    Bottom,
    BottomLeft,
    BottomRight,
    Top,
    TopLeft,
    TopRight,
}

pub struct GameMode {
    pub players: Vec<player::Player>,
    pub walls: Vec<wall::Wall>,
    pub levels: Vec<levels::Level>,
}

impl GameMode {
    pub fn new(file_path: &str, difficulty: Difficulty) -> Self {
        let mut f = File::open(file_path).expect("Unable to open gamemode file");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Unable to read gamemode file");
        let yaml = &YamlLoader::load_from_str(&contents).unwrap()[0]["gamemode"];

        let bar_size = yaml["difficulty"][difficulty.to_str()]["bar-size"]
            .as_f64()
            .expect("Missing bar-size property") as f32;

        let mut players = Vec::new();
        let mut walls = Vec::new();
        let mut player1 = player::Player::new(bar_size);

        for side in [
            Side::Bottom,
            Side::BottomLeft,
            Side::BottomRight,
            Side::Top,
            Side::TopLeft,
            Side::TopRight,
        ]
        .iter()
        {
            let input = yaml["difficulty"][difficulty.to_str()]["controls"][side.to_str()]["input"]
                .as_str()
                .expect("Missing or invalid difficulty configuration");
            match Controller::from_str(input) {
                Controller::Player1 => {
                    player1.bars.push(player::Bar::new(side, bar_size));
                }
                Controller::Player2 => {}
                Controller::Player3 => {}
                Controller::Wall => {
                    walls.push(wall::Wall::new(side));
                }
            }
        }
        players.push(player1);

        GameMode {
            players: players,
            walls: walls,
            levels: Vec::new(),
        }
    }
}

impl Controller {
    pub fn from_str(input: &str) -> Controller {
        match &input.to_lowercase()[..] {
            "player1" => Controller::Player1,
            "player2" => Controller::Player2,
            "player3" => Controller::Player3,
            "wall" => Controller::Wall,
            _ => panic!("Invalid controller"),
        }
    }
}

impl Side {
    pub fn from_str(input: &str) -> Side {
        match &input.to_lowercase()[..] {
            "bottom" => Side::Bottom,
            "bottom-left)" => Side::BottomLeft,
            "bottom-right" => Side::BottomRight,
            "top" => Side::Top,
            "top-left)" => Side::TopLeft,
            "top-right" => Side::TopRight,
            _ => panic!("Invalid side"),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Side::Bottom => "bottom",
            Side::BottomLeft => "bottom-left",
            Side::BottomRight => "bottom-right",
            Side::Top => "top",
            Side::TopLeft => "top-left",
            Side::TopRight => "top-right",
        }
    }
    pub fn to_ang(&self) -> f32 {
        match self {
            Side::Bottom => 60.0,
            Side::BottomLeft => 120.0,
            Side::BottomRight => 0.0,
            Side::Top => 240.0,
            Side::TopLeft => 180.0,
            Side::TopRight => 300.0,
        }
    }
}

impl Difficulty {
    pub fn from_str(input: &str) -> Difficulty {
        match &input.to_lowercase()[..] {
            "easy" => Difficulty::Easy,
            "normal" => Difficulty::Normal,
            "hard" => Difficulty::Hard,
            _ => panic!("Invalid difficulty"),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Hard => "hard",
        }
    }
}
