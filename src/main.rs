use bevy::{color::palettes::css::PURPLE, prelude::*};
use rand::Rng;

const RENEW_MONEY: u32 = 2500;
const START_MONEY: u32 = 10000;
const AREAS_NUM: u8 = 44;

struct Player {
    area: u8,
    money: u32,
}

impl Player {
    pub fn move_areas(&mut self, value: i32) {
        let area = self.area + value as u8;
        if area > AREAS_NUM {
            self.money += RENEW_MONEY;
        }
        self.area = area % AREAS_NUM;
    }
}

struct Hotel {
    name: String,
    group: Hotels,
    upgrade: HotelUpgrade,
    owner: Option<Player>,
}

impl Hotel {
    pub fn new(name: String, group: Hotels) -> Self {
        Hotel {
            name,
            group,
            upgrade: HotelUpgrade::None,
            owner: None,
        }
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }
}

struct Restaurant {
    name: String,
    owner: Option<Player>,
}

impl Restaurant {
    pub fn new(name: String) -> Self {
        Restaurant { name, owner: None }
    }
}

struct Station {
    name: String,
    owner: Option<Player>,
}

impl Station {
    pub fn new(name: String) -> Self {
        Station { name, owner: None }
    }
}

struct Special {
    name: String,
    owner: Option<Player>,
}

impl Special {
    pub fn new(name: String) -> Self {
        Special { name, owner: None }
    }
}

enum HotelUpgrade {
    None,
    OneHause,
    DwoHause,
    ThreeHause,
    FourHause,
    OneHotel,
}

enum Hotels {
    Pink,
    Blue,
    Purple,
    Green,
    Orange,
    DarkGreen,
    Red,
    Yellow,
}

enum Areas {
    Start,
    Hotel(Hotel),
    Chance,
    Tax(Taxes),
    Restaurant(Restaurant),
    Draw,
    Station(Station),
    Special(Special),
    Hospital,
    Vacation,
    JumpHospital,
}

enum Taxes {
    IncomeTax,
    LucsusTax,
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}
fn main() {
    // let mut rng = rand::rng();
    // let value = rng.random_range(2..=12);
    // println!("{value}");

    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).run();
    

    let scandic_hotel = Hotel::new(String::from("Scandic"), Hotels::Purple);
    let royal_hotel = Hotel::new(String::from("Royal"), Hotels::Purple);
    let riviera_hotel = Hotel::new(String::from("Riviera"), Hotels::Green);
    let palace_hotel = Hotel::new(String::from("Palace"), Hotels::Green);
    let imperial_hotel = Hotel::new(String::from("Imperial"), Hotels::Green);
    let hyatt_hotel = Hotel::new(String::from("Hyatt"), Hotels::Pink);
    let de_ville_hotel = Hotel::new(String::from("de Ville"), Hotels::Pink);
    let maxim_hotel = Hotel::new(String::from("Maxim"), Hotels::Pink);
    let astoria_hotel = Hotel::new(String::from("Astoria"), Hotels::Blue);
    let forum_hotel = Hotel::new(String::from("Forum"), Hotels::Blue);
    let continental_hotel = Hotel::new(String::from("Continental"), Hotels::Blue);
    let radison_hotel = Hotel::new(String::from("Radison"), Hotels::Orange);
    let sheraton_hotel = Hotel::new(String::from("Sheraton"), Hotels::Orange);
    let holiday_inn_hotel = Hotel::new(String::from("Holiday INN"), Hotels::Orange);
    let mercure_hotel = Hotel::new(String::from("Mercure"), Hotels::DarkGreen);
    let intercontinental_hotel = Hotel::new(String::from("Intercontinental"), Hotels::DarkGreen);
    let victoria_hotel = Hotel::new(String::from("Victoria"), Hotels::DarkGreen);
    let plaza_hotel = Hotel::new(String::from("Plaza"), Hotels::Red);
    let savoy_hotel = Hotel::new(String::from("Savoy"), Hotels::Red);
    let marriott_hotel = Hotel::new(String::from("Marriott"), Hotels::Red);
    let ritz_hotel = Hotel::new(String::from("Ritz"), Hotels::Yellow);
    let hilton_hotel = Hotel::new(String::from("Hilton"), Hotels::Yellow);

    let schanghaj_restaurant = Restaurant::new(String::from("Schanghaj"));
    let cascade_restaurant = Restaurant::new(String::from("Cascade"));
    let corso_restaurant = Restaurant::new(String::from("Corso"));
    let paradise_restaurant = Restaurant::new(String::from("Paradise"));

    let termini_station = Station::new(String::from("Termini"));
    let waterloo_station = Station::new(String::from("Waterloo"));
    let victoria_station = Station::new(String::from("Victoria"));

    let power_station_special = Special::new(String::from("Power Station"));
    let filters_special = Special::new(String::from("Filters"));

    let board: [Areas; AREAS_NUM as usize] = [
        Areas::Start,
        Areas::Hotel(scandic_hotel),
        Areas::Chance,
        Areas::Hotel(royal_hotel),
        Areas::Tax(Taxes::IncomeTax),
        Areas::Station(termini_station),
        Areas::Restaurant(schanghaj_restaurant),
        Areas::Hotel(riviera_hotel),
        Areas::Draw,
        Areas::Hotel(palace_hotel),
        Areas::Hotel(imperial_hotel),
        Areas::Hospital,
        Areas::Hotel(hyatt_hotel),
        Areas::Special(power_station_special),
        Areas::Hotel(de_ville_hotel),
        Areas::Hotel(maxim_hotel),
        Areas::Draw,
        Areas::Restaurant(cascade_restaurant),
        Areas::Hotel(astoria_hotel),
        Areas::Chance,
        Areas::Hotel(forum_hotel),
        Areas::Hotel(continental_hotel),
        Areas::Vacation,
        Areas::Hotel(radison_hotel),
        Areas::Chance,
        Areas::Hotel(sheraton_hotel),
        Areas::Hotel(holiday_inn_hotel),
        Areas::Station(waterloo_station),
        Areas::Restaurant(corso_restaurant),
        Areas::Hotel(mercure_hotel),
        Areas::Hotel(intercontinental_hotel),
        Areas::Special(filters_special),
        Areas::Hotel(victoria_hotel),
        Areas::JumpHospital,
        Areas::Hotel(plaza_hotel),
        Areas::Hotel(savoy_hotel),
        Areas::Chance,
        Areas::Hotel(marriott_hotel),
        Areas::Station(victoria_station),
        Areas::Restaurant(paradise_restaurant),
        Areas::Draw,
        Areas::Hotel(ritz_hotel),
        Areas::Tax(Taxes::LucsusTax),
        Areas::Hotel(hilton_hotel),
    ];
}
