// Route definitions

use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => { html! { <Home /> }},
    }
}
