// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

const TAX_RATE: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.parse().unwrap();
            let tax = num * TAX_RATE;
            let owner = num * OWNERPER;
            let profit = num * PROFITPER;
            let opex = num * OPEXPER;
            let result: String = format!("Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpex: {:.2}", tax, owner, profit, opex);
            ui.set_results(result.into());
        }
    });

    ui.run()?;

    Ok(())
}
