use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::fs::{self, File};
use std::io::{self, prelude::*};

fn main() {
    let licenses = ["MIT", "Apache-2.0", "GPL-3.0", "Unlicense"];

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
        "Unlicense" => include_str!("unlicense.txt"),
        _ => panic!("Invalid license selected."),
    };

    let license_file_path = "LICENSE";

    if fs::metadata(license_file_path).is_ok() {
        // License file already exists
        let replace_existing = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("A LICENSE file already exists. Do you want to replace it?")
            .interact()
            .unwrap();

        if !replace_existing {
            println!("Operation canceled. No changes made.");
            return;
        }
    }

    match File::create(license_file_path).and_then(|mut file| file.write_all(license_text.as_bytes())) {
        Ok(_) => {
            println!(
                "A LICENSE file for the {} license was successfully created!\nMake sure you replace unique values (your name for example)",
                selected_license
            );
        }
        Err(err) => eprintln!("Error creating LICENSE file: {}", err),
    }
}
