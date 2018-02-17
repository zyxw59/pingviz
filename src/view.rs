use cairo;
use gdk::ContextExt;
use gtk::{
    self,
    ContainerExt,
    WidgetExt,
};
use relm::{Relm, Update, Widget};

use stdin;
use util::Data;

/// Number of data points to display
const NUM_DATA_POINTS: usize = 120;

/// A `Model` stores information about the state of the program
#[derive(Clone,Debug)]
pub struct Model {
    /// x-coordinates of data plotted on the graph
    x: Data<f64>,
    /// y-coordinates of data plotted on the graph
    y: Data<f64>,
    /// A sum of the data points, for quick calculation of the average
    sum: f64,
    /// A sum of the squares of the data points, for quick calculation of the variance
    sqsum: f64,
}

impl Model {
    pub fn new() -> Model {
        Model {
            x: Data::with_capacity(NUM_DATA_POINTS),
            y: Data::with_capacity(NUM_DATA_POINTS),
            sum: 0.0,
            sqsum: 0.0,
        }
    }

    /// Clear the data
    pub fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        self.sum = 0.0;
        self.sqsum = 0.0;
    }

    /// Push the given `x`, `y` pair
    pub fn push(&mut self, x: f64, y: f64) {
        self.x.push(x);
        if let Some(old) = self.y.push(y) {
            self.sum -= old;
            self.sqsum -= old * old;
        }
        self.sum += y;
        self.sqsum += y * y;
    }

    /// Draw the graph to the provided `Context` with the given `width` and `height`
    pub fn draw(&self, ctx: &cairo::Context, width: f64, height: f64) {
        // pixels of padding around graph view
        let padding = 5.0;
        // draw background
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.rectangle(0.0, 0.0, width, height);
        ctx.fill();
        ctx.set_source_rgb(0.0, 0.0, 0.0);
        ctx.set_line_width(1.0);
        ctx.rectangle(padding, padding, width - 2.0 * padding, height - 2.0 * padding);
        ctx.stroke();

        if self.len() == 0 {
            return;
        }

        // we can unwrap here because as long as self.len() > 0, at least one (x, y) pair has been
        // pushed, so the bounds have been set
        let x_bounds = self.x.bounds().unwrap();
        let y_bounds = self.y.bounds().unwrap();

        // now use height and width of padded area
        let width = width - 2.0 * padding;
        let height = height - 2.0 * padding;

        // data -> screen coordinate conversion:
        // x_s = x_d * dx + x_0
        // y_s = y_d * dy + x_0

        let dx = if x_bounds.range() == 0.0 { 0.0 } else { width / x_bounds.range() };
        let dy = if y_bounds.range() == 0.0 { 0.0 } else { height / y_bounds.range() };

        // if either the x or y range is 0, center points on that axis (rather than placing them
        // along the edge)
        let x0 = padding + if dx == 0.0 { width / 2.0 } else { -x_bounds.min() * dx };
        let y0 = padding + if dy == 0.0 { height / 2.0 } else { -y_bounds.min() * dy };

        // draw line
        ctx.move_to(self.x.first().unwrap() * dx + x0, self.y.first().unwrap() * dy + y0);
        for (&x, &y) in self.x.iter().zip(self.y.iter()) {
            ctx.line_to(x * dx + x0, y * dy + y0);
        }
        ctx.stroke();

        // draw points
        ctx.set_line_width(6.0);
        ctx.set_line_cap(cairo::LineCap::Round);
        for (&x, &y) in self.x.iter().zip(self.y.iter()) {
            ctx.move_to(x * dx + x0, y * dy + y0);
            ctx.close_path();
        }
        ctx.stroke();
    }

    /// Return the mean of the `y` data
    pub fn mean(&self) -> f64 {
        self.sum / (self.x.len() as f64)
    }

    /// Return the variance of the `y` data
    pub fn var(&self) -> f64 {
        let n = self.len() as f64;
        self.sqsum / n - self.sum / n * self.sum / n
    }

    /// Return the standard deviation of the `y` data
    pub fn std(&self) -> f64 {
        self.var().sqrt()
    }

    /// Return the number of data points
    pub fn len(&self) -> usize {
        self.x.len()
    }
}

#[derive(Clone,Copy,Debug,Msg)]
pub enum Msg {
    Pass,
    Push(f64, f64),
    Quit,
}

#[derive(Clone)]
pub struct Win {
    model: Model,
    window: gtk::Window,
    graph: gtk::DrawingArea,
}

impl Win {
    /// Draws the graph
    fn draw(&mut self) {
        let ctx = cairo::Context::create_from_window(&self.graph.get_window().unwrap());
        let alloc = self.graph.get_allocation();
        self.model.draw(&ctx, alloc.width.into(), alloc.height.into());
    }
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        Model::new()
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let stream = stdin::stdin();
        relm.connect_exec_ignore_err(stream, |s| {
            let mut split = s.split_whitespace();
            let x = split.next().and_then(|s| s.parse().ok());
            let y = split.next().and_then(|s| s.parse().ok());
            match (x, y) {
                (Some(x), Some(y)) => Msg::Push(x, y),
                _ => Msg::Pass,
            }
        });
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Push(x, y) => {
                self.model.push(x, y);
                self.draw();
            },
            Msg::Quit => gtk::main_quit(),
            Msg::Pass => (),
        }
    }
}

impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let graph = gtk::DrawingArea::new();

        window.add(&graph);

        connect!(relm,
                 window,
                 connect_delete_event(_, _),
                 return (Some(Msg::Quit), gtk::Inhibit(false)));
        window.show_all();

        Win {
            model,
            window,
            graph,
        }
    }
}
