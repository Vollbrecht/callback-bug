sixtyfps::include_modules!();

use sixtyfps::SharedPixelBuffer;
use std::thread;
use std::time::Duration;
use plotters::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;




// 
static GLOBAL_VAR: Lazy<Mutex<i32>> = Lazy::new(||Mutex::new(0));


// call_value is not used in the function
// this function update screen
fn render_call_by_value(call_value: i32) -> sixtyfps::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(640,480);
    let size = (pixel_buffer.width() as u32, pixel_buffer.height() as u32);
    
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    let root = backend.into_drawing_area();

    root.fill(&BLACK).expect("error filling drawing area");

    // get current w aka angular frequency
    let phase = GLOBAL_VAR.lock().unwrap().clone() as f64 / 100.0;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .expect("error building coordinate system");
    chart
        .draw_series(
            LineSeries::new(
                (-314..314).map(|x,|x as f64 / 100.0).map(|x | (x, (x+phase).sin())),
                &RED                
            )
        )
        .expect("error drawing series");
    
    root.present().expect("error presenting to backend");
    drop(chart);
    drop(root);
    sixtyfps::Image::from_rgb8(pixel_buffer)
}

// this function does not update screen
fn render_call_void() -> sixtyfps::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(640,480);
    let size = (pixel_buffer.width() as u32, pixel_buffer.height() as u32);
    
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    let root = backend.into_drawing_area();

    root.fill(&BLACK).expect("error filling drawing area");

    // get current w aka angular frequency
    let phase = GLOBAL_VAR.lock().unwrap().clone() as f64 / 100.0;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .expect("error building coordinate system");
    chart
        .draw_series(
            LineSeries::new(
                (-314..314).map(|x,|x as f64 / 100.0).map(|x | (x, (x+phase).sin())),
                &RED                
            )
        )
        .expect("error drawing series");
    
    root.present().expect("error presenting to backend");
    drop(chart);
    drop(root);
    sixtyfps::Image::from_rgb8(pixel_buffer)
}


fn main() {  
 
    let ui = AppWindow::new();

    let handle = ui.as_weak();
    thread::spawn( move || {
        loop{

            let mut phase = GLOBAL_VAR.lock().unwrap().clone();
            phase += 1;
            // if phase = 2*pi -> wrap around
            if phase > (314*2+1) {
                phase = 0;
            }
            *GLOBAL_VAR.lock().unwrap() = phase; 

            let ui = handle.clone();
            sixtyfps::invoke_from_event_loop(move || {
                let ui = ui.unwrap();
                ui.set_frame_counter(ui.get_frame_counter() + 1);
                ui.set_phase_value(phase);
            });
                    
            thread::sleep(Duration::from_millis(16));
        }
    });
    
    ui.on_call_by_value(render_call_by_value);
    ui.on_call_void(render_call_void);
    ui.run();
}
