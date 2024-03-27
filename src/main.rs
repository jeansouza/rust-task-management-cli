use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_owned()));
    let mut navigator = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:

        // 1. get current page from navigator. If there is no current page exit the loop.
        let currrent_page_opt = navigator.get_current_page();
        if currrent_page_opt.is_none() {
            break;
        }
        let currrent_page = currrent_page_opt.unwrap();

        // 2. render page
        if let Err(error) = currrent_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
        };

        // 3. get user input
        let input = get_user_input();

        // 4. pass input to page's input handler
        let handle_result = currrent_page.handle_input(&input);

        match handle_result {
            Ok(action) => {
                // 5. if the page's input handler returns an action let the navigator process the action
                if action.is_some() {
                    if let Err(error) = navigator.handle_action(action.unwrap()) {
                        println!(
                            "Error handling action: {}\nPress any key to continue...",
                            error
                        );
                        wait_for_key_press();
                    };
                }
            }
            Err(error) => {
                println!(
                    "Error handling input: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            }
        }
    }
}
