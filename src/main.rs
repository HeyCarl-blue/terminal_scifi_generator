use scifi_generator::generator::*;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
enum BodyType {
    STAR,
    PLANET,
    SATELLITE
}

impl From<&str> for BodyType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "star" | "s" => BodyType::STAR,
            "planet" | "p" => BodyType::PLANET,
            "satellite" | "sat" => BodyType::SATELLITE,
            _ => BodyType::STAR
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        long = "body",
        short = "b",
        required = true,
        possible_values = &["star", "s", "planet", "p", "satellite", "sat"],
        help = "Type of the celestial body to generate",
        parse(from_str))
    ]
    body_type: BodyType,

    #[structopt(long = "name", short = "n", parse(from_str))]
    name: Option<String>,

    #[structopt(
        long = "class",
        short = "c",
        possible_values = &["O", "B", "A", "F", "G", "K", "M"],
        help = "Harvard Spectral classification of the star to generate",
        parse(from_str))
    ]
    star_class: Option<StarClass>,

    #[structopt(
        long = "type",
        short = "t",
        possible_values = &["terrestrial", "t", "gasgiant", "gg", "icegiant", "ig"],
        help = "Type of planet to generate",
        parse(from_str))
    ]
    planet_type: Option<PlanetType>,
}

fn main() {
    let args = Cli::from_args();

    let name = match args.name {
        Some(v) => v,
        None => "NONAME".to_string()
    };
    
    let body_type = args.body_type;

    match body_type {
        BodyType::STAR => {
            let star_class = args.star_class;
            match star_class {
                Some(v) => { println!("{}", Star::generate_from_class(&name, v)); },
                _ => { println!("{}", Star::generate(&name)); }
            }
        },
        BodyType::PLANET => {
            let planet_type = args.planet_type;
            match planet_type {
                Some(v) => { println!("{}", Planet::generate_from_type(&name, v)) },
                _ => { println!("{}", Planet::generate(&name)); }
            }
        },
        BodyType::SATELLITE => {
            println!("{}", Satellite::generate(&name));
        }
    }
}