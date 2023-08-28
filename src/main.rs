mod cli;
use cli::CaptureMode;
use screenshot::*;

fn main() -> Result<()> {
    unsafe {
        RoInitialize(RO_INIT_MULTITHREADED)?;
    }

    let mode = CaptureMode::from_args();

    let item = match mode {
        CaptureMode::Window(query) => {
            let window = get_window_from_query(&query)?;
            create_capture_item_for_window(window.handle)?
        }
        CaptureMode::Monitor(id) => {
            let displays = enumerate_displays()?;
            if id == 0 {
                println!("Invalid input, ids start with 1.");
                std::process::exit(1);
            }
            let index = (id - 1) as usize;
            if index >= displays.len() {
                println!("Invalid input, id is higher than the number of displays!");
                std::process::exit(1);
            }
            let display = &displays[index];
            create_capture_item_for_monitor(display.handle)?
        }
        CaptureMode::Primary => {
            let monitor_handle =
                unsafe { MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY) };
            create_capture_item_for_monitor(monitor_handle)?
        }
    };

    take_screenshot(&item)?;

    Ok(())
}
