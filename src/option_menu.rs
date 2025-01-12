extern crate skim;
use skim::prelude::*;
use std::io::Cursor;

fn main() {
    // First menu options
    let main_menu = "Users\nGroups\nApps";

    // Run the main menu
    let selected = run_skim_menu(main_menu.to_string(), "Select an option:".to_string());

    match selected.as_str() {
        "Users" => {
            let users = "User1\nUser2\nUser3";
            run_skim_menu(users.to_string(), "Select a user:".to_string());
        }
        "Groups" => {
            let groups = "Group1\nGroup2\nGroup3";
            run_skim_menu(groups.to_string(), "Select a group:".to_string());
        }
        "Apps" => {
            let apps = "App1\nApp2\nApp3";
            run_skim_menu(apps.to_string(), "Select an app:".to_string());
        }
        _ => println!("Invalid selection"),
    }
}

fn run_skim_menu(input: String, prompt: String) -> String {
    let options = SkimOptionsBuilder::default()
        .height("50%".to_string())
        .prompt(prompt)
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    selected_items
        .get(0)
        .map(|item| item.output().to_string())
        .unwrap_or_else(String::new)
}
