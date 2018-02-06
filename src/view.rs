use gtk;
#[allow(unused_imports)]
use gtk::{
    Inhibit,
    LabelExt,
    WidgetExt,
};
use relm::Widget;

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

#[derive(Msg)]
pub enum Msg {
    Push(usize, usize),
    Quit,
}

relm_widget! {
    impl Widget for Win {
        fn model() -> Model {
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

        view!{
            gtk::Window {
                gtk::Box {
                    gtk::Label {
                        text: &self.model.avg.to_string(),
                    },
                },
                delete_event(_, _) => (Msg::Quit, Inhibit(false)),
            }
        }
    }
}
