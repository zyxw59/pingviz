use gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{Relm, Update, Widget};

pub struct Model {
}

pub struct Win {
    model: Model,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        Model {
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        connect!(relm,
                 window,
                 connect_delete_event(_, _),
                 return (Some(Msg::Quit), Inhibit(false)));
        window.show_all();

        Win {
            model,
            window: window,
        }
    }
}
