// Import necessary Yew functionalities
use yew::prelude::*;
use yew_router::prelude::*;
use yew::Renderer;
use stylist::yew::Global; // For applying global styles
use web_sys::console; // For logging to the browser's console
use wasm_bindgen::prelude::*;
use js_sys::JsString; // For handling JavaScript strings in Rust

// Define your app's global style, route switch, and contexts here
// These might be specific to your application's architecture.
// For example:
// use your_crate::styles::global_sty::global_style;
// use your_crate::route::{switch, Route};
// use your_crate::contexts::text_provider::TextProvider;

// Inline JavaScript function example
// This function is written in JavaScript and embedded directly in Rust.
// It gets the 'session_id' from the URL's query parameters or returns 'ID_NOT_FOUND' if not present.
#[wasm_bindgen(inline_js = "
    export function getSessionId1() {
        const params = new URLSearchParams(window.location.search);
        return params.get('session_id') || 'ID_NOT_FOUND';
    }
")]
extern "C" {
    fn getSessionId1() -> JsString;
}

// Imported JavaScript function example
// This function is defined in a separate JavaScript file, which should be located in your project's directory.
// Note: The file will be moved to the 'snippets' folder during the build process if using `cargo build`.
#[wasm_bindgen(module = "/session.js")]
extern "C" {
    fn getSessionId2() -> JsString;
}

// The main App component
#[function_component(App)]
pub fn app() -> Html {
    // Call the inline JavaScript function and log the result to the console
    let session_id_inline = getSessionId1().as_string().unwrap_or_else(|| "No Session ID".to_string());
    console::log_1(&JsValue::from_str(&format!("Session ID Inline: {}", session_id_inline)));

    // Call the imported JavaScript function and log the result to the console
    let session_id_module = getSessionId2().as_string().unwrap_or_else(|| "No Session ID".to_string());
    console::log_1(&JsValue::from_str(&format!("Session ID Module: {}", session_id_module)));

    html! {
        <>
            <BrowserRouter>
                <Global css={/* global_style() */} />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

// Main function for initializing the Yew application
fn main() {
    // Initialize the logger for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    // Render the App component
    Renderer::<App>::new().render();
}
