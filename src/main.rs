extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod view;

use view::Win;

fn main() {
    Win::run(()).unwrap();
}
