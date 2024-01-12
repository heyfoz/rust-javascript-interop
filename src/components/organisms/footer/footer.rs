
use yew::prelude::*;
use stylist::yew::styled_component;

use crate::{components::organisms::footer::footer_sty::style, route::Route, types::text::LanguageText};

#[styled_component(Footer)]
pub fn footer() -> Html {

    html! {
        <footer class={ style() }>
            <div class="footer">
                <p>{ text.text.string_1.clone() }</p>
            </div> 
        </footer>
    }
}
