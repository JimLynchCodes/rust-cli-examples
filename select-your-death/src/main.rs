use inquire::{Select,InquireError};

fn main() {
    let options: Vec<&str> = vec![
        "Drowning",
        "Torture",
        "Starving",
        "Cancer",
        "Freezing",
        "Crucifixion",
        "Heart Attack",
        "Falling from a building",
        "Suffocation",
        "Poison",
        "Electrification",
        "Shark bite",
        "Roundhouse kick from Chuck Norris",
        "Airplane crash",
        "Car Accident",
        "Snake Bite",
        "Bomb explosion",
        "Falling into a volcano",
        "Choking",
        "Drug overdose",
        "Beheading",
        "Lost at sea",
        "Beating with a cucumber"
    ];

    let ans: Result<&str, InquireError> =
        Select::new("What's your preferred way to die?", options).prompt();

    match ans {
        Ok(choice) => println!("{}! Okay... well, this is weird!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
}
