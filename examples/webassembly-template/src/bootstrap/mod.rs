use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use async_std::task::block_on;
use std::panic::PanicHookInfo;
use std::panic;
use log::{Level, Metadata, Record, LevelFilter};
use web_sys::HtmlElement;
use log::info;
use log::error;
use std::future::Future;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Error => "red",
                Level::Warn => "orange",
                Level::Info => "green",
                Level::Debug => "blue",
                Level::Trace => "gray",
            };

            let message = &format!(
                "<p><span style='color: {0}'>{1}</span> [{2}] {3}</p>",
                color,
                record.metadata().level().as_str(),
                record.metadata().target(),
                record.args()
            );

            let document = web_sys::window().unwrap().document().unwrap();
            let console_element = document.get_element_by_id("text-console").unwrap();
            let console_html_element = console_element.dyn_into::<HtmlElement>().unwrap();
            console_html_element.set_inner_html(&(console_html_element.inner_html() + message));
        }
    }

    fn flush(&self) {}
}

static LOGGER: ConsoleLogger = ConsoleLogger;

// Custom panic hook function
fn console_panic_hook(info: &PanicHookInfo) {
    log::error!("{}", info);
}


pub async fn run_with_logger<F, Fut>(level_filter: LevelFilter, function: F) where
    F: Fn() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>, {
    panic::set_hook(Box::new(console_panic_hook));
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level_filter);

    let document = web_sys::window().unwrap().document().unwrap();

    // Create a button
    let button = document.create_element("button").unwrap();
    button.set_text_content(Some("Run Example"));

    // Create a text console (div)
    let text_console = document.create_element("div").unwrap();
    text_console.set_attribute("id", "text-console").unwrap();

    document.body().unwrap().append_child(&button).unwrap();
    document.body().unwrap().append_child(&text_console).unwrap();

    let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
        let document = web_sys::window().unwrap().document().unwrap();
        let console_element = document.get_element_by_id("text-console").unwrap();
        let console_html_element = console_element.dyn_into::<HtmlElement>().unwrap();
        console_html_element.set_inner_html("");

        block_on(async {
            match function.await {
                Ok(_) => {
                    info!("Done.");
                }
                Err(x) => {
                    error!("{}" , x);
                }
            }
        });
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    let listener: &js_sys::Function = closure.as_ref().unchecked_ref();

    button.add_event_listener_with_callback("click", listener).unwrap();

    closure.forget();
}