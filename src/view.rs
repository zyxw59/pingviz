use gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{Relm, Update, Widget};

pub struct Model {
    data: Vec<(usize, usize)>,
    avg: usize,
}

impl Model {
    fn push(&mut self, seq: usize, time: usize) {
        self.data.push((seq, time));
        let n = self.data.len();
        self.avg = (self.avg * (n - 1) + time) / n;
    }
}

pub struct Win {
    model: Model,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Push(usize, usize),
    Quit,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        Model {
            data: Vec::new(),
            avg: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Push(seq, time) => {
                self.model.push(seq, time);
            },
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
