mod game;

use std::io::{self, Write};

use game::{command::Command, state::GameState};
fn main() {

    let mut game = GameState::new(2);

    loop {
        println!("\nAktualny gracz: {}", game.current_player);
        println!("1 - RollDice");
        println!("2 - BuyProperty");
        println!("3 - EndTurn");
        println!("4 - Exit");

        print!("Wybierz opcję: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = match input.trim() {
            "1" => Some(Command::RollDice),
            "2" => Some(Command::BuyProperty),
            "3" => Some(Command::EndTurn),
            "4" => break,
            _ => {
                println!("Niepoprawna opcja");
                None
            }
        };

        if let Some(cmd) = command {
            if let Err(e) = game.apply(cmd) {
                println!("Błąd: {}", e);
            }
        }

        println!("{:#?}", game);
    }


    // let scandic_hotel = Hotel::new(String::from("Scandic"), Hotels::Purple);
    // let royal_hotel = Hotel::new(String::from("Royal"), Hotels::Purple);
    // let riviera_hotel = Hotel::new(String::from("Riviera"), Hotels::Green);
    // let palace_hotel = Hotel::new(String::from("Palace"), Hotels::Green);
    // let imperial_hotel = Hotel::new(String::from("Imperial"), Hotels::Green);
    // let hyatt_hotel = Hotel::new(String::from("Hyatt"), Hotels::Pink);
    // let de_ville_hotel = Hotel::new(String::from("de Ville"), Hotels::Pink);
    // let maxim_hotel = Hotel::new(String::from("Maxim"), Hotels::Pink);
    // let astoria_hotel = Hotel::new(String::from("Astoria"), Hotels::Blue);
    // let forum_hotel = Hotel::new(String::from("Forum"), Hotels::Blue);
    // let continental_hotel = Hotel::new(String::from("Continental"), Hotels::Blue);
    // let radison_hotel = Hotel::new(String::from("Radison"), Hotels::Orange);
    // let sheraton_hotel = Hotel::new(String::from("Sheraton"), Hotels::Orange);
    // let holiday_inn_hotel = Hotel::new(String::from("Holiday INN"), Hotels::Orange);
    // let mercure_hotel = Hotel::new(String::from("Mercure"), Hotels::DarkGreen);
    // let intercontinental_hotel = Hotel::new(String::from("Intercontinental"), Hotels::DarkGreen);
    // let victoria_hotel = Hotel::new(String::from("Victoria"), Hotels::DarkGreen);
    // let plaza_hotel = Hotel::new(String::from("Plaza"), Hotels::Red);
    // let savoy_hotel = Hotel::new(String::from("Savoy"), Hotels::Red);
    // let marriott_hotel = Hotel::new(String::from("Marriott"), Hotels::Red);
    // let ritz_hotel = Hotel::new(String::from("Ritz"), Hotels::Yellow);
    // let hilton_hotel = Hotel::new(String::from("Hilton"), Hotels::Yellow);

    // let schanghaj_restaurant = Restaurant::new(String::from("Schanghaj"));
    // let cascade_restaurant = Restaurant::new(String::from("Cascade"));
    // let corso_restaurant = Restaurant::new(String::from("Corso"));
    // let paradise_restaurant = Restaurant::new(String::from("Paradise"));

    // let termini_station = Station::new(String::from("Termini"));
    // let waterloo_station = Station::new(String::from("Waterloo"));
    // let victoria_station = Station::new(String::from("Victoria"));

    // let power_station_special = Special::new(String::from("Power Station"));
    // let filters_special = Special::new(String::from("Filters"));

    // let board: [Areas; AREAS_NUM as usize] = [
    //     Areas::Start,
    //     Areas::Hotel(scandic_hotel),
    //     Areas::Chance,
    //     Areas::Hotel(royal_hotel),
    //     Areas::Tax(Taxes::IncomeTax),
    //     Areas::Station(termini_station),
    //     Areas::Restaurant(schanghaj_restaurant),
    //     Areas::Hotel(riviera_hotel),
    //     Areas::Draw,
    //     Areas::Hotel(palace_hotel),
    //     Areas::Hotel(imperial_hotel),
    //     Areas::Hospital,
    //     Areas::Hotel(hyatt_hotel),
    //     Areas::Special(power_station_special),
    //     Areas::Hotel(de_ville_hotel),
    //     Areas::Hotel(maxim_hotel),
    //     Areas::Draw,
    //     Areas::Restaurant(cascade_restaurant),
    //     Areas::Hotel(astoria_hotel),
    //     Areas::Chance,
    //     Areas::Hotel(forum_hotel),
    //     Areas::Hotel(continental_hotel),
    //     Areas::Vacation,
    //     Areas::Hotel(radison_hotel),
    //     Areas::Chance,
    //     Areas::Hotel(sheraton_hotel),
    //     Areas::Hotel(holiday_inn_hotel),
    //     Areas::Station(waterloo_station),
    //     Areas::Restaurant(corso_restaurant),
    //     Areas::Hotel(mercure_hotel),
    //     Areas::Hotel(intercontinental_hotel),
    //     Areas::Special(filters_special),
    //     Areas::Hotel(victoria_hotel),
    //     Areas::JumpHospital,
    //     Areas::Hotel(plaza_hotel),
    //     Areas::Hotel(savoy_hotel),
    //     Areas::Chance,
    //     Areas::Hotel(marriott_hotel),
    //     Areas::Station(victoria_station),
    //     Areas::Restaurant(paradise_restaurant),
    //     Areas::Draw,
    //     Areas::Hotel(ritz_hotel),
    //     Areas::Tax(Taxes::LucsusTax),
    //     Areas::Hotel(hilton_hotel),
    // ];
}
