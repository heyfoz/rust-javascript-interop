use yew::prelude::*;
use web_sys::{window, console};

// Import organisms with use crate::components::organisms::organism_dir::organism_name::Organism_Styled_Component
use crate::components::organisms::header::header::Header;

#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <>
            <Header />
            <main>
                // Main organism usage
            </main>
            // Footer organism useage 
            <Footer />
        </>
    } 
}

