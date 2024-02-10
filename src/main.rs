use dialoguer::{theme::ColorfulTheme, Select};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let licenses = ["MIT", "Apache-2.0", "GPL-3.0"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license:")
        .items(&licenses)
        .default(0)
        .interact()
        .unwrap();

    let selected_license = licenses[selection];

    let license_text = match selected_license {
        "MIT" => include_str!("mit_license.txt"),
        "Apache-2.0" => include_str!("apache_license.txt"),
        "GPL-3.0" => include_str!("gpl_license.txt"),
        _ => panic!("Invalid license selected."),
    };

    if let Err(err) =
        File::create("LICENSE").and_then(|mut file| file.write_all(license_text.as_bytes()))
    {
        eprintln!("Error creating LICENSE file: {}", err);
    } else {
        println!(
            "A LICENSE file for the {} license was successfully created!",
            selected_license
        );
    }
}
