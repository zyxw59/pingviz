extern crate cairo;
extern crate futures;
extern crate futures_glib;
extern crate gdk;
extern crate gtk;
extern crate rand;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod stdin;
mod util;
mod view;

use view::Win;

fn main() {
    Win::run(()).unwrap();
}
