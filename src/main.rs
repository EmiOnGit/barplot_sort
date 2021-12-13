mod sorter;
mod helper;
mod barplot;
mod delegate;

use std::sync::{Arc, Mutex};
use std::thread;
use barplot::BoxPlot;
use delegate::Delegate;
use druid::widget::{Label, Flex};
use druid::{
    AppLauncher, LocalizedString, WindowDesc, WidgetExt, Selector,
};

use druid::{Data, Lens, Widget};
use helper::create_data;
use sorter::Swap;
const FINISH_SORTING: Selector<Swap> = Selector::new("finish sorting the data");

const BAR_WIDTH: f64 = 40.;
const WINDOW_TITLE: LocalizedString<BarPlotData> = LocalizedString::new("Graphs");


fn ui_builder(data: Arc<Mutex<Vec<f32>>>) -> impl Widget<BarPlotData> {
    let label = Label::new(|data: &BarPlotData, _env: &_| format!("iterations: {:?}",data.swap))
    .padding(5.0)
    .center();
    Flex::column().with_child(label).with_child(BoxPlot::new(data))

}

#[derive(Clone, Lens, Data)]
struct BarPlotData {
    #[data(eq)]
    swap: Option<Swap>
}

fn main() {
    let data = create_data(20);
    
    let ui = ui_builder(data.clone());
    
    // describe the main window
    let main_window = WindowDesc::new( ui)
        .title(WINDOW_TITLE)
        .window_size((1600.0, 800.0));

    // create the initial app state
    // let initial_state = BarPlotData {
    //     name: "World".into(),
    //     iteration: Box::new(0),
    //     status: Rc::new(RefCell::new(v)),
    // };
    // start the application
    let initial_state = BarPlotData { swap: None };
    let launcher = AppLauncher::with_window(main_window);
        // .launch(initial_state)
        // .expect("Failed to launch application");

    let event_sink = launcher.get_external_handle();
    let data_clone = data.clone();
    thread::spawn(move || {
        let steps = sorter::insertion(&mut data_clone.lock().unwrap());
        steps.send_swaps(event_sink);
        println!("finished");
    });

    launcher
        .delegate(Delegate {})

        .launch(initial_state)

        .expect("launch failed");

}
