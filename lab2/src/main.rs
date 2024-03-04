
mod apps;
mod controllers;
mod models;
mod views;

fn main() {
  apps::console_app::run_app();
  apps::gui_app::run_app();
}
