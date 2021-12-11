mod sorter;
mod helper;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use druid::widget::{prelude::*, Label, Flex};
use druid::{
    AppLauncher, Color, LocalizedString, Rect, WindowDesc, ExtEventSink, WidgetExt,
};

use druid::{Data, Env, Lens, Widget};
use helper::create_data;
use sorter::Swap;

const BAR_WIDTH: f64 = 40.;
const WINDOW_TITLE: LocalizedString<IterData> = LocalizedString::new("Graphs");


fn ui_builder() -> impl Widget<IterData> {
    
    let label = Label::new(|data: &IterData, _env: &_| format!("iterations: {}",data.iteration))
    .padding(5.0)
    .center();
    Flex::column().with_child(label).with_child(BoxPlot::default())

}
#[derive(Clone, Lens)]
struct IterData {
    name: String,
    iteration: Box<u32>,
    status: Rc<RefCell<Vec<f32>>>,
}

impl Data for IterData {
    fn same(&self, other: &Self) -> bool {
        self.iteration == other.iteration

    }
}

fn main() {
    let v = create_data(10);
    // describe the main window
    let main_window = WindowDesc::new( ui_builder())
        .title(WINDOW_TITLE)
        .window_size((1600.0, 800.0));

    // create the initial app state
    let initial_state = IterData {
        name: "World".into(),
        iteration: Box::new(0),
        status: Rc::new(RefCell::new(v)),

    };
    // start the application
    let launcher = AppLauncher::with_window(main_window);
        // .launch(initial_state)
        // .expect("Failed to launch application");
    let event_sink = launcher.get_external_handle();
    let d = initial_state.status.borrow().clone();
    thread::spawn(move || sort(event_sink,d));

    launcher
        .launch(initial_state)
        .expect("launch failed");

}
fn sort(event_sink: ExtEventSink, mut data: Vec<f32>) {
    let swaps = sorter::sort(&mut data);
    for swap in swaps {
        thread::sleep(Duration::from_secs_f32(0.2));

        event_sink.add_idle_callback(move |data: &mut IterData| {
            data.iteration = Box::new(data.iteration.as_ref() + 1);
            if let Swap::Atomic((i1,i2)) = swap {
                data.status.borrow_mut().swap(i1, i2);
            }
        });
    }
}
struct BoxPlot{
    color: Color,
}
impl BoxPlot {
    pub fn new(color: Color) -> BoxPlot {
        BoxPlot {color}
    }
}
impl Default for BoxPlot {
    fn default() -> BoxPlot {
        BoxPlot {color: Color::BLACK}
    }
}


impl Widget<IterData> for BoxPlot{
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut IterData, _env: &Env) {
      
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &IterData,
        _env: &Env,
    ) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &IterData, data: &IterData, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &IterData,
        _env: &Env,
    ) -> Size {
        let size = Size::new(1600.0, 800.0);
            bc.constrain(size)
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &IterData, _env: &Env) {
        let size = ctx.size();
        let ground_line = size.height - 50.;
      
        for (index,value) in data.status.borrow().iter().enumerate() {
            let rect = Rect::new(BAR_WIDTH * index as f64, ground_line, (BAR_WIDTH * (index + 1) as f64) - 10.,ground_line - *value as f64 * 20. - 20.);
            ctx.fill(rect, &self.color);
        }
    }
}

