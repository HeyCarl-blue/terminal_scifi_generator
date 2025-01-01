pub mod generator {
    use rand:: {
        distributions::{Distribution, Standard}, thread_rng, Rng
    };
    use std::fmt;
    use structopt::StructOpt;

    #[derive(Debug, Clone, Copy, StructOpt)]
    pub enum StarClass {
        O,
        B,
        A,
        F,
        G,
        K,
        M
    }

    impl StarClass {
        pub fn color(self) -> &'static str {
            match self {
                StarClass::O => "\x1b[38;2;0;102;196m",
                StarClass::B => "\x1b[38;2;68;199;255m",
                StarClass::A => "\x1b[38;2;255;255;255m",
                StarClass::F => "\x1b[38;2;254;252;150m",
                StarClass::G => "\x1b[38;2;254;254;76m",
                StarClass::K => "\x1b[38;2;254;207;81m",
                StarClass::M => "\x1b[38;2;254;85;43m",
            }
        }

        pub fn color_str(self) -> &'static str {
            match self {
                StarClass::O => "Blue star",
                StarClass::B => "Light blue star",
                StarClass::A => "White star",
                StarClass::F => "Yellow-white star",
                StarClass::G => "Yellow star",
                StarClass::K => "Orange star",
                StarClass::M => "Red star",
            }
        }
    }

    impl From<&str> for StarClass {
        fn from(s: &str) -> Self {
            match s.to_uppercase().as_str() {
                "O" => StarClass::O,
                "B" => StarClass::B,
                "A" => StarClass::A,
                "F" => StarClass::F,
                "G" => StarClass::G,
                "K" => StarClass::K,
                "M" => StarClass::M,
                _ => StarClass::G,  // Default case
            }
        }
    }

    impl Distribution<StarClass> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StarClass {
            match rng.gen_range(0..=6) {
                0 => StarClass::O,
                1 => StarClass::B,
                2 => StarClass::A,
                3 => StarClass::F,
                4 => StarClass::G,
                5 => StarClass::K,
                _ => StarClass::M
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Star<'a> {
        pub name: &'a str,
        pub class: StarClass,
        pub diameter: f64,
        pub temp: f64,
    }

    impl<'a> Star<'a> {
        pub fn new(name: &'a str, class: StarClass, diameter: f64, temp: f64) -> Star<'a> {
            Star { name, class, diameter, temp }
        }

        fn rand_temp_from_class(class: StarClass) -> f64 {
            let mut rng = thread_rng();
            match class {
                StarClass::O => rng.gen_range(33_000.0..70_000.0),
                StarClass::B => rng.gen_range(10_000.0..33_000.0),
                StarClass::A => rng.gen_range(7_500.0..10_000.0),
                StarClass::F => rng.gen_range(6_000.0..7_500.0),
                StarClass::G => rng.gen_range(5_200.0..6_000.0),
                StarClass::K => rng.gen_range(3_700.0..5_200.0),
                StarClass::M => rng.gen_range(1_000.0..3_700.0),
            }
        }

        fn rand_diameter_from_class(class: StarClass) -> f64 {
            let mut rng = thread_rng();
            let mult = match class {
                StarClass::O => rng.gen_range(6.6..7.0),
                StarClass::B => rng.gen_range(1.8..6.6),
                StarClass::A => rng.gen_range(1.4..1.8),
                StarClass::F => rng.gen_range(1.15..1.4),
                StarClass::G => rng.gen_range(0.96..1.15),
                StarClass::K => rng.gen_range(0.7..0.96),
                StarClass::M => rng.gen_range(0.1..0.7),
            };
            mult * Star::SUN.diameter
        }

        fn class_from_temp(temp: f64) -> StarClass {
            if temp < 3_700.0 {
                StarClass::M
            } else if (3_700.0..5_200.0).contains(&temp) {
                StarClass::K
            } else if (5_200.0..6_000.0).contains(&temp) {
                StarClass::G
            } else if (6_000.0..7_500.0).contains(&temp) {
                StarClass::F
            } else if (7_500.0..10_000.0).contains(&temp) {
                StarClass::A
            } else if (10_000.0..33_000.0).contains(&temp) {
                StarClass::B
            } else {
                StarClass::O
            }
        }

        fn class_from_diameter(diameter: f64) -> StarClass {
            let d = diameter / Star::SUN.diameter;
            if d < 0.7 {
                StarClass::M
            } else if (0.7..0.96).contains(&d) {
                StarClass::K
            } else if (0.96..1.15).contains(&d) {
                StarClass::G
            } else if (1.15..1.4).contains(&d) {
                StarClass::F
            } else if (1.4..1.8).contains(&d) {
                StarClass::A
            } else if (1.8..6.6).contains(&d) {
                StarClass::B
            } else {
                StarClass::O
            }
        }

        pub fn generate(name: &'a str) -> Star<'a> {
            let class = rand::random::<StarClass>();
            Star::generate_from_class(name, class)
        }

        pub fn generate_from_class(name: &'a str, class: StarClass) -> Star<'a> {
            let diameter = Star::rand_diameter_from_class(class);
            let temp = Star::rand_temp_from_class(class);
            Star::new(name, class, diameter, temp)
        }

        pub fn generate_from_temp(name: &'a str, temp: f64) -> Star<'a> {
            let class = Star::class_from_temp(temp);
            let diameter = Star::rand_diameter_from_class(class);
            Star::new(name, class, diameter, temp)
        }

        pub fn generate_from_diameter(name: &'a str, diameter: f64) -> Star<'a> {
            let class = Star::class_from_diameter(diameter);
            let temp: f64 = Star::rand_temp_from_class(class);
            Star::new(name, class, diameter, temp)
        }

        fn respect_to_sun(self) -> [f64 ; 2] {
            [ self.diameter / Star::SUN.diameter, self.temp / Star::SUN.temp ]
        }
    }

    impl<'a> Star<'_> {
        pub const SUN: Star<'static> = Star { name: "Sun", class: StarClass::G, diameter: 1.39095_E+6, temp: 5777.0 };
    }

    impl<'a> fmt::Display for Star<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const FRAME: i32 = 10;
            let radius = FRAME as f64 * self.diameter / (7.0 * Star::SUN.diameter);
            let ri = if radius < 4.0 { 4.0 } else if radius > FRAME as f64 { FRAME as f64 } else { radius as f64 } as i32;

            const SIZE: usize = (FRAME * 2 + 1) as usize * (FRAME * 2 + 1) as usize;
            let mut chars: Vec<String> = Vec::with_capacity(SIZE+1);

            for y in -FRAME..=FRAME {
                for x in -FRAME..=FRAME {
                    if x*x + y*y - ri * ri < 0 {
                        chars.push(format!("{}█\x1b[0m", self.class.color()));
                    } else {
                        chars.push(String::from(" ".to_string()));
                    }
                }
                chars.push("\n".to_string());
            }

            let to_sun = self.respect_to_sun();

            write!(f, "{}\n{}:\n-- Star Class: {:?}\n-- {}\n-- Diameter: {:.3e} km  ==>  {:.2} of sun\n-- Surface Temperature: {:.3e} ˚K -- {:.3e} ˚C   ==>   {:.2} of sun",
                chars.concat(),
                self.name,
                self.class,
                self.class.color_str(),
                self.diameter,
                to_sun[0],
                self.temp,
                self.temp - 273.15,
                to_sun[1]
            )
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Satellite<'a> {
        pub name: &'a str,
        pub diameter: f64,
        pub orbital_distance: f64,
    }

    impl<'a> Satellite<'a> {
        pub fn new(name: &'a str, diameter: f64, orbital_distance: f64) -> Satellite<'a> {
            Satellite { name, diameter, orbital_distance }
        }

        pub fn generate(name: &'a str) -> Satellite<'a> {
            let mut rng = thread_rng();
            let diameter = rng.gen_range(5.0..8_000.0);
            let orbital_distance = rng.gen_range(1_000.0..1_000_000.0);

            Satellite::new(name, diameter, orbital_distance)
        }

        pub fn respect_to_luna(self) -> [f64; 2] {
            [
                self.diameter / Satellite::LUNA.diameter,
                self.orbital_distance / Satellite::LUNA.orbital_distance
            ]
        }
    }

    impl<'a> Satellite<'_> {
        pub const LUNA: Satellite<'static> = Satellite { name: "Luna", diameter: 3_474.0, orbital_distance: 384_400.0 };
    }

    impl<'a> fmt::Display for Satellite<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let to_luna = self.respect_to_luna();
            write!(f, "{}:\n-- Diameter: {:.3e} km  ==>  {:.2} of luna\n-- Orbital Distance: {:.3e} km  ==>  {:.2} of luna",
                self.name,
                self.diameter,
                to_luna[0],
                self.orbital_distance,
                to_luna[1]
            )
        }
    }

    #[derive(Debug, Clone, Copy, StructOpt)]
    pub enum PlanetType {
        TERRESTRIAL,
        GASGIANT,
        ICEGIANT,
        // EXOPLANET,
    }

    impl Distribution<PlanetType> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetType {
            match rng.gen_range(0..=2) {
                0 => PlanetType::TERRESTRIAL,
                1 => PlanetType::GASGIANT,
                _ => PlanetType::ICEGIANT,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Planet<'a> {
        pub name: &'a str,
        pub planet_type: PlanetType,
        pub diameter: f64,
        pub mass: f64,
        pub orbital_period: f64,
    }

    impl<'a> Planet<'a> {
        pub fn new(name: &'a str, planet_type: PlanetType, diameter: f64, mass: f64, orbital_period: f64) -> Planet<'a> {
            Planet { name, planet_type, diameter, mass, orbital_period }
        }

        pub fn generate_from_type(name: &'a str, planet_type: PlanetType) -> Planet<'a> {
            let mut rng = thread_rng();
            let diameter = match planet_type {
                PlanetType::TERRESTRIAL => rng.gen_range(4_000.0..20_000.0),
                PlanetType::GASGIANT => rng.gen_range(45_000.0..150_000.0),
                PlanetType::ICEGIANT => rng.gen_range(45_000.0..60_000.0),
            };
            let mass = match planet_type {
                PlanetType::TERRESTRIAL => rng.gen_range(2_E+22..2.9_E+25),
                PlanetType::GASGIANT => rng.gen_range(5_E+26..3_E+28),
                PlanetType::ICEGIANT => rng.gen_range(8_E+26..3.6_E+28),
            };
            let orbital_period = match planet_type {
                PlanetType::TERRESTRIAL => rng.gen_range(70.0..750.0),
                PlanetType::GASGIANT => rng.gen_range(3_000.0..80_000.0),
                PlanetType::ICEGIANT => rng.gen_range(3_000.0..80_000.0),
            };

            Planet::new(name, planet_type, diameter, mass, orbital_period)
        }

        pub fn generate_from_type_and_diameter(name: &'a str, planet_type: PlanetType, diameter: f64) -> Planet<'a> {
            let mut rng = thread_rng();
            let mass = rng.gen_range(2_E+22..2.9_E+25);
            let orbital_period = rng.gen_range(70.0..750.0);

            Planet::new(name, planet_type, diameter, mass, orbital_period)
        }

        pub fn generate(name: &'a str) -> Planet<'a> {
            let planet_type = rand::random::<PlanetType>();
            Planet::generate_from_type(name, planet_type)
        }

        pub fn surface_gravity(self) -> f64 {
            const G: f64 = 6.67_E-11;
            let radius = self.diameter * 0.5;
            G * self.mass / (radius * radius) * 1_E-6
        }

        pub fn orbital_velocity(self) -> f64 {
            let secs = self.orbital_period * 86_400.0;
            self.diameter / secs
        }

        fn respect_to_earth(self) -> [f64; 4] {
            [
                self.diameter / Planet::EARTH.diameter,
                self.mass / Planet::EARTH.mass,
                self.orbital_period / Planet::EARTH.orbital_period,
                self.surface_gravity() / Planet::EARTH.surface_gravity()
            ]
        }
    }

    impl<'a> Planet<'_> {
        pub const EARTH: Planet<'static> = Planet {
            name: "Earth",
            planet_type: PlanetType::TERRESTRIAL,
            diameter: 12_745.274,
            mass: 5.9726_E+24,
            orbital_period: 365.0
        };
    }

    impl<'a> fmt::Display for Planet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let to_earth = self.respect_to_earth();
            write!(f, "{}:\n-- Planet Type: {:?}\n-- Diameter: {:.3e} km   ==>   {:.2} of earth\n-- Mass: {:.3e} kg   ==>   {:.2} of earth\n-- Orbital Period: {:.2} days   ==>   {:.2} of earth\n-- Surface Gravity: {:.2} m/s^2   ==>   {:.2} g",
                self.name,
                self.planet_type,
                self.diameter,
                to_earth[0],
                self.mass,
                to_earth[1],
                self.orbital_period,
                to_earth[2],
                self.surface_gravity(),
                to_earth[3]
            )
        }
    }

    impl From<&str> for PlanetType {
        fn from(s: &str) -> Self {
            match s.to_lowercase().as_str() {
                "terrestrial" | "rocky" => PlanetType::TERRESTRIAL,
                "gas" | "gas giant" => PlanetType::GASGIANT,
                "ice" | "ice giant" => PlanetType::ICEGIANT,
                _ => PlanetType::TERRESTRIAL,  // Default case
            }
        }
    }
}